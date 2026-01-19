use crate::{mock::*, types::*, Error, Event};
use curve25519_dalek::{
    constants::RISTRETTO_BASEPOINT_POINT, ristretto::RistrettoPoint, scalar::Scalar,
};
use frame::testing_prelude::*;
use nazgul::{
    blsag::BLSAG,
    traits::{KeyImageGen, Sign},
};
use rand_core::OsRng;

// ========== 辅助函数 ==========

/// 生成密钥对 (用于 Poll 加密公钥)
fn generate_keypair() -> (CompressedRistrettoWrapper, ScalarWrapper) {
    let mut rng = OsRng;
    let private_key = Scalar::random(&mut rng);
    let public_key = private_key * &RISTRETTO_BASEPOINT_POINT;

    (
        CompressedRistrettoWrapper::from(public_key),
        ScalarWrapper::from(private_key),
    )
}

/// 生成测试用的 Ring
fn generate_ring(size: usize) -> Vec<CompressedRistrettoWrapper> {
    (0..size)
        .map(|_| CompressedRistrettoWrapper::from(RistrettoPoint::random(&mut OsRng)))
        .collect()
}

/// 生成完整的投票数据 (包括环签名)
fn create_vote(
    ring_size: usize,
    secret_index: usize,
    ciphertext_data: &[u8],
) -> (
    CompressedRistrettoWrapper,      // ephemeral_public_key
    Vec<u8>,                         // ciphertext
    ScalarWrapper,                   // challenge
    Vec<ScalarWrapper>,              // responses
    Vec<CompressedRistrettoWrapper>, // ring
    CompressedRistrettoWrapper,      // key_image
) {
    let mut rng = OsRng;

    // 1. 生成学生私钥和对应的 Ring
    let student_private_key = Scalar::random(&mut rng);
    let student_public_key = student_private_key * &RISTRETTO_BASEPOINT_POINT;

    let mut ring: Vec<RistrettoPoint> = (0..ring_size - 1)
        .map(|_| RistrettoPoint::random(&mut rng))
        .collect();
    ring[secret_index] = student_public_key;

    // 2. 生成临时公钥 (模拟 ECIES)
    let ephemeral_public_key = CompressedRistrettoWrapper::from(RistrettoPoint::random(&mut rng));
    let ciphertext = ciphertext_data.to_vec();

    // 3. 计算 Key Image
    let key_image = CompressedRistrettoWrapper::from(
        BLSAG::generate_key_image::<blake2::Blake2b512>(student_private_key),
    );

    // 4. 构造签名消息 (ephemeral_public_key || ciphertext)
    let encrypted_vote = EncryptedVote::<ConstU32<128>> {
        ephemeral_public_key: ephemeral_public_key.clone(),
        ciphertext: BoundedVec::try_from(ciphertext.clone()).unwrap(),
        key_image: key_image.clone(),
    };
    let message = encrypted_vote.to_bytes();

    // 5. 生成 BLSAG 签名
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
        signature
            .ring
            .into_iter()
            .map(CompressedRistrettoWrapper::from)
            .collect(),
        key_image,
    )
}

// ========== 测试用例 ==========

// ---------- Ring Registration ----------

#[test]
fn register_ring_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let ring = generate_ring(RING_SIZE);

        assert_ok!(RingSigVoting::register_ring(
            RuntimeOrigin::signed(ALICE),
            ring.clone()
        ));

        assert_eq!(RingSigVoting::ring_count(), 1);
        assert_eq!(RingSigVoting::rings(0).unwrap().len(), ring.len());

        System::assert_last_event(Event::RingRegistered { ring_id: 0 }.into());
    });
}

#[test]
fn register_ring_fails_without_admin() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let ring = generate_ring(RING_SIZE);

        assert_noop!(
            RingSigVoting::register_ring(RuntimeOrigin::signed(BOB), ring),
            BadOrigin
        );
    });
}

#[test]
fn register_ring_fails_with_oversized_ring() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let oversized = 20; // MaxRingSize = 16
        let ring = generate_ring(oversized);

        assert_noop!(
            RingSigVoting::register_ring(RuntimeOrigin::signed(ALICE), ring),
            Error::<Test>::RingTooLarge
        );
    });
}

// ---------- Teacher Authorization ----------

#[test]
fn authorize_teacher_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        assert_ok!(RingSigVoting::authorize_teacher(
            RuntimeOrigin::signed(ALICE),
            BOB
        ));

        assert!(RingSigVoting::teachers(BOB).is_some());
        System::assert_last_event(Event::TeacherAuthorized { who: BOB }.into());
    });
}

#[test]
fn revoke_teacher_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // 先授权
        assert_ok!(RingSigVoting::authorize_teacher(
            RuntimeOrigin::signed(ALICE),
            BOB
        ));

        // 再撤销
        assert_ok!(RingSigVoting::revoke_teacher(
            RuntimeOrigin::signed(ALICE),
            BOB
        ));

        assert!(RingSigVoting::teachers(BOB).is_none());
        System::assert_last_event(Event::TeacherRevoked { who: BOB }.into());
    });
}

// ---------- Poll Creation ----------

#[test]
fn create_poll_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // 1. 注册 Ring
        let ring = generate_ring(RING_SIZE);
        assert_ok!(RingSigVoting::register_ring(
            RuntimeOrigin::signed(ALICE),
            ring
        ));

        // 2. 授权 BOB 为 Teacher
        assert_ok!(RingSigVoting::authorize_teacher(
            RuntimeOrigin::signed(ALICE),
            BOB
        ));

        // 3. 生成加密公钥
        let (public_key, _) = generate_keypair();

        // 4. 创建 Poll
        assert_ok!(RingSigVoting::create_poll(
            RuntimeOrigin::signed(BOB),
            0, // ring_id
            POLL_DESCRIPTION.to_vec(),
            POLL_METADATA.to_vec(),
            DEADLINE,
            public_key,
        ));

        // 验证
        assert_eq!(RingSigVoting::poll_count(), 1);
        let poll = RingSigVoting::polls(0).unwrap();
        assert_eq!(poll.status, PollStatus::Active);
        assert_eq!(poll.deadline, DEADLINE);
        assert!(poll.tally.is_none());
        assert!(poll.poll_private_key.is_none());

        System::assert_last_event(Event::PollCreated { poll_id: 0 }.into());
    });
}

#[test]
fn create_poll_fails_without_authorization() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let ring = generate_ring(RING_SIZE);
        assert_ok!(RingSigVoting::register_ring(
            RuntimeOrigin::signed(ALICE),
            ring
        ));

        let (public_key, _) = generate_keypair();

        // BOB 未被授权
        assert_noop!(
            RingSigVoting::create_poll(
                RuntimeOrigin::signed(BOB),
                0,
                POLL_DESCRIPTION.to_vec(),
                POLL_METADATA.to_vec(),
                DEADLINE,
                public_key,
            ),
            Error::<Test>::NoPermission
        );
    });
}

#[test]
fn create_poll_fails_with_invalid_deadline() {
    new_test_ext().execute_with(|| {
        System::set_block_number(10);

        let ring = generate_ring(RING_SIZE);
        assert_ok!(RingSigVoting::register_ring(
            RuntimeOrigin::signed(ALICE),
            ring
        ));

        assert_ok!(RingSigVoting::authorize_teacher(
            RuntimeOrigin::signed(ALICE),
            BOB
        ));

        let (public_key, _) = generate_keypair();

        // 过去的时间
        assert_noop!(
            RingSigVoting::create_poll(
                RuntimeOrigin::signed(BOB),
                0,
                POLL_DESCRIPTION.to_vec(),
                POLL_METADATA.to_vec(),
                5, // deadline < current_block
                public_key,
            ),
            Error::<Test>::InvalidDeadline
        );
    });
}

#[test]
fn create_poll_fails_with_nonexistent_ring() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        assert_ok!(RingSigVoting::authorize_teacher(
            RuntimeOrigin::signed(ALICE),
            BOB
        ));

        let (public_key, _) = generate_keypair();

        assert_noop!(
            RingSigVoting::create_poll(
                RuntimeOrigin::signed(BOB),
                999, // 不存在的 ring_id
                POLL_DESCRIPTION.to_vec(),
                POLL_METADATA.to_vec(),
                DEADLINE,
                public_key,
            ),
            Error::<Test>::RingNotFound
        );
    });
}

// ---------- Voting ----------

#[test]
fn vote_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // 1. 准备环境
        let (ephemeral_pk, ciphertext, challenge, responses, ring, key_image) =
            create_vote(RING_SIZE, SECRET_INDEX, SIMPLE_CIPHERTEXT);

        assert_ok!(RingSigVoting::register_ring(
            RuntimeOrigin::signed(ALICE),
            ring
        ));

        assert_ok!(RingSigVoting::authorize_teacher(
            RuntimeOrigin::signed(ALICE),
            BOB
        ));

        let (public_key, _) = generate_keypair();
        assert_ok!(RingSigVoting::create_poll(
            RuntimeOrigin::signed(BOB),
            0,
            POLL_DESCRIPTION.to_vec(),
            POLL_METADATA.to_vec(),
            DEADLINE,
            public_key,
        ));

        // 2. 投票 (无签名交易)
        assert_ok!(RingSigVoting::vote(
            RuntimeOrigin::none(),
            0, // poll_id
            ephemeral_pk,
            ciphertext,
            challenge,
            responses,
            key_image,
        ));

        // 3. 验证
        assert_eq!(RingSigVoting::encrypted_votes(0).len(), 1);
        System::assert_last_event(
            Event::VoteSubmitted {
                poll_id: 0,
                vote_index: 0,
            }
            .into(),
        );
    });
}

#[test]
fn vote_prevents_double_voting() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let (ephemeral_pk, ciphertext, challenge, responses, ring, key_image) =
            create_vote(RING_SIZE, SECRET_INDEX, SIMPLE_CIPHERTEXT);

        assert_ok!(RingSigVoting::register_ring(
            RuntimeOrigin::signed(ALICE),
            ring
        ));

        assert_ok!(RingSigVoting::authorize_teacher(
            RuntimeOrigin::signed(ALICE),
            BOB
        ));

        let (public_key, _) = generate_keypair();
        assert_ok!(RingSigVoting::create_poll(
            RuntimeOrigin::signed(BOB),
            0,
            POLL_DESCRIPTION.to_vec(),
            POLL_METADATA.to_vec(),
            DEADLINE,
            public_key,
        ));

        // 第一次投票成功
        assert_ok!(RingSigVoting::vote(
            RuntimeOrigin::none(),
            0,
            ephemeral_pk.clone(),
            ciphertext.clone(),
            challenge.clone(),
            responses.clone(),
            key_image.clone(),
        ));

        // 第二次投票失败 (相同 key_image)
        assert_noop!(
            RingSigVoting::vote(
                RuntimeOrigin::none(),
                0,
                ephemeral_pk,
                ciphertext,
                challenge,
                responses,
                key_image,
            ),
            Error::<Test>::KeyImageAlreadyUsed
        );
    });
}

#[test]
fn vote_fails_on_invalid_signature() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let (ephemeral_pk, ciphertext, _, _, ring, key_image) =
            create_vote(RING_SIZE, SECRET_INDEX, SIMPLE_CIPHERTEXT);

        assert_ok!(RingSigVoting::register_ring(
            RuntimeOrigin::signed(ALICE),
            ring
        ));

        assert_ok!(RingSigVoting::authorize_teacher(
            RuntimeOrigin::signed(ALICE),
            BOB
        ));

        let (public_key, _) = generate_keypair();
        assert_ok!(RingSigVoting::create_poll(
            RuntimeOrigin::signed(BOB),
            0,
            POLL_DESCRIPTION.to_vec(),
            POLL_METADATA.to_vec(),
            DEADLINE,
            public_key,
        ));

        // 使用错误的签名
        let wrong_challenge = ScalarWrapper::from(Scalar::random(&mut OsRng));
        let wrong_responses = vec![ScalarWrapper::from(Scalar::random(&mut OsRng)); RING_SIZE];

        assert_noop!(
            RingSigVoting::vote(
                RuntimeOrigin::none(),
                0,
                ephemeral_pk,
                ciphertext,
                wrong_challenge,
                wrong_responses,
                key_image,
            ),
            Error::<Test>::InvalidSignature
        );
    });
}

#[test]
fn vote_fails_on_paused_poll() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let (ephemeral_pk, ciphertext, challenge, responses, ring, key_image) =
            create_vote(RING_SIZE, SECRET_INDEX, SIMPLE_CIPHERTEXT);

        assert_ok!(RingSigVoting::register_ring(
            RuntimeOrigin::signed(ALICE),
            ring
        ));

        assert_ok!(RingSigVoting::authorize_teacher(
            RuntimeOrigin::signed(ALICE),
            BOB
        ));

        let (public_key, _) = generate_keypair();
        assert_ok!(RingSigVoting::create_poll(
            RuntimeOrigin::signed(BOB),
            0,
            POLL_DESCRIPTION.to_vec(),
            POLL_METADATA.to_vec(),
            DEADLINE,
            public_key,
        ));

        // 暂停投票
        assert_ok!(RingSigVoting::pause_poll(
            RuntimeOrigin::signed(BOB),
            0,
            b"Test pause".to_vec()
        ));

        // 投票失败
        assert_noop!(
            RingSigVoting::vote(
                RuntimeOrigin::none(),
                0,
                ephemeral_pk,
                ciphertext,
                challenge,
                responses,
                key_image,
            ),
            Error::<Test>::InvalidPollStatus
        );
    });
}

// ---------- Poll Lifecycle ----------

#[test]
fn tally_poll_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let ring = generate_ring(RING_SIZE);
        assert_ok!(RingSigVoting::register_ring(
            RuntimeOrigin::signed(ALICE),
            ring
        ));

        assert_ok!(RingSigVoting::authorize_teacher(
            RuntimeOrigin::signed(ALICE),
            BOB
        ));

        let (public_key, _) = generate_keypair();
        assert_ok!(RingSigVoting::create_poll(
            RuntimeOrigin::signed(BOB),
            0,
            POLL_DESCRIPTION.to_vec(),
            POLL_METADATA.to_vec(),
            DEADLINE,
            public_key,
        ));

        assert_ok!(RingSigVoting::tally_poll(RuntimeOrigin::signed(ALICE), 0));

        let poll = RingSigVoting::polls(0).unwrap();
        assert_eq!(poll.status, PollStatus::Tallying);

        System::assert_last_event(Event::PollTallying { poll_id: 0 }.into());
    });
}

#[test]
fn tally_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // 1. 准备环境
        let ring = generate_ring(RING_SIZE);
        assert_ok!(RingSigVoting::register_ring(
            RuntimeOrigin::signed(ALICE),
            ring
        ));

        assert_ok!(RingSigVoting::authorize_teacher(
            RuntimeOrigin::signed(ALICE),
            BOB
        ));

        // 2. 生成匹配的公私钥对
        let (public_key, private_key) = generate_keypair();

        assert_ok!(RingSigVoting::create_poll(
            RuntimeOrigin::signed(BOB),
            0,
            POLL_DESCRIPTION.to_vec(),
            POLL_METADATA.to_vec(),
            DEADLINE,
            public_key,
        ));

        // 3. 进入 Tallying 状态
        assert_ok!(RingSigVoting::tally_poll(RuntimeOrigin::signed(ALICE), 0));

        // 4. 揭示结果
        let tally_result = BoundedVec::try_from(b"Result:  100".to_vec()).unwrap();
        assert_ok!(RingSigVoting::tally(
            RuntimeOrigin::signed(BOB),
            0,
            tally_result.clone(),
            private_key.clone(),
        ));

        // 5. 验证
        let poll = RingSigVoting::polls(0).unwrap();
        assert_eq!(poll.status, PollStatus::Completed);
        assert_eq!(poll.tally, Some(tally_result.clone()));
        assert_eq!(poll.poll_private_key, Some(private_key.clone()));

        System::assert_last_event(
            Event::PollCompleted {
                poll_id: 0,
                tally: tally_result,
                private_key,
            }
            .into(),
        );
    });
}

#[test]
fn tally_fails_with_wrong_private_key() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let ring = generate_ring(RING_SIZE);
        assert_ok!(RingSigVoting::register_ring(
            RuntimeOrigin::signed(ALICE),
            ring
        ));

        assert_ok!(RingSigVoting::authorize_teacher(
            RuntimeOrigin::signed(ALICE),
            BOB
        ));

        let (public_key, _) = generate_keypair();
        let (_, wrong_private_key) = generate_keypair(); // 不匹配的私钥

        assert_ok!(RingSigVoting::create_poll(
            RuntimeOrigin::signed(BOB),
            0,
            POLL_DESCRIPTION.to_vec(),
            POLL_METADATA.to_vec(),
            DEADLINE,
            public_key,
        ));

        assert_ok!(RingSigVoting::tally_poll(RuntimeOrigin::signed(ALICE), 0));

        let tally_result = BoundedVec::try_from(b"Result".to_vec()).unwrap();

        assert_noop!(
            RingSigVoting::tally(
                RuntimeOrigin::signed(BOB),
                0,
                tally_result,
                wrong_private_key,
            ),
            Error::<Test>::InvalidSignature
        );
    });
}

#[test]
fn pause_poll_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let ring = generate_ring(RING_SIZE);
        assert_ok!(RingSigVoting::register_ring(
            RuntimeOrigin::signed(ALICE),
            ring
        ));

        assert_ok!(RingSigVoting::authorize_teacher(
            RuntimeOrigin::signed(ALICE),
            BOB
        ));

        let (public_key, _) = generate_keypair();
        assert_ok!(RingSigVoting::create_poll(
            RuntimeOrigin::signed(BOB),
            0,
            POLL_DESCRIPTION.to_vec(),
            POLL_METADATA.to_vec(),
            DEADLINE,
            public_key,
        ));

        assert_ok!(RingSigVoting::pause_poll(
            RuntimeOrigin::signed(BOB),
            0,
            b"Emergency pause".to_vec()
        ));

        let poll = RingSigVoting::polls(0).unwrap();
        assert_eq!(poll.status, PollStatus::Paused);

        System::assert_last_event(
            Event::PollPaused {
                poll_id: 0,
                reason: b"Emergency pause".to_vec(),
            }
            .into(),
        );
    });
}

#[test]
fn cancel_poll_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let ring = generate_ring(RING_SIZE);
        assert_ok!(RingSigVoting::register_ring(
            RuntimeOrigin::signed(ALICE),
            ring
        ));

        assert_ok!(RingSigVoting::authorize_teacher(
            RuntimeOrigin::signed(ALICE),
            BOB
        ));

        let (public_key, _) = generate_keypair();
        assert_ok!(RingSigVoting::create_poll(
            RuntimeOrigin::signed(BOB),
            0,
            POLL_DESCRIPTION.to_vec(),
            POLL_METADATA.to_vec(),
            DEADLINE,
            public_key,
        ));

        assert_ok!(RingSigVoting::cancel_poll(
            RuntimeOrigin::signed(BOB),
            0,
            b"Cancelled by owner".to_vec()
        ));

        let poll = RingSigVoting::polls(0).unwrap();
        assert_eq!(poll.status, PollStatus::Cancelled);
    });
}

#[test]
fn active_poll_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let ring = generate_ring(RING_SIZE);
        assert_ok!(RingSigVoting::register_ring(
            RuntimeOrigin::signed(ALICE),
            ring
        ));

        assert_ok!(RingSigVoting::authorize_teacher(
            RuntimeOrigin::signed(ALICE),
            BOB
        ));

        let (public_key, _) = generate_keypair();
        assert_ok!(RingSigVoting::create_poll(
            RuntimeOrigin::signed(BOB),
            0,
            POLL_DESCRIPTION.to_vec(),
            POLL_METADATA.to_vec(),
            DEADLINE,
            public_key,
        ));

        // 先暂停
        assert_ok!(RingSigVoting::pause_poll(
            RuntimeOrigin::signed(BOB),
            0,
            b"Pause".to_vec()
        ));

        // 再激活
        assert_ok!(RingSigVoting::active_poll(RuntimeOrigin::signed(BOB), 0));

        let poll = RingSigVoting::polls(0).unwrap();
        assert_eq!(poll.status, PollStatus::Active);

        System::assert_last_event(Event::PollActive { poll_id: 0 }.into());
    });
}

#[test]
fn set_deadline_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let ring = generate_ring(RING_SIZE);
        assert_ok!(RingSigVoting::register_ring(
            RuntimeOrigin::signed(ALICE),
            ring
        ));

        assert_ok!(RingSigVoting::authorize_teacher(
            RuntimeOrigin::signed(ALICE),
            BOB
        ));

        let (public_key, _) = generate_keypair();
        assert_ok!(RingSigVoting::create_poll(
            RuntimeOrigin::signed(BOB),
            0,
            POLL_DESCRIPTION.to_vec(),
            POLL_METADATA.to_vec(),
            DEADLINE,
            public_key,
        ));

        let new_deadline = DEADLINE * 2;
        assert_ok!(RingSigVoting::set_deadline(
            RuntimeOrigin::signed(BOB),
            0,
            new_deadline
        ));

        let poll = RingSigVoting::polls(0).unwrap();
        assert_eq!(poll.deadline, new_deadline);

        System::assert_last_event(
            Event::PollDeadlineUpdated {
                poll_id: 0,
                new_deadline,
            }
            .into(),
        );
    });
}

// ---------- Ownership Transfer ----------

#[test]
fn change_owner_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let ring = generate_ring(RING_SIZE);
        assert_ok!(RingSigVoting::register_ring(
            RuntimeOrigin::signed(ALICE),
            ring
        ));

        assert_ok!(RingSigVoting::authorize_teacher(
            RuntimeOrigin::signed(ALICE),
            BOB
        ));

        let (public_key, _) = generate_keypair();
        assert_ok!(RingSigVoting::create_poll(
            RuntimeOrigin::signed(BOB),
            0,
            POLL_DESCRIPTION.to_vec(),
            POLL_METADATA.to_vec(),
            DEADLINE,
            public_key,
        ));

        // 转移所有权
        assert_ok!(RingSigVoting::change_owner(
            RuntimeOrigin::signed(ALICE),
            0,
            ALICE
        ));

        let poll = RingSigVoting::polls(0).unwrap();
        assert_eq!(poll.owner, ALICE);

        System::assert_last_event(
            Event::PollOwnershipTransferred {
                poll_id: 0,
                from: BOB,
                to: ALICE,
            }
            .into(),
        );
    });
}

// ---------- Edge Cases ----------

#[test]
fn cannot_vote_on_nonexistent_poll() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let (ephemeral_pk, ciphertext, challenge, responses, _, key_image) =
            create_vote(RING_SIZE, SECRET_INDEX, SIMPLE_CIPHERTEXT);

        assert_noop!(
            RingSigVoting::vote(
                RuntimeOrigin::none(),
                999, // 不存在的 poll_id
                ephemeral_pk,
                ciphertext,
                challenge,
                responses,
                key_image,
            ),
            Error::<Test>::PollNotFound
        );
    });
}

#[test]
fn cannot_tally_without_admin() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let ring = generate_ring(RING_SIZE);
        assert_ok!(RingSigVoting::register_ring(
            RuntimeOrigin::signed(ALICE),
            ring
        ));

        assert_ok!(RingSigVoting::authorize_teacher(
            RuntimeOrigin::signed(ALICE),
            BOB
        ));

        let (public_key, _) = generate_keypair();
        assert_ok!(RingSigVoting::create_poll(
            RuntimeOrigin::signed(BOB),
            0,
            POLL_DESCRIPTION.to_vec(),
            POLL_METADATA.to_vec(),
            DEADLINE,
            public_key,
        ));

        // BOB 不是 Admin
        assert_noop!(
            RingSigVoting::tally_poll(RuntimeOrigin::signed(BOB), 0),
            BadOrigin
        );
    });
}
