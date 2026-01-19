use chacha20poly1305::{
    ChaCha20Poly1305, KeyInit, Nonce,
    aead::{Aead, Payload},
};
use curve25519_dalek::{
    constants,
    ristretto::{CompressedRistretto, RistrettoPoint},
    scalar::Scalar,
};
use nazgul::traits::{Sign, Verify};
use nazgul::{blsag::BLSAG, traits::KeyImageGen};
use rand_core::{OsRng, RngCore};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

#[derive(Serialize)]
pub struct VoteSignature {
    pub challenge: String,
    pub responses: Vec<String>,
    pub key_image: String,
    pub ephemeral_public_key: String,
    pub ciphertext: String,
}

#[derive(Serialize)]
pub struct KeyPair {
    pub secret_key: String,
    pub public_key: String,
}

// --- 辅助函数 ---

fn hex_to_scalar(hex_str: &str) -> Result<Scalar, String> {
    let bytes = hex::decode(hex_str).map_err(|e| format!("Invalid hex: {}", e))?;
    if bytes.len() != 32 {
        return Err("Scalar must be 32 bytes".into());
    }
    let mut arr = [0u8; 32];
    arr.copy_from_slice(&bytes);
    Option::from(Scalar::from_canonical_bytes(arr)).ok_or("Invalid scalar bytes".into())
}

fn hex_to_point(hex_str: &str) -> Result<RistrettoPoint, String> {
    let bytes = hex::decode(hex_str).map_err(|e| format!("Invalid hex: {}", e))?;
    if bytes.len() != 32 {
        return Err("Point must be 32 bytes".into());
    }
    let point = CompressedRistretto::from_slice(&bytes)
        .map_err(|_| "Invalid Ristretto point decompression")?
        .decompress()
        .ok_or("Invalid Ristretto point decompression")?;
    Ok(point)
}

// --- 导出函数 ---

#[wasm_bindgen]
pub fn generate_keypair() -> Result<JsValue, JsValue> {
    let mut csprng = OsRng;
    let secret_key = Scalar::random(&mut csprng);
    let public_key = secret_key * constants::RISTRETTO_BASEPOINT_POINT;

    let pair = KeyPair {
        secret_key: hex::encode(secret_key.to_bytes()),
        public_key: hex::encode(public_key.compress().to_bytes()),
    };

    Ok(serde_wasm_bindgen::to_value(&pair)?)
}

#[wasm_bindgen]
pub fn create_vote(
    ring_pubkeys_hex: Vec<String>,
    secret_key_hex: String,
    secret_index: usize,
    poll_content: &[u8],
    poll_public_key_hex: String,
    genesis_hash_hex: String, // 新增参数
    poll_id: u32,             // 新增参数
) -> Result<JsValue, JsValue> {
    // 1. 解析输入
    let secret_key = hex_to_scalar(&secret_key_hex).map_err(|e| JsValue::from_str(&e))?;
    let poll_pk = hex_to_point(&poll_public_key_hex).map_err(|e| JsValue::from_str(&e))?;
    let genesis_hash = hex::decode(genesis_hash_hex)
        .map_err(|e| JsValue::from_str(&format!("Invalid genesis hash: {}", e)))?;

    if genesis_hash.len() != 32 {
        return Err(JsValue::from_str("Genesis hash must be 32 bytes"));
    }

    let mut ring: Vec<RistrettoPoint> = Vec::new();
    for pk_hex in ring_pubkeys_hex {
        ring.push(hex_to_point(&pk_hex).map_err(|e| JsValue::from_str(&e))?);
    }

    if secret_index >= ring.len() {
        return Err(JsValue::from_str("Secret index out of bounds"));
    }

    // 2. 获取 Key Image
    let key_image = BLSAG::generate_key_image::<sha2::Sha512>(secret_key);

    // 3. 加密过程 (ECIES 实现)
    let mut csprng = OsRng;

    // A. 生成临时的 Diffie-Hellman 密钥对
    let ephemeral_secret = Scalar::random(&mut csprng);
    let ephemeral_public_key = ephemeral_secret * constants::RISTRETTO_BASEPOINT_POINT;

    // B. 计算共享秘密 Shared Secret
    let shared_point = ephemeral_secret * poll_pk;

    // C. 密钥派生 (KDF)
    let mut hasher = <Sha256 as Digest>::new();
    hasher.update(shared_point.compress().as_bytes());
    let symmetric_key = hasher.finalize(); // 32 bytes

    // D. 构造 AAD (Additional Authenticated Data)
    // aad = genesis_hash || poll_id || key_image
    let mut aad = Vec::with_capacity(32 + 4 + 32);
    aad.extend_from_slice(&genesis_hash);
    aad.extend_from_slice(&poll_id.to_le_bytes()); // Substrate 使用 Little Endian
    aad.extend_from_slice(key_image.compress().as_bytes());

    // E. 对称加密 (ChaCha20-Poly1305)
    // 设置 Nonce 为全 0。注意：标准 ChaCha20Poly1305 使用 12 字节 Nonce。
    let nonce_bytes = [0u8; 12];
    let nonce = Nonce::from_slice(&nonce_bytes);

    let cipher = ChaCha20Poly1305::new_from_slice(&symmetric_key)
        .map_err(|_| JsValue::from_str("Key initialization failed"))?;

    // 加密：传入 AAD
    let payload = Payload {
        msg: poll_content,
        aad: &aad,
    };

    let ciphertext_payload = cipher
        .encrypt(nonce, payload)
        .map_err(|_| JsValue::from_str("Encryption failed"))?;

    // 注意：由于 Nonce 是固定的全 0，我们不需要将其拼接到输出的 ciphertext 中。
    // 如果解密方不知道这一点，它将无法解密。
    let final_ciphertext = ciphertext_payload;

    // 4. 构建签名消息
    // 通常是对 (Ephemeral PubKey || Ciphertext) 进行签名
    let mut message = Vec::new();
    message.extend_from_slice(ephemeral_public_key.compress().as_bytes());
    message.extend_from_slice(&final_ciphertext);

    // 5. 生成真正的环签名 (BLSAG)
    let signature = BLSAG::sign::<sha2::Sha512, OsRng>(secret_key, ring, secret_index, &message);

    // 6. 返回结果
    let result = VoteSignature {
        challenge: hex::encode(signature.challenge.to_bytes()),
        responses: signature
            .responses
            .iter()
            .map(|s| hex::encode(s.to_bytes()))
            .collect(),
        key_image: hex::encode(signature.key_image.compress().to_bytes()), // 这里的 key_image 应该与上面计算的一致
        ephemeral_public_key: hex::encode(ephemeral_public_key.compress().to_bytes()),
        ciphertext: hex::encode(final_ciphertext),
    };

    Ok(serde_wasm_bindgen::to_value(&result)?)
}

