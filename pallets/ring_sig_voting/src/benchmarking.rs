use super::*;
use frame::{deps::frame_benchmarking::v2::*, prelude::*};

#[benchmarks]
mod benchmarks {
    use super::*;
    use crate::pallet::Pallet as RingSigVoting;
    use crate::mock::*;
    use frame_system::RawOrigin;
    use frame::deps::frame_support::traits::Currency;

    const BIG_ENOUGH: u32 = 1_000_000_000;

    #[benchmark]
    fn register_ring_group() {
        let caller: T::AccountId = whitelisted_caller();
        let ring = gen_ring::<T>();

        #[extrinsic_call]
        RingSigVoting::register_ring_group(RawOrigin::Signed(caller), ring);

        assert!(RingGroups::<T>::get(0).is_some());
    }

    #[benchmark]
    fn create_poll() {
        let caller: T::AccountId = whitelisted_caller();
        let poll_id = 0;
        let description = b"Poll 0".to_vec();
        let ring_id = 0;
        let ring = gen_ring::<T>();

        let balance = T::Currency::minimum_balance() * BIG_ENOUGH.into();
        T::Currency::make_free_balance_be(&caller, balance);

        RingSigVoting::<T>::register_ring_group(RawOrigin::Signed(caller.clone()).into(), ring).unwrap();
        assert!(RingGroups::<T>::get(ring_id).is_some());

        #[extrinsic_call]
        RingSigVoting::create_poll(
            RawOrigin::Signed(caller),
            description.clone().try_into().unwrap(),
            poll_id,
            None,
        );

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

        let balance = T::Currency::minimum_balance() * BIG_ENOUGH.into();
        T::Currency::make_free_balance_be(&caller, balance);

        RingSigVoting::<T>::register_ring_group(RawOrigin::Signed(caller.clone()).into(), ring).unwrap();
        assert!(RingGroups::<T>::get(ring_id).is_some());

        RingSigVoting::<T>::create_poll(
            RawOrigin::Signed(caller.clone()).into(),
            description.clone().try_into().unwrap(),
            0,
            None,
        ).unwrap();
        let poll = Polls::<T>::get(poll_id).unwrap();
        assert_eq!(poll.description.into_inner(), description);
        assert_eq!(poll.status, PollStatus::Voting);

        #[extrinsic_call]
        RingSigVoting::close_poll(RawOrigin::Root, poll_id);
        assert_eq!(Polls::<T>::get(poll_id).unwrap().status, PollStatus::Closed);
    }

    #[benchmark]
    fn anonymous_vote() {
        let caller: T::AccountId = whitelisted_caller();
        let vote = VoteOption::Yea;
        let poll_id = 0;
        let description = b"Poll 0".to_vec();
        let ring_id = 0;
        let (challenge, responses, ring, key_images) = gen_signature::<T>(poll_id, vote);

        let balance = T::Currency::minimum_balance() * BIG_ENOUGH.into();
        T::Currency::make_free_balance_be(&caller, balance);

        RingSigVoting::<T>::register_ring_group(RawOrigin::Signed(caller.clone()).into(), ring).unwrap();
        assert!(RingGroups::<T>::get(ring_id).is_some());

        RingSigVoting::<T>::create_poll(
            RawOrigin::Signed(caller.clone()).into(),
            description.clone().try_into().unwrap(),
            0,
            None,
        ).unwrap();
        let poll = Polls::<T>::get(poll_id).unwrap();
        assert_eq!(poll.description.into_inner(), description);
        assert_eq!(poll.status, PollStatus::Voting);

        #[extrinsic_call]
        RingSigVoting::anonymous_vote(
            RawOrigin::Signed(caller),
            poll_id,
            vote,
            challenge,
            responses,
            key_images,
        );

        assert_eq!(PollVotes::<T>::get(poll_id), (1, 0));
    }
}
