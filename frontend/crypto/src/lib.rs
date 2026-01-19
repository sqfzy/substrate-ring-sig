use chacha20poly1305::{
    ChaCha20Poly1305, Nonce,
    aead::{Aead, KeyInit, Payload},
};
use curve25519_dalek::{
    constants::RISTRETTO_BASEPOINT_POINT, ristretto::RistrettoPoint, scalar::Scalar,
};
use js_sys::{Array, Uint8Array};
use nazgul::{
    blsag::BLSAG,
    traits::{KeyImageGen, Sign},
};
use rand_core::OsRng;
use wasm_bindgen::prelude::*;

// 优化 WASM 内存分配
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

// ========== 数据结构 ==========

#[wasm_bindgen]
pub struct KeyPair {
    public_key: Vec<u8>,
    private_key: Vec<u8>,
}

#[wasm_bindgen]
impl KeyPair {
    #[wasm_bindgen(getter)]
    pub fn public_key(&self) -> Uint8Array {
        Uint8Array::from(&self.public_key[..])
    }

    #[wasm_bindgen(getter)]
    pub fn private_key(&self) -> Uint8Array {
        Uint8Array::from(&self.private_key[..])
    }
}

#[wasm_bindgen]
pub struct EncryptedVote {
    ephemeral_public_key: Vec<u8>,
    ciphertext: Vec<u8>,
    challenge: Vec<u8>,
    responses: Vec<Vec<u8>>,
    key_image: Vec<u8>,
}

#[wasm_bindgen]
impl EncryptedVote {
    #[wasm_bindgen(getter)]
    pub fn ephemeral_public_key(&self) -> Uint8Array {
        Uint8Array::from(&self.ephemeral_public_key[..])
    }

    #[wasm_bindgen(getter)]
    pub fn ciphertext(&self) -> Uint8Array {
        Uint8Array::from(&self.ciphertext[..])
    }

    #[wasm_bindgen(getter)]
    pub fn challenge(&self) -> Uint8Array {
        Uint8Array::from(&self.challenge[..])
    }

    #[wasm_bindgen(getter)]
    pub fn responses(&self) -> Array {
        let arr = Array::new();
        for response in &self.responses {
            arr.push(&Uint8Array::from(&response[..]));
        }
        arr
    }

    #[wasm_bindgen(getter)]
    pub fn key_image(&self) -> Uint8Array {
        Uint8Array::from(&self.key_image[..])
    }
}

// ========== 核心 API ==========

/// 生成学生密钥对
#[wasm_bindgen]
pub fn generate_student_keypair() -> Result<KeyPair, JsValue> {
    let mut rng = OsRng;
    let private_key = Scalar::random(&mut rng);
    let public_key = private_key * &RISTRETTO_BASEPOINT_POINT;

    Ok(KeyPair {
        public_key: public_key.compress().to_bytes().to_vec(),
        private_key: private_key.to_bytes().to_vec(),
    })
}

/// 加密投票内容 (ECIES)
#[wasm_bindgen]
pub fn encrypt_vote(
    vote_content: &[u8],
    poll_public_key: &[u8],
    genesis_hash: &[u8],
    poll_id: u32,
    key_image: &[u8],
) -> Result<Vec<u8>, JsValue> {
    // 1. 生成临时密钥对
    let mut rng = OsRng;
    let ephemeral_private = Scalar::random(&mut rng);
    let ephemeral_public = ephemeral_private * &RISTRETTO_BASEPOINT_POINT;

    // 2. 计算共享密钥
    let poll_pub_point = bytes_to_point(poll_public_key)?;
    let shared_secret = ephemeral_private * poll_pub_point;

    let shared_key = shared_secret.compress().to_bytes();

    // 3. 构造 AAD (genesis_hash || poll_id || key_image)
    let mut aad = Vec::with_capacity(68);
    aad.extend_from_slice(genesis_hash);
    aad.extend_from_slice(&poll_id.to_le_bytes());
    aad.extend_from_slice(key_image);

    // 4. 加密 (ChaCha20Poly1305)
    let cipher = ChaCha20Poly1305::new(&shared_key.into());

    let nonce = Nonce::from_slice(&[0u8; 12]);
    let payload = Payload {
        msg: vote_content,
        aad: &aad,
    };
    let ciphertext = cipher
        .encrypt(nonce, payload)
        .map_err(|e| JsValue::from_str(&format!("Encryption failed: {:?}", e)))?;

    // 5. 返回:  ephemeral_public (32) || ciphertext (variable)
    let mut result = ephemeral_public.compress().to_bytes().to_vec();
    result.extend_from_slice(&ciphertext);

    Ok(result)
}

/// 生成环签名投票
///
/// # Parameters
/// - `student_private_key`: 学生私钥 (32 bytes)
/// - `ring_public_keys`: Array of public keys (每个 32 bytes)
/// - `secret_index`: 学生在环中的位置
/// - `vote_content`: 投票内容
/// - `poll_public_key`: 投票公钥 (32 bytes)
/// - `genesis_hash`: 创世区块哈希 (32 bytes)
/// - `poll_id`: 投票 ID
#[wasm_bindgen]
pub fn create_ring_signature_vote(
    student_private_key: &[u8],
    ring_public_keys: Array, // js_sys::Array
    secret_index: usize,
    vote_content: &[u8],
    poll_public_key: &[u8],
    genesis_hash: &[u8],
    poll_id: u32,
) -> Result<EncryptedVote, JsValue> {
    // 1. 解析私钥
    let secret_key = bytes_to_scalar(student_private_key)?;

    // 2. 构建 Ring (从 JS Array 转换)
    let mut ring: Vec<RistrettoPoint> = Vec::new();
    for i in 0..ring_public_keys.length() {
        let pk_array = ring_public_keys.get(i);
        let pk_bytes = Uint8Array::new(&pk_array).to_vec();
        ring.push(bytes_to_point(&pk_bytes)?);
    }

    // 3. 计算 key_image
    let key_image = BLSAG::generate_key_image::<blake2::Blake2b512>(secret_key)
        .compress()
        .to_bytes()
        .to_vec();

    // 4. 加密投票
    let encrypted = encrypt_vote(
        vote_content,
        poll_public_key,
        genesis_hash,
        poll_id,
        &key_image,
    )?;

    // 5. 构造签名消息:  ephemeral_public || ciphertext
    let message = &encrypted;

    // 6. 生成 BLSAG 签名
    let signature =
        BLSAG::sign::<blake2::Blake2b512, OsRng>(secret_key, ring.clone(), secret_index, message);

    Ok(EncryptedVote {
        ephemeral_public_key: encrypted[..32].to_vec(),
        ciphertext: encrypted[32..].to_vec(),
        challenge: signature.challenge.to_bytes().to_vec(),
        responses: signature
            .responses
            .iter()
            .map(|r| r.to_bytes().to_vec())
            .collect(),
        key_image,
    })
}

/// 解密投票 (供管理员使用)
#[wasm_bindgen]
pub fn decrypt_vote(
    encrypted_data: &[u8],
    poll_private_key: &[u8],
    genesis_hash: &[u8],
    poll_id: u32,
    key_image: &[u8],
) -> Result<Vec<u8>, JsValue> {
    if encrypted_data.len() < 32 {
        return Err(JsValue::from_str("Invalid encrypted data"));
    }

    // 1. 提取临时公钥
    let ephemeral_public = bytes_to_point(&encrypted_data[..32])?;
    let ciphertext = &encrypted_data[32..];

    // 2. 重建共享密钥
    let poll_private = bytes_to_scalar(poll_private_key)?;
    let shared_secret = poll_private * ephemeral_public;

    let shared_key = shared_secret.compress().to_bytes();

    // 3. 构造 AAD
    let mut aad = Vec::with_capacity(68);
    aad.extend_from_slice(genesis_hash);
    aad.extend_from_slice(&poll_id.to_le_bytes());
    aad.extend_from_slice(key_image);

    // 4. 解密
    let cipher = ChaCha20Poly1305::new(&shared_key.into());

    let nonce = Nonce::from_slice(&[0u8; 12]);
    let payload = Payload {
        msg: ciphertext,
        aad: &aad,
    };
    cipher
        .decrypt(nonce, payload)
        .map_err(|e| JsValue::from_str(&format!("Decryption failed: {:?}", e)))
}

// ========== 辅助函数 ==========

fn bytes_to_scalar(bytes: &[u8]) -> Result<Scalar, JsValue> {
    if bytes.len() != 32 {
        return Err(JsValue::from_str("Invalid scalar length"));
    }
    let mut arr = [0u8; 32];
    arr.copy_from_slice(bytes);
    Scalar::from_canonical_bytes(arr)
        .into_option()
        .ok_or_else(|| JsValue::from_str("Invalid scalar encoding"))
}

fn bytes_to_point(bytes: &[u8]) -> Result<RistrettoPoint, JsValue> {
    if bytes.len() != 32 {
        return Err(JsValue::from_str("Invalid point length"));
    }
    let mut arr = [0u8; 32];
    arr.copy_from_slice(bytes);
    curve25519_dalek::ristretto::CompressedRistretto(arr)
        .decompress()
        .ok_or_else(|| JsValue::from_str("Invalid compressed point"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypair_generation() {
        let keypair = generate_student_keypair().unwrap();
        assert_eq!(keypair.public_key.len(), 32);
        assert_eq!(keypair.private_key.len(), 32);
    }

    #[test]
    fn test_encryption_roundtrip() {
        let vote = b"Yes";
        let genesis_hash = [1u8; 32];
        let poll_id = 42;
        let key_image = [2u8; 32];

        // 生成密钥对
        let keypair = generate_student_keypair().unwrap();
        let poll_public_key = keypair.public_key;
        let poll_private_key = keypair.private_key;

        // 加密
        let encrypted =
            encrypt_vote(vote, &poll_public_key, &genesis_hash, poll_id, &key_image).unwrap();

        // 解密
        let decrypted = decrypt_vote(
            &encrypted,
            &poll_private_key,
            &genesis_hash,
            poll_id,
            &key_image,
        )
        .unwrap();

        assert_eq!(decrypted, vote);
    }
}

