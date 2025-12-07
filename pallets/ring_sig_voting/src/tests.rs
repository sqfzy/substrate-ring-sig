use crate::{mock::*, types::*, Error, Event};
use curve25519_dalek::{
    ristretto::{CompressedRistretto, RistrettoPoint},
    scalar::Scalar,
};
use frame::testing_prelude::*;
use nazgul::blsag::BLSAG;
use nazgul::traits::{Sign, Verify};
use rand_core::OsRng;
use sha2::Sha512;

// Helper function to generate a test ring
fn generate_test_ring(size: usize) -> Vec<CompressedRistrettoWrapper> {
    let mut csprng = OsRng;
    (0..size)
        .map(|_| CompressedRistrettoWrapper::from(RistrettoPoint::random(&mut csprng)))
        .collect()
}

// Helper function to generate a BLSAG signature for testing
fn generate_test_signature(
    ring_size: usize,
    secret_index: usize,
    message: &[u8],
) -> (
    ScalarWrapper,
    Vec<ScalarWrapper>,
    Vec<CompressedRistrettoWrapper>,
    CompressedRistrettoWrapper,
) {
    let mut csprng = OsRng;

    // Generate secret key
    let secret_key = Scalar::random(&mut csprng);

    // Generate ring
    let mut ring: Vec<RistrettoPoint> = (0..ring_size)
        .map(|_| RistrettoPoint::random(&mut csprng))
        .collect();

    // Insert actual public key at secret index
    let public_key = secret_key * &curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
    ring[secret_index] = public_key;

    // Sign
    let signature = BLSAG::sign::<Sha512, OsRng>(vec![secret_key], ring.clone(), secret_index, message);

    // Verify it works
    assert!(BLSAG::verify::<Sha512>(signature. clone(), message));

    let challenge = ScalarWrapper::from(signature.challenge);
    let responses: Vec<ScalarWrapper> = signature.responses.into_iter().map(Into::into).collect();
    let ring_wrapped: Vec<CompressedRistrettoWrapper> =
        ring.into_iter().map(|p| p.into()).collect();
    let key_image = CompressedRistrettoWrapper::from(signature.key_image);

    (challenge, responses, ring_wrapped, key_image)
}

// Helper function to create a dummy verification key
fn create_dummy_vk() -> VkWrapper {
    use ark_bls12_381::{Bls12_381, Fr, G1Affine, G2Affine};
    use ark_groth16::VerifyingKey;
    use ark_ec::AffineRepr;
    
    let vk = VerifyingKey::<Bls12_381> {
        alpha_g1: G1Affine::generator(),
        beta_g2: G2Affine::generator(),
        gamma_g2: G2Affine::generator(),
        delta_g2: G2Affine::generator(),
        gamma_abc_g1: vec![G1Affine::generator(); 3],
    };
    
    VkWrapper::from(vk)
}

#[test]
fn register_ring_group_works() {
    new_test_ext(). execute_with(|| {
        System::set_block_number(1);

        let ring = generate_test_ring(10);

        assert_ok!(RingSigVoting::register_ring_group(
            RuntimeOrigin::root(),
            ring. clone()
        ));

        assert_eq!(RingSigVoting::ring_count(), 1);
        assert_eq!(RingSigVoting::rings(0). unwrap().len(), ring.len());

        System::assert_last_event(
            Event::RingRegistered { ring_id: 0 }. into()
        );
    });
}

#[test]
fn register_ring_group_fails_with_oversized_ring() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // Try to create a ring larger than MaxRingSize (16)
        let ring = generate_test_ring(20);

        assert_noop!(
            RingSigVoting::register_ring_group(RuntimeOrigin::root(), ring),
            Error::<Test>::RingTooLarge
        );
    });
}

#[test]
fn create_poll_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let ring = generate_test_ring(10);
        assert_ok!(RingSigVoting::register_ring_group(
            RuntimeOrigin::root(),
            ring.clone()
        ));

        let description = b"Test Poll". to_vec();
        let metadata_hash = H256::random();
        let deadline = 100u64;
        let tally_public_key = CompressedRistrettoWrapper::from(RistrettoPoint::random(&mut OsRng));
        let tally_vk = create_dummy_vk();

        assert_ok!(RingSigVoting::create_poll(
            RuntimeOrigin::root(),
            0, // ring_id
            description. clone(),
            metadata_hash,
            deadline,
            tally_public_key,
            tally_vk,
        ));

        assert_eq!(RingSigVoting::poll_count(), 1);
        let poll = RingSigVoting::polls(0).unwrap();
        assert_eq!(poll.status, PollStatus::Active);
        assert_eq!(poll.deadline, deadline);

        System::assert_last_event(
            Event::PollCreated {
                poll_id: 0,
                creator: ALICE,
            }
            .into(),
        );
    });
}

#[test]
fn create_poll_fails_with_invalid_deadline() {
    new_test_ext().execute_with(|| {
        System::set_block_number(10);

        let ring = generate_test_ring(10);
        assert_ok!(RingSigVoting::register_ring_group(
            RuntimeOrigin::root(),
            ring.clone()
        ));

        let description = b"Test Poll".to_vec();
        let metadata = b"Poll metadata".to_vec();
        let deadline = 5u64; // Past deadline
        let tally_public_key = CompressedRistrettoWrapper::from(RistrettoPoint::random(&mut OsRng));
        let tally_vk = create_dummy_vk();

        assert_noop!(
            RingSigVoting::create_poll(
                RuntimeOrigin::root(),
                0,
                description,
                metadata,
                deadline,
                tally_public_key,
                tally_vk,
            ),
            Error::<Test>::InvalidDeadline
        );
    });
}

#[test]
fn vote_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // Setup ring and poll
        let ring = generate_test_ring(10);
        assert_ok!(RingSigVoting::register_ring_group(
            RuntimeOrigin::root(),
            ring.clone()
        ));

        let description = b"Test Poll".to_vec();
        let metadata = b"Poll metadata".to_vec();
        let deadline = 100u64;
        let tally_public_key = CompressedRistrettoWrapper::from(RistrettoPoint::random(&mut OsRng));
        let tally_vk = create_dummy_vk();

        assert_ok!(RingSigVoting::create_poll(
            RuntimeOrigin::root(),
            0,
            description,
            metadata,
            deadline,
            tally_public_key. clone(),
            tally_vk,
        ));

        // Create encrypted vote
        let ephemeral_public_key = CompressedRistrettoWrapper::from(RistrettoPoint::random(&mut OsRng));
        let ciphertext = vec![1u8, 2, 3, 4, 5];

        // Create message for signature
        let encrypted_vote = EncryptedVote {
            ephemeral_public_key: ephemeral_public_key. clone(),
            ciphertext: BoundedVec::try_from(ciphertext.clone()).unwrap(),
        };
        let message = encrypted_vote.to_bytes();

        // Generate signature
        let (challenge, responses, sig_ring, key_image) =
            generate_test_signature(10, 3, &message);

        assert_ok!(RingSigVoting::vote(
            RuntimeOrigin::signed(ALICE),
            0, // poll_id
            ephemeral_public_key,
            ciphertext,
            challenge,
            responses,
            key_image,
        ));

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

        // Setup
        let ring = generate_test_ring(10);
        assert_ok!(RingSigVoting::register_ring_group(
            RuntimeOrigin::root(),
            ring.clone()
        ));

        let tally_public_key = CompressedRistrettoWrapper::from(RistrettoPoint::random(&mut OsRng));
        let tally_vk = create_dummy_vk();

        assert_ok!(RingSigVoting::create_poll(
            RuntimeOrigin::root(),
            0,
            b"Test". to_vec(),
            b"Poll metadata".to_vec(),
            100u64,
            tally_public_key,
            tally_vk,
        ));

        // Create vote
        let ephemeral_public_key = CompressedRistrettoWrapper::from(RistrettoPoint::random(&mut OsRng));
        let ciphertext = vec![1u8, 2, 3];
        let encrypted_vote = EncryptedVote {
            ephemeral_public_key: ephemeral_public_key.clone(),
            ciphertext: BoundedVec::try_from(ciphertext.clone()).unwrap(),
        };
        let message = encrypted_vote.to_bytes();

        let (challenge, responses, sig_ring, key_image) =
            generate_test_signature(10, 3, &message);

        // First vote should succeed
        assert_ok!(RingSigVoting::vote(
            RuntimeOrigin::signed(ALICE),
            0,
            ephemeral_public_key. clone(),
            ciphertext. clone(),
            challenge. clone(),
            responses.clone(),
            key_image. clone(),
        ));

        // Second vote with same key_image should fail
        assert_noop!(
            RingSigVoting::vote(
                RuntimeOrigin::signed(BOB),
                0,
                ephemeral_public_key,
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
fn tally_poll_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // Setup
        let ring = generate_test_ring(10);
        assert_ok!(RingSigVoting::register_ring_group(
            RuntimeOrigin::root(),
            ring.clone()
        ));

        let tally_vk = create_dummy_vk();
        assert_ok!(RingSigVoting::create_poll(
            RuntimeOrigin::root(),
            0,
            b"Test".to_vec(),
            b"Poll metadata".to_vec(),
            100u64,
            CompressedRistrettoWrapper::from(RistrettoPoint::random(&mut OsRng)),
            tally_vk,
        ));

        // Transition to tallying
        assert_ok!(RingSigVoting::tally_poll(RuntimeOrigin::root(), 0));

        let poll = RingSigVoting::polls(0).unwrap();
        assert_eq!(poll.status, PollStatus::Tallying);

        System::assert_last_event(Event::PollTallying { poll_id: 0 }. into());
    });
}

#[test]
fn cancel_poll_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let ring = generate_test_ring(10);
        assert_ok!(RingSigVoting::register_ring_group(
            RuntimeOrigin::root(),
            ring
        ));

        let tally_vk = create_dummy_vk();
        assert_ok!(RingSigVoting::create_poll(
            RuntimeOrigin::root(),
            0,
            b"Test".to_vec(),
            b"Poll metadata".to_vec(),
            100u64,
            CompressedRistrettoWrapper::from(RistrettoPoint::random(&mut OsRng)),
            tally_vk,
        ));

        assert_ok!(RingSigVoting::cancel_poll(RuntimeOrigin::root(), 0));

        let poll = RingSigVoting::polls(0).unwrap();
        assert_eq!(poll. status, PollStatus::Cancelled);

        System::assert_last_event(Event::PollCancelled { poll_id: 0 }.into());
    });
}

#[test]
fn pause_poll_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let ring = generate_test_ring(10);
        assert_ok!(RingSigVoting::register_ring_group(
            RuntimeOrigin::root(),
            ring
        ));

        let tally_vk = create_dummy_vk();
        assert_ok!(RingSigVoting::create_poll(
            RuntimeOrigin::root(),
            0,
            b"Test". to_vec(),
            b"Poll metadata".to_vec(),
            100u64,
            CompressedRistrettoWrapper::from(RistrettoPoint::random(&mut OsRng)),
            tally_vk,
        ));

        assert_ok!(RingSigVoting::pause_poll(RuntimeOrigin::root(), 0));
        let poll = RingSigVoting::polls(0).unwrap();
        assert_eq!(poll.status, PollStatus::Paused);

        System::assert_last_event(Event::PollPaused { poll_id: 0 }.into());
    });
}

#[test]
fn set_deadline_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let ring = generate_test_ring(10);
        assert_ok!(RingSigVoting::register_ring_group(
            RuntimeOrigin::root(),
            ring
        ));

        let tally_vk = create_dummy_vk();
        assert_ok!(RingSigVoting::create_poll(
            RuntimeOrigin::root(),
            0,
            b"Test".to_vec(),
            b"Poll metadata".to_vec(),
            100u64,
            CompressedRistrettoWrapper::from(RistrettoPoint::random(&mut OsRng)),
            tally_vk,
        ));

        let new_deadline = 200u64;
        assert_ok!(RingSigVoting::set_deadline(
            RuntimeOrigin::root(),
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

#[test]
fn vote_fails_on_invalid_signature() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let ring = generate_test_ring(10);
        assert_ok!(RingSigVoting::register_ring_group(
            RuntimeOrigin::root(),
            ring. clone()
        ));

        let tally_vk = create_dummy_vk();
        assert_ok!(RingSigVoting::create_poll(
            RuntimeOrigin::root(),
            0,
            b"Test". to_vec(),
            b"Poll metadata".to_vec(),
            100u64,
            CompressedRistrettoWrapper::from(RistrettoPoint::random(&mut OsRng)),
            tally_vk,
        ));

        let ephemeral_public_key = CompressedRistrettoWrapper::from(RistrettoPoint::random(&mut OsRng));
        let ciphertext = vec![1u8, 2, 3];

        // Create signature for different message
        let wrong_message = b"wrong message";
        let (challenge, responses, sig_ring, key_image) =
            generate_test_signature(10, 3, wrong_message);

        assert_noop!(
            RingSigVoting::vote(
                RuntimeOrigin::signed(ALICE),
                0,
                ephemeral_public_key,
                ciphertext,
                challenge,
                responses,
                key_image,
            ),
            Error::<Test>::InvalidSignature
        );
    });
}

#[test]
fn vote_fails_on_inactive_poll() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let ring = generate_test_ring(10);
        assert_ok!(RingSigVoting::register_ring_group(
            RuntimeOrigin::root(),
            ring.clone()
        ));

        let tally_vk = create_dummy_vk();
        assert_ok!(RingSigVoting::create_poll(
            RuntimeOrigin::root(),
            0,
            b"Test".to_vec(),
            b"Poll metadata".to_vec(),
            100u64,
            CompressedRistrettoWrapper::from(RistrettoPoint::random(&mut OsRng)),
            tally_vk,
        ));

        // Pause the poll
        assert_ok!(RingSigVoting::pause_poll(RuntimeOrigin::root(), 0));

        let ephemeral_public_key = CompressedRistrettoWrapper::from(RistrettoPoint::random(&mut OsRng));
        let ciphertext = vec![1u8, 2, 3];
        let encrypted_vote = EncryptedVote {
            ephemeral_public_key: ephemeral_public_key.clone(),
            ciphertext: BoundedVec::try_from(ciphertext.clone()).unwrap(),
        };
        let message = encrypted_vote.to_bytes();

        let (challenge, responses, sig_ring, key_image) =
            generate_test_signature(10, 3, &message);

        assert_noop!(
            RingSigVoting::vote(
                RuntimeOrigin::signed(ALICE),
                0,
                ephemeral_public_key,
                ciphertext,
                challenge,
                responses,
                key_image,
            ),
            Error::<Test>::InvalidPollStatus
        );
    });
}
