use crate::{mock::*, types::*, Error, Event};
use frame::testing_prelude::*;

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
        let oversized_ring = <<Test as crate::Config>::MaxRingSize as frame::traits::Get<u32>>::get() as usize * 2;
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

        assert_ok!(RingSigVoting::create_poll(
            RuntimeOrigin::signed(ALICE),
            0,
            POLL_DESCRIPTION.to_vec(),
            POLL_METADATA.to_vec(),
            DEADLINE,
            random_tally_key(),
            create_dummy_vk(),
        ));

        assert_eq!(RingSigVoting::poll_count(), 1);
        assert_poll_status(0, PollStatus::Active);

        let poll = RingSigVoting::polls(0).unwrap();
        assert_eq!(poll.deadline, DEADLINE);

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

        let ring = generate_test_ring(RING_SIZE);
        assert_ok!(register_ring(RuntimeOrigin::signed(ALICE), ring));

        let past_deadline = 5;

        assert_noop!(
            RingSigVoting::create_poll(
                RuntimeOrigin::signed(ALICE),
                0,
                POLL_DESCRIPTION.to_vec(),
                POLL_METADATA.to_vec(),
                past_deadline,
                random_tally_key(),
                create_dummy_vk(),
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
            create_vote_with_signature::<Test>(
                RING_SIZE,
                SECRET_INDEX,
                SIMPLE_CIPHERTEXT,
            );

        assert_ok!(register_ring(RuntimeOrigin::signed(ALICE), sig_ring));

        assert_ok!(RingSigVoting::create_poll(
            RuntimeOrigin::signed(ALICE),
            0,
            POLL_DESCRIPTION.to_vec(),
            POLL_METADATA.to_vec(),
            DEADLINE,
            random_tally_key(),
            create_dummy_vk(),
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
            create_vote_with_signature::<Test>(
                RING_SIZE,
                SECRET_INDEX,
                SIMPLE_CIPHERTEXT,
            );

        assert_ok!(register_ring(RuntimeOrigin::signed(ALICE), sig_ring));

        assert_ok!(RingSigVoting::create_poll(
            RuntimeOrigin::signed(ALICE),
            0,
            POLL_DESCRIPTION.to_vec(),
            POLL_METADATA.to_vec(),
            DEADLINE,
            random_tally_key(),
            create_dummy_vk(),
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
        let poll_id = setup_poll(RING_SIZE, DEADLINE);

        assert_ok!(RingSigVoting::tally_poll(
            RuntimeOrigin::signed(ALICE),
            poll_id
        ));

        assert_poll_status(poll_id, PollStatus::Tallying);
        System::assert_last_event(Event::PollTallying { poll_id }.into());
    });
}

#[test]
fn cancel_poll_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let poll_id = setup_poll(RING_SIZE, DEADLINE);

        assert_ok!(RingSigVoting::cancel_poll(
            RuntimeOrigin::signed(ALICE),
            poll_id
        ));

        assert_poll_status(poll_id, PollStatus::Cancelled);
        System::assert_last_event(Event::PollCancelled { poll_id }.into());
    });
}

#[test]
fn pause_poll_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let poll_id = setup_poll(RING_SIZE, DEADLINE);

        assert_ok!(RingSigVoting::pause_poll(
            RuntimeOrigin::signed(ALICE),
            poll_id
        ));

        assert_poll_status(poll_id, PollStatus::Paused);
        System::assert_last_event(Event::PollPaused { poll_id }.into());
    });
}

#[test]
fn set_deadline_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let poll_id = setup_poll(RING_SIZE, DEADLINE);
        let new_deadline = DEADLINE * 2;

        assert_ok!(RingSigVoting::set_deadline(
            RuntimeOrigin::signed(ALICE),
            poll_id,
           new_deadline  
        ));

        let poll = RingSigVoting::polls(poll_id).unwrap();
        assert_eq!(poll.deadline, new_deadline );
        System::assert_last_event(
            Event::PollDeadlineUpdated {
                poll_id,
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
        let poll_id = setup_poll(RING_SIZE, DEADLINE);

        let ephemeral_public_key = random_tally_key();
        let ciphertext = SIMPLE_CIPHERTEXT.to_vec();

        // Create signature for WRONG message
        let wrong_message = b"wrong message";
        let (challenge, responses, _sig_ring, key_image) =
            generate_test_signature(RING_SIZE, SECRET_INDEX, wrong_message);

        assert_noop!(
            submit_vote(
                RuntimeOrigin::signed(ALICE),
                poll_id,
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
        let poll_id = setup_poll(RING_SIZE, DEADLINE);

        // Pause the poll
        assert_ok!(RingSigVoting::pause_poll(
            RuntimeOrigin::signed(ALICE),
            poll_id
        ));

        let (ephemeral_public_key, ciphertext, challenge, responses, _sig_ring, key_image) =
            create_vote_with_signature::<Test>(
                RING_SIZE,
                SECRET_INDEX,
                SIMPLE_CIPHERTEXT,
            );

        assert_noop!(
            submit_vote(
                RuntimeOrigin::signed(ALICE),
                poll_id,
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
