use super::*;
use crate::{mock::*, types::*};
use curve25519_dalek::{
    constants::RISTRETTO_BASEPOINT_POINT, ristretto::RistrettoPoint, scalar::Scalar,
};
use frame::{deps::frame_benchmarking::v2::*, prelude::*};
use nazgul::{
    blsag::BLSAG,
    traits::{KeyImageGen, Sign},
};
use rand_core::OsRng;
use scale_info::prelude::vec::Vec;

// ========== 辅助函数 ==========

/// 生成测试 Ring（返回公钥列表）
fn generate_test_ring(size: usize) -> Vec<CompressedRistrettoWrapper> {
    (0..size)
        .map(|_| CompressedRistrettoWrapper::from(RistrettoPoint::random(&mut OsRng)))
        .collect()
}

/// 生成密钥对
fn generate_test_keypair() -> (CompressedRistrettoWrapper, ScalarWrapper) {
    let mut rng = OsRng;
    let private_key = Scalar::random(&mut rng);
    let public_key = private_key * &RISTRETTO_BASEPOINT_POINT;

    (
        CompressedRistrettoWrapper::from(public_key),
        ScalarWrapper::from(private_key),
    )
}

/// 由管理员注册一组公钥到 StudentKeys，返回对应的 StudentId 数组
fn register_keys_and_get_ids<T: Config>(
    admin: &T::AccountId,
    keys: &[CompressedRistrettoWrapper],
) -> Vec<u32> {
    use crate::Pallet as RingSigVoting;
    use frame_system::RawOrigin;

    let mut ids = Vec::new();
    for key in keys {
        let id = RingSigVoting::<T>::next_student_id();
        RingSigVoting::<T>::register_student_key(
            RawOrigin::Signed(admin.clone()).into(),
            key.clone(),
        )
        .unwrap();
        ids.push(id);
    }
    ids
}

/// 辅助函数：注册公钥并组建环，返回 ring_id (始终为当前 ring_count)
fn setup_ring<T: Config>(admin: &T::AccountId, keys: &[CompressedRistrettoWrapper]) {
    use crate::Pallet as RingSigVoting;
    use frame_system::RawOrigin;

    let student_ids = register_keys_and_get_ids::<T>(admin, keys);
    RingSigVoting::<T>::register_ring(RawOrigin::Signed(admin.clone()).into(), student_ids)
        .unwrap();
}

/// 生成完整的投票数据
fn create_test_vote<T: Config>(
    ring_size: usize,
    secret_index: usize,
    ciphertext_data: &[u8],
    poll_id: u32,
) -> (
    CompressedRistrettoWrapper,
    Vec<u8>,
    ScalarWrapper,
    Vec<ScalarWrapper>,
    Vec<CompressedRistrettoWrapper>,
    CompressedRistrettoWrapper,
) {
    let mut rng = OsRng;

    let student_private_key = Scalar::random(&mut rng);
    let student_public_key = student_private_key * &RISTRETTO_BASEPOINT_POINT;

    let mut ring: Vec<RistrettoPoint> = (0..ring_size - 1)
        .map(|_| RistrettoPoint::random(&mut rng))
        .collect();
    ring.insert(secret_index, student_public_key);

    let ephemeral_public_key = CompressedRistrettoWrapper::from(RistrettoPoint::random(&mut rng));
    let ciphertext = ciphertext_data.to_vec();

    let key_image = CompressedRistrettoWrapper::from(
        BLSAG::generate_key_image::<blake2::Blake2b512>(student_private_key),
    );

    let encrypted_vote = EncryptedVote::<T::MaxCiphertextLength> {
        ephemeral_public_key: ephemeral_public_key.clone(),
        ciphertext: BoundedVec::try_from(ciphertext.clone()).unwrap(),
        key_image: key_image.clone(),
    };
    let genesis_hash = frame_system::Pallet::<T>::block_hash(
        BlockNumberFor::<T>::from(0u32)
    );
    let message = encrypted_vote.construct_message(genesis_hash.as_ref(), poll_id);

    let signature = BLSAG::sign::<blake2::Blake2b512, OsRng>(
        student_private_key,
        ring.clone(),
        secret_index,
        &message,
    );

    (
        ephemeral_public_key,
        ciphertext,
        ScalarWrapper::from(signature.challenge),
        signature
            .responses
            .into_iter()
            .map(ScalarWrapper::from)
            .collect(),
        ring.into_iter()
            .map(CompressedRistrettoWrapper::from)
            .collect(),
        key_image,
    )
}

// ========== Benchmarks ==========

#[benchmarks]
mod benchmarks {
    use super::*;
    use crate::Pallet as RingSigVoting;
    use frame_system::RawOrigin;

    /// 获取管理员账户
    fn get_admin<T: Config>() -> T::AccountId {
        whitelisted_caller()
    }

    /// 获取教师账户
    fn get_teacher<T: Config>() -> T::AccountId {
        account("teacher", 0, 0)
    }

    // ---------- register_ring ----------

    #[benchmark]
    fn register_ring() {
        let caller = get_admin::<T>();
        let ring = generate_test_ring(RING_SIZE);

        // 提前注册公钥并收集 IDs
        let student_ids = register_keys_and_get_ids::<T>(&caller, &ring);

        #[extrinsic_call]
        _(RawOrigin::Signed(caller), student_ids);

        assert_eq!(RingSigVoting::<T>::ring_count(), 1);
    }

    // ---------- create_poll ----------

    #[benchmark]
    fn create_poll() {
        let admin = get_admin::<T>();
        let teacher = get_teacher::<T>();

        // 1. 注册公钥并组建 Ring
        let ring = generate_test_ring(RING_SIZE);
        setup_ring::<T>(&admin, &ring);

        // 2. 授权 Teacher
        RingSigVoting::<T>::authorize_teacher(RawOrigin::Signed(admin).into(), teacher.clone())
            .unwrap();

        // 3. 准备参数
        let description = POLL_DESCRIPTION.to_vec();
        let metadata = POLL_METADATA.to_vec();
        let deadline = DEADLINE.into();
        let (poll_public_key, _) = generate_test_keypair();

        #[extrinsic_call]
        _(
            RawOrigin::Signed(teacher),
            0,
            description,
            metadata,
            deadline,
            poll_public_key,
        );

        assert_eq!(RingSigVoting::<T>::poll_count(), 1);
    }

    // ---------- vote ----------

    #[benchmark]
    fn vote() {
        let admin = get_admin::<T>();
        let teacher = get_teacher::<T>();

        // 1. 生成投票数据（包含完整的 ring 公钥列表）
        let (ephemeral_public_key, ciphertext, challenge, responses, sig_ring, key_image) =
            create_test_vote::<T>(RING_SIZE, SECRET_INDEX, SIMPLE_CIPHERTEXT, 0);

        // 2. 注册公钥并组建 Ring
        setup_ring::<T>(&admin, &sig_ring);

        // 3. 授权 Teacher 并创建 Poll
        RingSigVoting::<T>::authorize_teacher(RawOrigin::Signed(admin).into(), teacher.clone())
            .unwrap();

        let (poll_public_key, _) = generate_test_keypair();
        RingSigVoting::<T>::create_poll(
            RawOrigin::Signed(teacher).into(),
            0,
            POLL_DESCRIPTION.to_vec(),
            POLL_METADATA.to_vec(),
            DEADLINE.into(),
            poll_public_key,
        )
        .unwrap();

        #[extrinsic_call]
        _(
            RawOrigin::None,
            0,
            ephemeral_public_key,
            ciphertext,
            challenge,
            responses,
            key_image,
        );

        assert_eq!(RingSigVoting::<T>::encrypted_votes(0).len(), 1);
    }

    // ---------- tally_poll ----------

    #[benchmark]
    fn tally_poll() {
        let admin = get_admin::<T>();
        let teacher = get_teacher::<T>();

        // 1. 注册公钥并组建 Ring
        let ring = generate_test_ring(RING_SIZE);
        setup_ring::<T>(&admin, &ring);

        RingSigVoting::<T>::authorize_teacher(
            RawOrigin::Signed(admin.clone()).into(),
            teacher.clone(),
        )
        .unwrap();

        let (poll_public_key, _) = generate_test_keypair();
        RingSigVoting::<T>::create_poll(
            RawOrigin::Signed(teacher).into(),
            0,
            POLL_DESCRIPTION.to_vec(),
            POLL_METADATA.to_vec(),
            DEADLINE.into(),
            poll_public_key,
        )
        .unwrap();

        #[extrinsic_call]
        _(RawOrigin::Signed(admin), 0);

        let poll = RingSigVoting::<T>::polls(0).unwrap();
        assert_eq!(poll.status, PollStatus::Tallying);
    }

    // ---------- cancel_poll ----------

    #[benchmark]
    fn cancel_poll() {
        let admin = get_admin::<T>();
        let teacher = get_teacher::<T>();

        let ring = generate_test_ring(RING_SIZE);
        setup_ring::<T>(&admin, &ring);

        RingSigVoting::<T>::authorize_teacher(RawOrigin::Signed(admin).into(), teacher.clone())
            .unwrap();

        let (poll_public_key, _) = generate_test_keypair();
        RingSigVoting::<T>::create_poll(
            RawOrigin::Signed(teacher.clone()).into(),
            0,
            POLL_DESCRIPTION.to_vec(),
            POLL_METADATA.to_vec(),
            DEADLINE.into(),
            poll_public_key,
        )
        .unwrap();

        let reason = b"Benchmark cancel".to_vec();

        #[extrinsic_call]
        _(RawOrigin::Signed(teacher), 0, reason);

        let poll = RingSigVoting::<T>::polls(0).unwrap();
        assert_eq!(poll.status, PollStatus::Cancelled);
    }

    // ---------- pause_poll ----------

    #[benchmark]
    fn pause_poll() {
        let admin = get_admin::<T>();
        let teacher = get_teacher::<T>();

        let ring = generate_test_ring(RING_SIZE);
        setup_ring::<T>(&admin, &ring);

        RingSigVoting::<T>::authorize_teacher(RawOrigin::Signed(admin).into(), teacher.clone())
            .unwrap();

        let (poll_public_key, _) = generate_test_keypair();
        RingSigVoting::<T>::create_poll(
            RawOrigin::Signed(teacher.clone()).into(),
            0,
            POLL_DESCRIPTION.to_vec(),
            POLL_METADATA.to_vec(),
            DEADLINE.into(),
            poll_public_key,
        )
        .unwrap();

        let reason = b"Benchmark pause".to_vec();

        #[extrinsic_call]
        _(RawOrigin::Signed(teacher), 0, reason);

        let poll = RingSigVoting::<T>::polls(0).unwrap();
        assert_eq!(poll.status, PollStatus::Paused);
    }

    // ---------- active_poll ----------

    #[benchmark]
    fn active_poll() {
        let admin = get_admin::<T>();
        let teacher = get_teacher::<T>();

        let ring = generate_test_ring(RING_SIZE);
        setup_ring::<T>(&admin, &ring);

        RingSigVoting::<T>::authorize_teacher(RawOrigin::Signed(admin).into(), teacher.clone())
            .unwrap();

        let (poll_public_key, _) = generate_test_keypair();
        RingSigVoting::<T>::create_poll(
            RawOrigin::Signed(teacher.clone()).into(),
            0,
            POLL_DESCRIPTION.to_vec(),
            POLL_METADATA.to_vec(),
            DEADLINE.into(),
            poll_public_key,
        )
        .unwrap();

        // 先暂停
        RingSigVoting::<T>::pause_poll(
            RawOrigin::Signed(teacher.clone()).into(),
            0,
            b"Pause".to_vec(),
        )
        .unwrap();

        #[extrinsic_call]
        _(RawOrigin::Signed(teacher), 0);

        let poll = RingSigVoting::<T>::polls(0).unwrap();
        assert_eq!(poll.status, PollStatus::Active);
    }

    // ---------- set_deadline ----------

    #[benchmark]
    fn set_deadline() {
        let admin = get_admin::<T>();
        let teacher = get_teacher::<T>();

        let ring = generate_test_ring(RING_SIZE);
        setup_ring::<T>(&admin, &ring);

        RingSigVoting::<T>::authorize_teacher(RawOrigin::Signed(admin).into(), teacher.clone())
            .unwrap();

        let (poll_public_key, _) = generate_test_keypair();
        RingSigVoting::<T>::create_poll(
            RawOrigin::Signed(teacher.clone()).into(),
            0,
            POLL_DESCRIPTION.to_vec(),
            POLL_METADATA.to_vec(),
            DEADLINE.into(),
            poll_public_key,
        )
        .unwrap();

        let new_deadline = (DEADLINE * 2).into();

        #[extrinsic_call]
        _(RawOrigin::Signed(teacher), 0, new_deadline);

        let poll = RingSigVoting::<T>::polls(0).unwrap();
        assert_eq!(poll.deadline, new_deadline);
    }

    // ---------- tally ----------

    #[benchmark]
    fn tally() {
        let admin = get_admin::<T>();
        let teacher = get_teacher::<T>();

        // 1. 注册公钥并组建 Ring
        let ring = generate_test_ring(RING_SIZE);
        setup_ring::<T>(&admin, &ring);

        RingSigVoting::<T>::authorize_teacher(
            RawOrigin::Signed(admin.clone()).into(),
            teacher.clone(),
        )
        .unwrap();

        // 2. 生成匹配的公私钥对
        let (poll_public_key, poll_private_key) = generate_test_keypair();

        RingSigVoting::<T>::create_poll(
            RawOrigin::Signed(teacher.clone()).into(),
            0,
            POLL_DESCRIPTION.to_vec(),
            POLL_METADATA.to_vec(),
            DEADLINE.into(),
            poll_public_key,
        )
        .unwrap();

        // 3. 进入 Tallying 状态
        RingSigVoting::<T>::tally_poll(RawOrigin::Signed(admin).into(), 0).unwrap();

        // 4. 准备计票结果
        let tally_result = BoundedVec::try_from(b"Benchmark result".to_vec()).unwrap();

        #[extrinsic_call]
        _(
            RawOrigin::Signed(teacher),
            0,
            tally_result.clone(),
            poll_private_key.clone(),
        );

        let poll = RingSigVoting::<T>::polls(0).unwrap();
        assert_eq!(poll.status, PollStatus::Completed);
        assert_eq!(poll.tally, Some(tally_result));
        assert_eq!(poll.poll_private_key, Some(poll_private_key));
    }

    // ---------- authorize_teacher ----------

    #[benchmark]
    fn authorize_teacher() {
        let admin = get_admin::<T>();
        let teacher = get_teacher::<T>();

        #[extrinsic_call]
        _(RawOrigin::Signed(admin), teacher.clone());

        assert!(RingSigVoting::<T>::teachers(teacher).is_some());
    }

    // ---------- revoke_teacher ----------

    #[benchmark]
    fn revoke_teacher() {
        let admin = get_admin::<T>();
        let teacher = get_teacher::<T>();

        // 先授权
        RingSigVoting::<T>::authorize_teacher(
            RawOrigin::Signed(admin.clone()).into(),
            teacher.clone(),
        )
        .unwrap();

        #[extrinsic_call]
        _(RawOrigin::Signed(admin), teacher.clone());

        assert!(RingSigVoting::<T>::teachers(teacher).is_none());
    }

    // ---------- change_owner ----------

    #[benchmark]
    fn change_owner() {
        let admin = get_admin::<T>();
        let teacher = get_teacher::<T>();
        let new_owner: T::AccountId = account("new_owner", 0, 0);

        // 1. 注册公钥并组建 Ring
        let ring = generate_test_ring(RING_SIZE);
        setup_ring::<T>(&admin, &ring);

        RingSigVoting::<T>::authorize_teacher(
            RawOrigin::Signed(admin.clone()).into(),
            teacher.clone(),
        )
        .unwrap();

        let (poll_public_key, _) = generate_test_keypair();
        RingSigVoting::<T>::create_poll(
            RawOrigin::Signed(teacher).into(),
            0,
            POLL_DESCRIPTION.to_vec(),
            POLL_METADATA.to_vec(),
            DEADLINE.into(),
            poll_public_key,
        )
        .unwrap();

        #[extrinsic_call]
        _(RawOrigin::Signed(admin), 0, new_owner.clone());

        let poll = RingSigVoting::<T>::polls(0).unwrap();
        assert_eq!(poll.owner, new_owner);
    }

    // 注册测试套件
    impl_benchmark_test_suite!(
        RingSigVoting,
        crate::mock::new_test_ext(),
        crate::mock::tests::Test
    );
}
