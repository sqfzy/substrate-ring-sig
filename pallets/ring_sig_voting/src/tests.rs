use crate::{mock::*, types::*, Error, Event};
use curve25519_dalek::{ristretto::RistrettoPoint, scalar::Scalar};
use frame::testing_prelude::*;
use rand_core::OsRng;

// Helper to generate key pair for testing
fn generate_key_pair() -> (CompressedRistrettoWrapper, ScalarWrapper) {
    let mut rng = OsRng;
    let private_key = Scalar::random(&mut rng);
    let public_key = RistrettoPoint::mul_base(&private_key);

    (
        CompressedRistrettoWrapper::from(public_key),
        ScalarWrapper::from(private_key),
    )
}

#[test]
fn register_ring_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let ring = generate_test_ring(RING_SIZE);

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
fn register_ring_fails_with_oversized_ring() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let oversized_ring =
            <<Test as crate::Config>::MaxRingSize as frame::traits::Get<u32>>::get() as usize * 2;
        let ring = generate_test_ring(oversized_ring);

        assert_noop!(
            RingSigVoting::register_ring(RuntimeOrigin::signed(ALICE), ring),
            Error::<Test>::RingTooLarge
        );
    });
}

#[test]
fn create_poll_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let ring = generate_test_ring(RING_SIZE);
        assert_ok!(register_ring(RuntimeOrigin::signed(ALICE), ring));

        let (public_key, _) = generate_key_pair();

        assert_ok!(RingSigVoting::create_poll(
            RuntimeOrigin::signed(ALICE),
            0,
            POLL_DESCRIPTION.to_vec(),
            POLL_METADATA.to_vec(),
            DEADLINE,
            public_key,
        ));

        assert_eq!(RingSigVoting::poll_count(), 1);
        assert_poll_status(0, PollStatus::Active);

        let poll = RingSigVoting::polls(0).unwrap();
        assert_eq!(poll.deadline, DEADLINE);
        assert!(poll.poll_private_key.is_none());
        assert!(poll.tally.is_none());

        System::assert_last_event(Event::PollCreated { poll_id: 0 }.into());
    });
}

#[test]
fn create_poll_fails_with_invalid_deadline() {
    new_test_ext().execute_with(|| {
        System::set_block_number(10);

        let ring = generate_test_ring(RING_SIZE);
        assert_ok!(register_ring(RuntimeOrigin::signed(ALICE), ring));

        let past_deadline = 5;
        let (public_key, _) = generate_key_pair();

        assert_noop!(
            RingSigVoting::create_poll(
                RuntimeOrigin::signed(ALICE),
                0,
                POLL_DESCRIPTION.to_vec(),
                POLL_METADATA.to_vec(),
                past_deadline,
                public_key,
            ),
            Error::<Test>::InvalidDeadline
        );
    });
}

#[test]
fn vote_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let (ephemeral_public_key, ciphertext, challenge, responses, sig_ring, key_image) =
            create_vote_with_signature::<Test>(RING_SIZE, SECRET_INDEX, SIMPLE_CIPHERTEXT);

        assert_ok!(register_ring(RuntimeOrigin::signed(ALICE), sig_ring));

        let (public_key, _) = generate_key_pair();
        assert_ok!(RingSigVoting::create_poll(
            RuntimeOrigin::signed(ALICE),
            0,
            POLL_DESCRIPTION.to_vec(),
            POLL_METADATA.to_vec(),
            DEADLINE,
            public_key,
        ));

        assert_ok!(submit_vote(
            RuntimeOrigin::signed(ALICE),
            0,
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

        let (ephemeral_public_key, ciphertext, challenge, responses, sig_ring, key_image) =
            create_vote_with_signature::<Test>(RING_SIZE, SECRET_INDEX, SIMPLE_CIPHERTEXT);

        assert_ok!(register_ring(RuntimeOrigin::signed(ALICE), sig_ring));

        let (public_key, _) = generate_key_pair();
        assert_ok!(RingSigVoting::create_poll(
            RuntimeOrigin::signed(ALICE),
            0,
            POLL_DESCRIPTION.to_vec(),
            POLL_METADATA.to_vec(),
            DEADLINE,
            public_key,
        ));

        // First vote succeeds
        assert_ok!(submit_vote(
            RuntimeOrigin::signed(ALICE),
            0,
            ephemeral_public_key.clone(),
            ciphertext.clone(),
            challenge.clone(),
            responses.clone(),
            key_image.clone(),
        ));

        // Second vote with same key_image fails
        assert_noop!(
            submit_vote(
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
        // Note: setup_poll helper likely needs update or we do manual setup
        // Assuming setup_poll uses random keys, we'll just do manual setup to be safe
        let ring = generate_test_ring(RING_SIZE);
        assert_ok!(register_ring(RuntimeOrigin::signed(ALICE), ring));

        let (public_key, _) = generate_key_pair();
        assert_ok!(RingSigVoting::create_poll(
            RuntimeOrigin::signed(ALICE),
            0,
            POLL_DESCRIPTION.to_vec(),
            POLL_METADATA.to_vec(),
            DEADLINE,
            public_key,
        ));

        assert_ok!(RingSigVoting::tally_poll(RuntimeOrigin::signed(ALICE), 0));

        assert_poll_status(0, PollStatus::Tallying);
        System::assert_last_event(Event::PollTallying { poll_id: 0 }.into());
    });
}

#[test]
fn tally_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // 1. Setup
        let ring = generate_test_ring(RING_SIZE);
        assert_ok!(register_ring(RuntimeOrigin::signed(ALICE), ring));

        // Generate valid keypair locally
        let (public_key, private_key) = generate_key_pair();

        assert_ok!(RingSigVoting::create_poll(
            RuntimeOrigin::signed(ALICE),
            0,
            POLL_DESCRIPTION.to_vec(),
            POLL_METADATA.to_vec(),
            DEADLINE,
            public_key,
        ));

        // 2. Transition to Tallying
        assert_ok!(RingSigVoting::tally_poll(RuntimeOrigin::signed(ALICE), 0));

        // 3. Reveal Outcome with correct key
        let claimed_tally = 100;
        assert_ok!(RingSigVoting::tally(
            RuntimeOrigin::signed(ALICE),
            0,
            claimed_tally,
            private_key.clone()
        ));

        // 4. Verification
        let poll = RingSigVoting::polls(0).unwrap();
        assert_eq!(poll.status, PollStatus::Completed);
        assert_eq!(poll.tally, Some(claimed_tally));
        assert_eq!(poll.poll_private_key, Some(private_key.clone()));

        System::assert_last_event(
            Event::PollOutcomeRevealed {
                poll_id: 0,
                tally: claimed_tally,
                private_key,
            }
            .into(),
        );
    });
}

#[test]
fn tally_fails_with_wrong_key() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // 1. Setup
        let ring = generate_test_ring(RING_SIZE);
        assert_ok!(register_ring(RuntimeOrigin::signed(ALICE), ring));

        let (public_key, _) = generate_key_pair();
        let (_, wrong_private_key) = generate_key_pair(); // Different key pair

        assert_ok!(RingSigVoting::create_poll(
            RuntimeOrigin::signed(ALICE),
            0,
            POLL_DESCRIPTION.to_vec(),
            POLL_METADATA.to_vec(),
            DEADLINE,
            public_key,
        ));

        // 2. Transition to Tallying
        assert_ok!(RingSigVoting::tally_poll(RuntimeOrigin::signed(ALICE), 0));

        // 3. Reveal Outcome with wrong key -> Should Fail
        assert_noop!(
            RingSigVoting::tally(RuntimeOrigin::signed(ALICE), 0, 100, wrong_private_key),
            Error::<Test>::InvalidSignature
        );

        // Status should still be Tallying
        assert_poll_status(0, PollStatus::Tallying);
    });
}

#[test]
fn cancel_poll_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let ring = generate_test_ring(RING_SIZE);
        assert_ok!(register_ring(RuntimeOrigin::signed(ALICE), ring));
        let (public_key, _) = generate_key_pair();

        assert_ok!(RingSigVoting::create_poll(
            RuntimeOrigin::signed(ALICE),
            0,
            POLL_DESCRIPTION.to_vec(),
            POLL_METADATA.to_vec(),
            DEADLINE,
            public_key,
        ));

        assert_ok!(RingSigVoting::cancel_poll(RuntimeOrigin::signed(ALICE), 0));

        assert_poll_status(0, PollStatus::Cancelled);
        System::assert_last_event(Event::PollCancelled { poll_id: 0 }.into());
    });
}

#[test]
fn pause_poll_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let ring = generate_test_ring(RING_SIZE);
        assert_ok!(register_ring(RuntimeOrigin::signed(ALICE), ring));
        let (public_key, _) = generate_key_pair();

        assert_ok!(RingSigVoting::create_poll(
            RuntimeOrigin::signed(ALICE),
            0,
            POLL_DESCRIPTION.to_vec(),
            POLL_METADATA.to_vec(),
            DEADLINE,
            public_key,
        ));

        assert_ok!(RingSigVoting::pause_poll(RuntimeOrigin::signed(ALICE), 0));

        assert_poll_status(0, PollStatus::Paused);
        System::assert_last_event(Event::PollPaused { poll_id: 0 }.into());
    });
}

#[test]
fn set_deadline_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let ring = generate_test_ring(RING_SIZE);
        assert_ok!(register_ring(RuntimeOrigin::signed(ALICE), ring));
        let (public_key, _) = generate_key_pair();

        assert_ok!(RingSigVoting::create_poll(
            RuntimeOrigin::signed(ALICE),
            0,
            POLL_DESCRIPTION.to_vec(),
            POLL_METADATA.to_vec(),
            DEADLINE,
            public_key,
        ));

        let new_deadline = DEADLINE * 2;

        assert_ok!(RingSigVoting::set_deadline(
            RuntimeOrigin::signed(ALICE),
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

        let (ephemeral_public_key, ciphertext, _, _, sig_ring, key_image) =
            create_vote_with_signature::<Test>(RING_SIZE, SECRET_INDEX, SIMPLE_CIPHERTEXT);

        assert_ok!(register_ring(RuntimeOrigin::signed(ALICE), sig_ring));
        let (public_key, _) = generate_key_pair();

        assert_ok!(RingSigVoting::create_poll(
            RuntimeOrigin::signed(ALICE),
            0,
            POLL_DESCRIPTION.to_vec(),
            POLL_METADATA.to_vec(),
            DEADLINE,
            public_key,
        ));

        // Create signature for WRONG message
        let wrong_message = b"wrong message";
        let (challenge, responses, _, _) =
            generate_test_signature(RING_SIZE, SECRET_INDEX, wrong_message);

        assert_noop!(
            submit_vote(
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

        let ring = generate_test_ring(RING_SIZE);
        assert_ok!(register_ring(RuntimeOrigin::signed(ALICE), ring));
        let (public_key, _) = generate_key_pair();

        assert_ok!(RingSigVoting::create_poll(
            RuntimeOrigin::signed(ALICE),
            0,
            POLL_DESCRIPTION.to_vec(),
            POLL_METADATA.to_vec(),
            DEADLINE,
            public_key,
        ));

        // Pause the poll
        assert_ok!(RingSigVoting::pause_poll(RuntimeOrigin::signed(ALICE), 0));

        let (ephemeral_public_key, ciphertext, challenge, responses, _, key_image) =
            create_vote_with_signature::<Test>(RING_SIZE, SECRET_INDEX, SIMPLE_CIPHERTEXT);

        assert_noop!(
            submit_vote(
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
