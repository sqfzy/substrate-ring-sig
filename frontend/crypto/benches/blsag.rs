/// bLSAG 客户端性能基准测试
///
/// 测量目标：不同环大小（5 / 10 / 20 / 50）下，
///   - 签名（Sign）耗时
///   - 验证（Verify）耗时
///
/// 运行方式：
///   cargo bench
///   cargo bench -- --output-format bencher | tee bench_results.txt
///
/// 对比论文数据：链上 `vote` 操作（含 bLSAG 验证）耗时 3943 µs（frame-omni-bencher 测得）。
/// 本 bench 在本地原生代码下运行，反映客户端实际体验；
/// 链上数值在 Wasm 沙箱内运行，结果会有所不同（通常更慢），可与本地结果对比。
use blake2::Blake2b512;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use curve25519_dalek::{
    constants::RISTRETTO_BASEPOINT_POINT,
    ristretto::RistrettoPoint,
    scalar::Scalar,
};
use nazgul::blsag::BLSAG;
use nazgul::traits::{Sign, Verify};
use rand::rngs::OsRng;

// ── 辅助函数 ──────────────────────────────────────────────────────────────────

/// 生成一个随机的 Ristretto 私钥（Scalar）
fn random_scalar() -> Scalar {
    Scalar::random(&mut OsRng)
}

/// 由私钥派生对应公钥
fn pubkey_from_scalar(sk: &Scalar) -> RistrettoPoint {
    sk * RISTRETTO_BASEPOINT_POINT
}

/// 构造一个包含 `ring_size` 个成员的环签名环，
/// 签名者位于 `signer_index` 处。
/// 返回 (私钥, 公钥列表, 签名者索引)
fn build_ring(ring_size: usize, signer_index: usize) -> (Scalar, Vec<RistrettoPoint>, usize) {
    assert!(signer_index < ring_size, "signer_index must be < ring_size");

    let signer_sk = random_scalar();
    let signer_pk = pubkey_from_scalar(&signer_sk);

    // 构建完整公钥列表：其他成员使用随机密钥对
    let mut ring: Vec<RistrettoPoint> = (0..ring_size)
        .map(|i| {
            if i == signer_index {
                signer_pk
            } else {
                pubkey_from_scalar(&random_scalar())
            }
        })
        .collect();

    (signer_sk, ring, signer_index)
}

/// 构造一条模拟真实场景的待签名消息：
/// genesis_hash(32) || poll_id(4) || key_image(32) || ephemeral_pk(32) || ciphertext(~64)
fn build_message() -> Vec<u8> {
    let mut msg = Vec::with_capacity(164);
    // 模拟 genesis_hash（32 字节随机）
    let genesis: [u8; 32] = rand::random();
    msg.extend_from_slice(&genesis);
    // poll_id
    msg.extend_from_slice(&42u32.to_be_bytes());
    // key_image（32 字节随机）
    let ki: [u8; 32] = rand::random();
    msg.extend_from_slice(&ki);
    // ephemeral_public_key（32 字节随机）
    let epk: [u8; 32] = rand::random();
    msg.extend_from_slice(&epk);
    // ciphertext（64 字节模拟投票密文）
    let ct: [u8; 64] = rand::random();
    msg.extend_from_slice(&ct);
    msg
}

// ── Benchmark 组：Sign ────────────────────────────────────────────────────────

fn bench_blsag_sign(c: &mut Criterion) {
    let mut group = c.benchmark_group("bLSAG/sign");

    // 模拟校园场景：小班 5 人、标准班 10 人、大班 20 人、合并选课 50 人
    for ring_size in [5usize, 10, 20, 50] {
        // 签名者固定放在环中间位置，避免边界效应
        let signer_index = ring_size / 2;

        group.bench_with_input(
            BenchmarkId::new("ring_size", ring_size),
            &ring_size,
            |b, &size| {
                // 每次 bench 迭代前重新生成密钥，避免缓存效应
                b.iter_with_setup(
                    || {
                        let (sk, ring, idx) = build_ring(size, signer_index);
                        let msg = build_message();
                        (sk, ring, idx, msg)
                    },
                    |(sk, ring, idx, msg)| {
                        // black_box 防止编译器优化掉计算
                        let _sig = BLSAG::sign::<Blake2b512>(
                            black_box(sk),
                            black_box(&ring),
                            black_box(idx),
                            black_box(&msg),
                        );
                    },
                );
            },
        );
    }

    group.finish();
}

// ── Benchmark 组：Verify ──────────────────────────────────────────────────────

fn bench_blsag_verify(c: &mut Criterion) {
    let mut group = c.benchmark_group("bLSAG/verify");

    for ring_size in [5usize, 10, 20, 50] {
        let signer_index = ring_size / 2;

        group.bench_with_input(
            BenchmarkId::new("ring_size", ring_size),
            &ring_size,
            |b, &size| {
                b.iter_with_setup(
                    || {
                        // 在 setup 阶段完成签名，bench 只计量验证
                        let (sk, ring, idx) = build_ring(size, signer_index);
                        let msg = build_message();
                        let sig = BLSAG::sign::<Blake2b512>(sk, &ring, idx, &msg)
                            .expect("signing should succeed in setup");
                        (sig, msg)
                    },
                    |(sig, msg)| {
                        let valid = BLSAG::verify::<Blake2b512>(
                            black_box(sig),
                            black_box(&msg),
                        );
                        // 确保验证结果被使用，避免被优化掉
                        assert!(black_box(valid));
                    },
                );
            },
        );
    }

    group.finish();
}

// ── Benchmark 组：Sign + Verify 端到端 ───────────────────────────────────────

/// 反映单个学生从生成签名到提交（含链下验证）的完整客户端耗时
fn bench_blsag_end_to_end(c: &mut Criterion) {
    let mut group = c.benchmark_group("bLSAG/end_to_end");

    for ring_size in [5usize, 10, 20, 50] {
        let signer_index = ring_size / 2;

        group.bench_with_input(
            BenchmarkId::new("ring_size", ring_size),
            &ring_size,
            |b, &size| {
                b.iter_with_setup(
                    || {
                        let (sk, ring, idx) = build_ring(size, signer_index);
                        let msg = build_message();
                        (sk, ring, idx, msg)
                    },
                    |(sk, ring, idx, msg)| {
                        let sig = BLSAG::sign::<Blake2b512>(
                            black_box(sk),
                            black_box(&ring),
                            black_box(idx),
                            black_box(&msg),
                        )
                        .expect("signing must succeed");

                        let valid = BLSAG::verify::<Blake2b512>(
                            black_box(sig),
                            black_box(&msg),
                        );
                        assert!(black_box(valid));
                    },
                );
            },
        );
    }

    group.finish();
}

// ── 注册并运行所有 benchmark ──────────────────────────────────────────────────

criterion_group!(
    benches,
    bench_blsag_sign,
    bench_blsag_verify,
    bench_blsag_end_to_end,
);
criterion_main!(benches);
