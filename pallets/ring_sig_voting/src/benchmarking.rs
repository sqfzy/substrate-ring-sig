use super::*;
use frame::{deps::frame_benchmarking::v2::*, prelude::*};

#[benchmarks]
mod benchmarks {
    use super::*;
    use crate::mock::*;
    use crate::pallet::Pallet as RingSigVoting;
    use frame_system::RawOrigin;

    #[benchmark]
    fn register_ring_group() {
        let caller: T::AccountId = whitelisted_caller();
        let ring = gen_ring::<T>();

        #[extrinsic_call]
        RingSigVoting::register_ring_group(RawOrigin::Signed(caller), ring.clone()).unwrap();

        assert!(RingGroups::<T>::get(0).is_some());
    }

    #[benchmark]
    fn create_poll() {
        let caller: T::AccountId = whitelisted_caller();
        let poll_id = 0;
        let description = b"Poll 0".to_vec();
        let ring_id = 0;
        let ring = gen_ring::<T>();

        RingSigVoting::<T>::register_ring_group(
            RawOrigin::Signed(caller.clone()).into(),
            ring.clone(),
        )
        .unwrap();
        assert!(RingGroups::<T>::get(ring_id).is_some());

        #[extrinsic_call]
        RingSigVoting::create_poll(
            RawOrigin::Signed(caller),
            description.clone().try_into().unwrap(),
            poll_id,
            None,
        )
        .unwrap();

        let poll = Polls::<T>::get(poll_id).unwrap();
        assert_eq!(poll.description.into_inner(), description);
        assert_eq!(poll.status, PollStatus::Voting);
    }

    #[benchmark]
    fn close_poll() {
        let caller: T::AccountId = whitelisted_caller();
        let poll_id = 0;
        let description = b"Poll 0".to_vec();
        let ring_id = 0;
        let ring = gen_ring::<T>();

        new_test_ext().execute_with(|| {
            System::set_block_number(1);

            assert_ok!(RingSigVoting::register_ring_group(
                RuntimeOrigin::signed(ALICE),
                ring,
            ));
            assert!(RingGroups::<T>::get(ring_id).is_some());

            assert_ok!(RingSigVoting::create_poll(
                RuntimeOrigin::signed(ALICE),
                description.clone().try_into().unwrap(),
                0,
                None,
            ));
            let poll = Polls::<T>::get(poll_id).unwrap();
            assert_eq!(poll.description.into_inner(), description);
            assert_eq!(poll.status, PollStatus::Voting);

            #[extrinsic_call]
            RingSigVoting::close_poll(RuntimeOrigin::root(), poll_id).unwrap();
            assert_eq!(Polls::<T>::get(poll_id).unwrap().status, PollStatus::Closed);
        });
    }

    #[benchmark]
    fn anonymous_vote() {
        let caller: T::AccountId = whitelisted_caller();

        let vote = VoteOption::Yea;
        let poll_id = 0;
        let description = b"Poll 0".to_vec();
        let ring_id = 0;
        let (challenge, responses, ring, key_images) = gen_signature::<T>(poll_id, vote);

        new_test_ext().execute_with(|| {
            System::set_block_number(1);

            assert_ok!(RingSigVoting::register_ring_group(
                RuntimeOrigin::signed(ALICE),
                ring.clone(),
            ));
            assert!(RingGroups::<T>::get(ring_id).is_some());

            assert_ok!(RingSigVoting::create_poll(
                RuntimeOrigin::signed(ALICE),
                description.clone().try_into().unwrap(),
                0,
                None,
            ));
            let poll = Polls::<T>::get(poll_id).unwrap();
            assert_eq!(poll.description.into_inner(), description);
            assert_eq!(poll.status, PollStatus::Voting);

            #[extrinsic_call]
            RingSigVoting::anonymous_vote(
                RuntimeOrigin::signed(1),
                poll_id,
                vote,
                challenge,
                responses,
                key_images,
            )
            .unwrap();

            assert_eq!(PollVotes::<T>::get(poll_id), (1, 0));
        });
    }

    impl_benchmark_test_suite!(
        RingSigVoting,
        crate::mock::new_test_ext(),
        crate::mock:T:
    );
}
