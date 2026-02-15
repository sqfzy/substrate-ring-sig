use curve25519_dalek::{
    constants::RISTRETTO_BASEPOINT_POINT,
    scalar::Scalar,
};
use rand_core::OsRng;
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let count = 10; // 生成 10 个公钥用于测试
    let filename = "ring_keys.txt";
    let mut file = File::create(filename)?;

    println!("正在生成 {} 个 Ristretto 公钥...", count);

    for i in 0..count {
        // 生成随机私钥
        let private_key = Scalar::random(&mut OsRng);
        // 派生公钥
        let public_key = private_key * RISTRETTO_BASEPOINT_POINT;
        let public_key_hex = hex::encode(public_key.compress().to_bytes());

        // 写入文件，每行一个
        writeln!(file, "{}", public_key_hex)?;
        
        // 顺便打印对应的私钥（仅用于测试时如果你想扮演这些人）
        let priv_hex = hex::encode(private_key.to_bytes());
        println!("[{}] Pub: {} | Priv: {}", i, public_key_hex, priv_hex);
    }

    println!("完成！公钥已保存至 {}", filename);
    Ok(())
}
