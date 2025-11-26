use crate::{mock::*, types::simple_voting::*, *};
use frame::testing_prelude::*;
use curve25519_dalek::{constants::RISTRETTO_BASEPOINT_POINT, scalar::Scalar};
use rand_core::OsRng;

#[test]
fn register_ring_group() {
    let ring = gen_ring::<Test>();

    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        assert_ok!(RingSigVoting::register_ring_group(
            RuntimeOrigin::signed(ALICE),
            ring,
        ));
        assert!(RingGroups::<Test>::get(0).is_some());
    });
}

#[test]
fn create_poll() {
    let poll_id = 0;
    let description = b"Poll 0". to_vec();
    let ring_id = 0;
    let ring = gen_ring::<Test>();
    
    // 生成加密密钥对
    let mut csprng = OsRng;
    let private_key_scalar = Scalar::random(&mut csprng);
    let public_key_point = private_key_scalar * RISTRETTO_BASEPOINT_POINT;
    let encryption_pubkey: H256 = public_key_point.compress().to_bytes(). into();

    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        assert_ok!(RingSigVoting::register_ring_group(
            RuntimeOrigin::signed(ALICE),
            ring,
        ));
        assert!(RingGroups::<Test>::get(ring_id).is_some());

        assert_ok!(RingSigVoting::create_poll(
            RuntimeOrigin::signed(ALICE),
            description. clone(). try_into().unwrap(),
            ring_id,
            None,
            None,
            encryption_pubkey,
        ));

        let poll = Polls::<Test>::get(poll_id).unwrap();
        assert_eq!(poll.description. into_inner(), description);
        assert_eq!(poll.status, PollStatus::Voting);
        assert_eq!(poll.encryption_public_key, Some(encryption_pubkey. 0));
    });
}

#[test]
fn close_poll() {
    let poll_id = 0;
    let description = b"Poll 0".to_vec();
    let ring_id = 0;
    let ring = gen_ring::<Test>();
    
    // 生成真实的密钥对
    let mut csprng = OsRng;
    let private_key_scalar = Scalar::random(&mut csprng);
    let public_key_point = private_key_scalar * RISTRETTO_BASEPOINT_POINT;
    
    let encryption_privkey: H256 = private_key_scalar.to_bytes().into();
    let encryption_pubkey: H256 = public_key_point.compress().to_bytes().into();
    
    let tally = (5u32, 3u32); // 模拟计票结果

    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        assert_ok!(RingSigVoting::register_ring_group(
            RuntimeOrigin::signed(ALICE),
            ring,
        ));
        assert!(RingGroups::<Test>::get(ring_id).is_some());

        assert_ok!(RingSigVoting::create_poll(
            RuntimeOrigin::signed(ALICE),
            description.clone().try_into().unwrap(),
            ring_id,
            None,
            None,
            encryption_pubkey,
        ));
        let poll = Polls::<Test>::get(poll_id).unwrap();
        assert_eq!(poll.description.into_inner(), description);
        assert_eq!(poll.status, PollStatus::Voting);

        assert_ok!(RingSigVoting::close_poll(
            RuntimeOrigin::root(),
            poll_id,
            encryption_privkey,
            tally
        ));
        
        let closed_poll = Polls::<Test>::get(poll_id).unwrap();
        assert_eq!(closed_poll.status, PollStatus::Closed);
        assert_eq!(closed_poll. encryption_private_key, Some(encryption_privkey.0));
        assert_eq!(PollVotes::<Test>::get(poll_id), tally);
    });
}

#[test]
fn anonymous_vote_encrypted() {
    let poll_id = 0;
    let description = b"Poll 0".to_vec();
    let ring_id = 0;
    
    // 生成加密密钥对
    let mut csprng = OsRng;
    let private_key_scalar = Scalar::random(&mut csprng);
    let public_key_point = private_key_scalar * RISTRETTO_BASEPOINT_POINT;
    let encryption_pubkey: H256 = public_key_point.compress().to_bytes().into();
    
    // 模拟加密数据
    let ephemeral_pubkey: H256 = [2u8; 32].into();
    let ciphertext: BoundedVec<u8, <Test as crate::Config>::MaxVoteSize> = 
        vec![1, 2, 3, 4]. try_into().unwrap();
    let auth_tag: primitive_types::H128 = [0u8; 16].into();
    
    // 生成环签名（需要对加密数据签名）
    let vote = Vote::Yea;
    let (challenge, responses, ring, key_images) = gen_signature_for_encrypted::<Test>(
        poll_id,
        vote,
        ephemeral_pubkey. 0,
        &ciphertext,
        auth_tag. 0,
    );

    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        assert_ok!(RingSigVoting::register_ring_group(
            RuntimeOrigin::signed(ALICE),
            ring. clone(),
        ));
        assert!(RingGroups::<Test>::get(ring_id). is_some());

        assert_ok!(RingSigVoting::create_poll(
            RuntimeOrigin::signed(ALICE),
            description.clone().try_into().unwrap(),
            ring_id,
            None,
            None,
            encryption_pubkey,
        ));
        let poll = Polls::<Test>::get(poll_id).unwrap();
        assert_eq!(poll.status, PollStatus::Voting);

        assert_ok!(RingSigVoting::anonymous_vote(
            RuntimeOrigin::signed(ALICE),
            poll_id,
            ephemeral_pubkey,
            ciphertext,
            auth_tag,
            challenge,
            responses,
            key_images,
        ));

        let encrypted_votes = EncryptedVotes::<Test>::get(poll_id);
        assert_eq!(encrypted_votes.len(), 1);
    });
}

#[test]
fn prevent_double_voting() {
    let poll_id = 0;
    let description = b"Poll 0".to_vec();
    let ring_id = 0;
    
    let mut csprng = OsRng;
    let private_key_scalar = Scalar::random(&mut csprng);
    let public_key_point = private_key_scalar * RISTRETTO_BASEPOINT_POINT;
    let encryption_pubkey: H256 = public_key_point.compress().to_bytes(). into();
    
    let ephemeral_pubkey: H256 = [2u8; 32].into();
    let ciphertext: BoundedVec<u8, <Test as crate::Config>::MaxVoteSize> = 
        vec![1, 2, 3, 4].try_into().unwrap();
    let auth_tag: primitive_types::H128 = [0u8; 16].into();
    
    let vote = Vote::Yea;
    let (challenge, responses, ring, key_images) = gen_signature_for_encrypted::<Test>(
        poll_id,
        vote,
        ephemeral_pubkey.0,
        &ciphertext,
        auth_tag.0,
    );

    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        assert_ok!(RingSigVoting::register_ring_group(
            RuntimeOrigin::signed(ALICE),
            ring.clone(),
        ));

        assert_ok!(RingSigVoting::create_poll(
            RuntimeOrigin::signed(ALICE),
            description.clone(). try_into().unwrap(),
            ring_id,
            None,
            None,
            encryption_pubkey,
        ));

        // 第一次投票应该成功
        assert_ok!(RingSigVoting::anonymous_vote(
            RuntimeOrigin::signed(ALICE),
            poll_id,
            ephemeral_pubkey,
            ciphertext. clone(),
            auth_tag,
            challenge,
            responses. clone(),
            key_images. clone(),
        ));

        // 第二次使用相同的 key_image 投票应该失败
        assert_err!(
            RingSigVoting::anonymous_vote(
                RuntimeOrigin::signed(BOB),
                poll_id,
                ephemeral_pubkey,
                ciphertext,
                auth_tag,
                challenge,
                responses,
                key_images,
            ),
            Error::<Test>::AlreadyVoted
        );
    });
}

#[test]
fn cannot_vote_on_closed_poll() {
    let poll_id = 0;
    let description = b"Poll 0".to_vec();
    let ring_id = 0;
    let ring = gen_ring::<Test>();
    
    let mut csprng = OsRng;
    let private_key_scalar = Scalar::random(&mut csprng);
    let public_key_point = private_key_scalar * RISTRETTO_BASEPOINT_POINT;
    
    let encryption_privkey: H256 = private_key_scalar.to_bytes(). into();
    let encryption_pubkey: H256 = public_key_point.compress().to_bytes().into();
    let tally = (0u32, 0u32);
    
    let ephemeral_pubkey: H256 = [2u8; 32].into();
    let ciphertext: BoundedVec<u8, <Test as crate::Config>::MaxVoteSize> = 
        vec![1, 2, 3, 4].try_into().unwrap();
    let auth_tag: primitive_types::H128 = [0u8; 16].into();
    
    let vote = Vote::Yea;
    let (challenge, responses, _ring, key_images) = gen_signature_for_encrypted::<Test>(
        poll_id,
        vote,
        ephemeral_pubkey.0,
        &ciphertext,
        auth_tag.0,
    );

    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        assert_ok!(RingSigVoting::register_ring_group(
            RuntimeOrigin::signed(ALICE),
            ring,
        ));

        assert_ok!(RingSigVoting::create_poll(
            RuntimeOrigin::signed(ALICE),
            description.clone().try_into().unwrap(),
            ring_id,
            None,
            None,
            encryption_pubkey,
        ));

        // 关闭投票
        assert_ok!(RingSigVoting::close_poll(
            RuntimeOrigin::root(),
            poll_id,
            encryption_privkey,
            tally
        ));

        // 尝试在关闭后投票应该失败
        assert_err!(
            RingSigVoting::anonymous_vote(
                RuntimeOrigin::signed(ALICE),
                poll_id,
                ephemeral_pubkey,
                ciphertext,
                auth_tag,
                challenge,
                responses,
                key_images,
            ),
            Error::<Test>::PollNotOpen
        );
    });
}

// #[test]
// fn register_ring_group() {
//     let ring = gen_ring::<Test>();
//
//     new_test_ext().execute_with(|| {
//         System::set_block_number(1);
//
//         assert_ok!(RingSigVoting::register_ring_group(
//             RuntimeOrigin::signed(1),
//             ring,
//         ));
//         assert!(RingGroups::<Test>::get(0).is_some());
//     });
// }
//
// #[test]
// fn create_poll() {
//     let poll_id = 0;
//     let description = b"Poll 0".to_vec();
//     let ring_id = 0;
//     let ring = gen_ring::<Test>();
//
//     new_test_ext().execute_with(|| {
//         System::set_block_number(1);
//
//         assert_ok!(RingSigVoting::register_ring_group(
//             RuntimeOrigin::signed(ALICE),
//             ring,
//         ));
//         assert!(RingGroups::<Test>::get(ring_id).is_some());
//
//         assert_ok!(RingSigVoting::create_poll(
//             RuntimeOrigin::signed(ALICE),
//             description.clone().try_into().unwrap(),
//             poll_id,
//             None,
//         ));
//
//         let poll = Polls::<Test>::get(poll_id).unwrap();
//         assert_eq!(poll.description.into_inner(), description);
//         assert_eq!(poll.status, PollStatus::Voting);
//     });
// }
//
// #[test]
// fn close_poll() {
//     let poll_id = 0;
//     let description = b"Poll 0".to_vec();
//     let ring_id = 0;
//     let ring = gen_ring::<Test>();
//
//     new_test_ext().execute_with(|| {
//         System::set_block_number(1);
//
//         assert_ok!(RingSigVoting::register_ring_group(
//             RuntimeOrigin::signed(ALICE),
//             ring,
//         ));
//         assert!(RingGroups::<Test>::get(ring_id).is_some());
//
//         assert_ok!(RingSigVoting::create_poll(
//             RuntimeOrigin::signed(ALICE),
//             description.clone().try_into().unwrap(),
//             0,
//             None,
//         ));
//         let poll = Polls::<Test>::get(poll_id).unwrap();
//         assert_eq!(poll.description.into_inner(), description);
//         assert_eq!(poll.status, PollStatus::Voting);
//
//         assert_ok!(RingSigVoting::close_poll(RuntimeOrigin::root(), poll_id));
//         assert_eq!(
//             Polls::<Test>::get(poll_id).unwrap().status,
//             PollStatus::Closed
//         );
//     });
// }
//
// #[test]
// fn anonymous_vote() {
//     let vote = Vote::Yea;
//     let poll_id = 0;
//     let description = b"Poll 0".to_vec();
//     let ring_id = 0;
//     let (challenge, responses, ring, key_images) = gen_signature::<Test>(poll_id, vote.clone());
//
//     new_test_ext().execute_with(|| {
//         System::set_block_number(1);
//
//         assert_ok!(RingSigVoting::register_ring_group(
//             RuntimeOrigin::signed(ALICE),
//             ring.clone(),
//         ));
//         assert!(RingGroups::<Test>::get(ring_id).is_some());
//
//         assert_ok!(RingSigVoting::create_poll(
//             RuntimeOrigin::signed(ALICE),
//             description.clone().try_into().unwrap(),
//             0,
//             None,
//         ));
//         let poll = Polls::<Test>::get(poll_id).unwrap();
//         assert_eq!(poll.description.into_inner(), description);
//         assert_eq!(poll.status, PollStatus::Voting);
//
//         assert_ok!(RingSigVoting::anonymous_vote(
//             RuntimeOrigin::signed(1),
//             poll_id,
//             vote,
//             challenge,
//             responses,
//             key_images,
//         ));
//
//         assert_eq!(PollVotes::<Test>::get(poll_id), (1, 0));
//     });
// }
