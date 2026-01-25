use blake2::{Blake2b512, Digest};
use chacha20poly1305::{
    ChaCha20Poly1305, Nonce,
    aead::{Aead, KeyInit, Payload},
};
use curve25519_dalek::{
    constants::RISTRETTO_BASEPOINT_POINT,
    ristretto::{CompressedRistretto, RistrettoPoint},
    scalar::Scalar,
};
use nazgul::{
    blsag::BLSAG,
    traits::{KeyImageGen, Sign, Verify},
};
use rand_core::OsRng;
use wasm_bindgen::prelude::*;

// ========== 初始化 ==========

#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

// ========== 1. 密钥生成 ==========

/// 生成随机私钥（32字节 hex）
#[wasm_bindgen]
pub fn generate_private_key() -> Result<String, String> {
    let private_key = Scalar::random(&mut OsRng);
    Ok(hex::encode(private_key.to_bytes()))
}

/// 从私钥派生公钥
#[wasm_bindgen]
pub fn derive_public_key(private_key_hex: &str) -> Result<String, String> {
    let private_key = parse_scalar(private_key_hex)?;
    let public_key = private_key * RISTRETTO_BASEPOINT_POINT;
    Ok(hex::encode(public_key.compress().to_bytes()))
}

// ========== 2. ECDH + ChaCha20Poly1305 加密 ==========

/// 加密投票内容
/// 返回：[ephemeral_public_key_hex, ciphertext_hex]
#[wasm_bindgen]
pub fn encrypt_vote(
    poll_public_key_hex: &str,
    plaintext: &[u8],
    aad: &[u8],
) -> Result<Vec<String>, String> {
    // 1. 生成临时密钥对
    let ephemeral_private = Scalar::random(&mut OsRng);
    let ephemeral_public = ephemeral_private * RISTRETTO_BASEPOINT_POINT;

    // 2. ECDH 计算共享密钥
    let poll_public_key = parse_point(poll_public_key_hex)?;
    let shared_secret = ephemeral_private * poll_public_key;

    // 3. 直接使用压缩点的字节作为加密密钥（与后端一致）
    let encryption_key = shared_secret.compress().to_bytes();

    // 4. ChaCha20Poly1305 加密
    let cipher = ChaCha20Poly1305::new(&encryption_key.into());
    let nonce = Nonce::from_slice(&[0u8; 12]);

    let payload = Payload {
        msg: plaintext,
        aad,
    };

    let ciphertext = cipher
        .encrypt(nonce, payload)
        .map_err(|e| format!("Encryption failed: {}", e))?;

    Ok(vec![
        hex::encode(ephemeral_public.compress().to_bytes()),
        hex::encode(ciphertext),
    ])
}

/// 解密投票内容
#[wasm_bindgen]
pub fn decrypt_vote(
    ephemeral_public_key_hex: &str,
    ciphertext_hex: &str,
    poll_private_key_hex: &str,
    aad: &[u8],
) -> Result<Vec<u8>, String> {
    let ephemeral_public = parse_point(ephemeral_public_key_hex)?;
    let ciphertext =
        hex::decode(ciphertext_hex).map_err(|e| format!("Invalid ciphertext hex: {}", e))?;
    let poll_private_key = parse_scalar(poll_private_key_hex)?;

    // ECDH 计算共享密钥
    let shared_secret = poll_private_key * ephemeral_public;

    // 直接使用压缩点的字节作为解密密钥
    let encryption_key = shared_secret.compress().to_bytes();

    let cipher = ChaCha20Poly1305::new(&encryption_key.into());
    let nonce = Nonce::from_slice(&[0u8; 12]);

    let payload = Payload {
        msg: &ciphertext,
        aad,
    };

    cipher
        .decrypt(nonce, payload)
        .map_err(|e| format!("Decryption failed: {}", e))
}

// ========== 3. BLSAG 环签名 ==========

/// 生成 Key Image
#[wasm_bindgen]
pub fn generate_key_image(private_key_hex: &str) -> Result<String, String> {
    let private_key = parse_scalar(private_key_hex)?;
    let key_image = BLSAG::generate_key_image::<Blake2b512>(private_key);
    Ok(hex::encode(key_image.compress().to_bytes()))
}

/// 生成 BLSAG 环签名
/// 输入 ring_hex 为诱饵列表 (Decoys)
/// 返回格式：[Challenge, KeyImage, NewRing_0...NewRing_N, Response_0...Response_N]
#[wasm_bindgen]
pub fn sign_blsag(
    ring_hex: Vec<String>,
    message: &[u8],
    private_key_hex: &str,
    secret_index: usize,
) -> Result<Vec<String>, String> {
    // 1. 解析输入的诱饵环 (Decoys)
    let ring: Vec<RistrettoPoint> = ring_hex
        .iter()
        .map(|h| parse_point(h))
        .collect::<Result<Vec<_>, _>>()?;

    // 边界检查：secret_index 允许等于 ring.len()，这取决于 nazgul 的实现
    // 但通常插入位置不能超过当前长度
    if secret_index > ring.len() {
        return Err("secret_index out of bounds".into());
    }

    // 2. 解析私钥
    let private_key = parse_scalar(private_key_hex)?;

    // 3. 生成签名
    // nazgul 内部会自动将从 private_key 推导出的公钥插入到 ring 的 secret_index 位置
    // 因此 signature.ring 的长度会比输入的 ring 多 1
    let signature = BLSAG::sign::<Blake2b512, OsRng>(private_key, ring, secret_index, message);

    // 4. 构建返回值
    // 格式约定: [Challenge, KeyImage, Ring_0...Ring_N, Response_0...Response_N]

    let mut result = Vec::new();

    // Index 0: Challenge
    result.push(hex::encode(signature.challenge.to_bytes()));

    // Index 1: Key Image
    result.push(hex::encode(signature.key_image.compress().to_bytes()));

    // Index 2 .. 2+N: 修改后的 Ring (包含了自己的公钥)
    for point in &signature.ring {
        result.push(hex::encode(point.compress().to_bytes()));
    }

    // Index 2+N .. End: Responses
    for response in &signature.responses {
        result.push(hex::encode(response.to_bytes()));
    }

    Ok(result)
}

/// 验证 BLSAG 环签名
#[wasm_bindgen]
pub fn verify_blsag(
    ring_hex: Vec<String>,
    message: &[u8],
    key_image_hex: &str,
    challenge_hex: &str,
    responses_hex: Vec<String>,
) -> Result<bool, String> {
    // 1. 解析环
    let ring: Vec<RistrettoPoint> = ring_hex
        .iter()
        .map(|h| parse_point(h))
        .collect::<Result<Vec<_>, _>>()?;

    // 2. 解析签名
    let key_image = parse_point(key_image_hex)?;
    let challenge = parse_scalar(challenge_hex)?;
    let responses: Vec<Scalar> = responses_hex
        .iter()
        .map(|h| parse_scalar(h))
        .collect::<Result<Vec<_>, _>>()?;

    if responses.len() != ring.len() {
        return Err("responses. len() != ring.len()".into());
    }

    // 3. 构造 BLSAG 结构
    let blsag = BLSAG {
        challenge,
        responses,
        key_image,
        ring,
    };

    // 4. 验证
    Ok(BLSAG::verify::<Blake2b512>(blsag, message))
}

#[wasm_bindgen]
pub fn hash_blake2b512(data: &[u8]) -> String {
    let mut hasher = <Blake2b512 as Digest>::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

// ========== 辅助函数 ==========

fn parse_scalar(hex: &str) -> Result<Scalar, String> {
    let bytes = hex::decode(hex).map_err(|e| format!("Invalid hex: {}", e))?;

    if bytes.len() != 32 {
        return Err("Scalar must be 32 bytes".into());
    }

    let mut arr = [0u8; 32];
    arr.copy_from_slice(&bytes);

    Scalar::from_canonical_bytes(arr)
        .into_option()
        .ok_or_else(|| "Invalid scalar encoding".into())
}

fn parse_point(hex: &str) -> Result<RistrettoPoint, String> {
    let bytes = hex::decode(hex).map_err(|e| format!("Invalid hex: {}", e))?;

    if bytes.len() != 32 {
        return Err("Point must be 32 bytes".into());
    }

    let mut arr = [0u8; 32];
    arr.copy_from_slice(&bytes);

    CompressedRistretto(arr)
        .decompress()
        .ok_or_else(|| "Invalid point encoding".into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
    use curve25519_dalek::ristretto::RistrettoPoint;
    use curve25519_dalek::scalar::Scalar;
    use rand::rngs::OsRng;

    // 辅助工具
    fn point_to_hex(p: &RistrettoPoint) -> String {
        hex::encode(p.compress().as_bytes())
    }
    fn scalar_to_hex(s: &Scalar) -> String {
        hex::encode(s.as_bytes())
    }

    #[test]
    fn test_sign_and_verify_complete_flow() {
        let mut rng = OsRng;

        // --- 准备阶段 ---
        let private_key = Scalar::random(&mut rng);
        // 注意：这里我们不需要手动把公钥放进环里，sign_blsag 会帮我们做

        // 构造诱饵环 (Decoys)，假设只有 2 个诱饵
        let decoy_count = 2;
        let decoys: Vec<RistrettoPoint> = (0..decoy_count)
            .map(|_| RistrettoPoint::random(&mut rng))
            .collect();
        let decoys_hex: Vec<String> = decoys.iter().map(point_to_hex).collect();

        let message = b"Sign this message";
        // 我们想把自己的公钥插入到索引 1 的位置
        let secret_index = 1;

        // --- 签名阶段 ---
        let sign_result = sign_blsag(
            decoys_hex.clone(), // 传入诱饵
            message,
            &scalar_to_hex(&private_key),
            secret_index,
        );

        assert!(sign_result.is_ok(), "签名生成失败: {:?}", sign_result.err());
        let result_vec = sign_result.unwrap();

        // --- 解析返回数据 ---
        // 格式: [Chal, KeyImage, Ring... (N个), Resp... (N个)]
        // 总长度 L = 2 + N + N = 2 + 2N
        // 所以 N (新环大小) = (L - 2) / 2

        let total_len = result_vec.len();
        assert!((total_len - 2) % 2 == 0, "返回数据格式错误");

        let new_ring_size = (total_len - 2) / 2;

        // 验证环的大小是否增加了 (诱饵数 2 -> 新环大小 3)
        assert_eq!(
            new_ring_size,
            decoy_count + 1,
            "环的大小应该包含诱饵 + 签名者公钥"
        );

        // 切片提取数据
        let challenge_hex = &result_vec[0];
        let key_image_hex = &result_vec[1];

        // 提取新环 (偏移量 2 开始，长度 new_ring_size)
        let new_ring_hex = result_vec[2..2 + new_ring_size].to_vec();

        // 提取响应 (剩余部分)
        let responses_hex = result_vec[2 + new_ring_size..].to_vec();

        assert_eq!(
            new_ring_hex.len(),
            responses_hex.len(),
            "Ring 和 Responses 长度必须一致"
        );

        // 验证：我们可以检查生成的环里，secret_index 位置是不是我们的公钥
        // 这是一个额外的完整性检查
        let derived_pub = derive_public_key(&scalar_to_hex(&private_key)).unwrap();
        assert_eq!(
            new_ring_hex[secret_index], derived_pub,
            "新环中指定位置必须是签名者的公钥"
        );

        println!("Generated Key Image: {}", key_image_hex);
        println!("New Ring Size: {}", new_ring_hex.len());

        // --- 验证阶段 ---
        // 必须使用 `new_ring_hex` 进行验证，而不是原始的 `decoys_hex`
        let verify_result = verify_blsag(
            new_ring_hex,
            message,
            key_image_hex,
            challenge_hex,
            responses_hex,
        );

        match verify_result {
            Ok(valid) => assert!(valid, "签名验证应该返回 true"),
            Err(e) => panic!("验证过程出错: {}", e),
        }
    }
}
