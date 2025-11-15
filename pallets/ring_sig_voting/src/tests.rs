use crate::{mock::*, *};
use frame::deps::sp_runtime;
use frame::testing_prelude::*;

#[test]
fn register_ring_group() {
    let ring = gen_ring::<Test>();

    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        assert_ok!(RingSigVoting::register_ring_group(
            RuntimeOrigin::signed(1),
            ring,
        ));
        assert!(RingGroups::<Test>::get(0).is_some());
    });
}

#[test]
fn create_poll() {
    let poll_id = 0;
    let description = b"Poll 0".to_vec();
    let ring_id = 0;
    let ring = gen_ring::<Test>();

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
            poll_id,
            None,
        ));

        let poll = Polls::<Test>::get(poll_id).unwrap();
        assert_eq!(poll.description.into_inner(), description);
        assert_eq!(poll.status, PollStatus::Voting);
    });
}

#[test]
fn close_poll() {
    let poll_id = 0;
    let description = b"Poll 0".to_vec();
    let ring_id = 0;
    let ring = gen_ring::<Test>();

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
            0,
            None,
        ));
        let poll = Polls::<Test>::get(poll_id).unwrap();
        assert_eq!(poll.description.into_inner(), description);
        assert_eq!(poll.status, PollStatus::Voting);

        assert_ok!(RingSigVoting::close_poll(RuntimeOrigin::root(), poll_id));
        assert_eq!(
            Polls::<Test>::get(poll_id).unwrap().status,
            PollStatus::Closed
        );
    });
}

#[test]
fn anonymous_vote() {
    let vote = VoteOption::Yea;
    let poll_id = 0;
    let description = b"Poll 0".to_vec();
    let ring_id = 0;
    let (challenge, responses, ring, key_images) = gen_signature::<Test>(poll_id, vote);

    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        assert_ok!(RingSigVoting::register_ring_group(
            RuntimeOrigin::signed(ALICE),
            ring.clone(),
        ));
        assert!(RingGroups::<Test>::get(ring_id).is_some());

        assert_ok!(RingSigVoting::create_poll(
            RuntimeOrigin::signed(ALICE),
            description.clone().try_into().unwrap(),
            0,
            None,
        ));
        let poll = Polls::<Test>::get(poll_id).unwrap();
        assert_eq!(poll.description.into_inner(), description);
        assert_eq!(poll.status, PollStatus::Voting);

        assert_ok!(RingSigVoting::anonymous_vote(
            RuntimeOrigin::signed(1),
            poll_id,
            vote,
            challenge,
            responses,
            key_images,
        ));

        assert_eq!(PollVotes::<Test>::get(poll_id), (1, 0));
    });
}
