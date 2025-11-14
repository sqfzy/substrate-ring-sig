use super::*;
use frame::{deps::frame_benchmarking::v2::*, prelude::*};

#[benchmarks]
mod benchmarks {
    use super::*;
    #[cfg(any(test, feature = "runtime-benchmarks"))]
    use crate::pallet::Pallet as RingSigVoting;
    use crate::utils::gen_signature;
    use frame_system::RawOrigin;

    #[benchmark]
    fn create_proposal() {
        let caller: T::AccountId = whitelisted_caller();
        let description = b"Benchmark Poll".to_vec();

        #[extrinsic_call]
        RingSigVoting::create_proposal(
            RawOrigin::Signed(caller),
            description.clone().try_into().unwrap(),
        );

        let proposal = Polls::<T>::get(0).unwrap();
        assert_eq!(proposal.description.into_inner(), description);
        assert_eq!(proposal.status, PollStatus::Voting);
    }

    #[benchmark]
    fn close_proposal() {
        let caller: T::AccountId = whitelisted_caller();
        let description = b"Benchmark Poll".to_vec();

        RingSigVoting::<T>::create_proposal(
            RawOrigin::Signed(caller.clone()).into(),
            description.clone().try_into().unwrap(),
        )
        .unwrap();

        #[extrinsic_call]
        RingSigVoting::close_proposal(RawOrigin::Signed(caller), 0);

        let proposal = Polls::<T>::get(0).unwrap();
        assert_eq!(proposal.status, PollStatus::Closed);
    }

    #[benchmark]
    fn anonymous_vote() {
        let caller: T::AccountId = whitelisted_caller();

        let description = b"Benchmark Poll".to_vec();

        RingSigVoting::<T>::create_proposal(
            RawOrigin::Signed(caller.clone()).into(),
            description.clone().try_into().unwrap(),
        )
        .unwrap();

        let proposal = Polls::<T>::get(0).unwrap();
        assert_eq!(proposal.description.into_inner(), description);
        assert_eq!(proposal.status, PollStatus::Voting);

        let (proposal_id, vote, challenge, responses, ring, key_images) =
            gen_signature::<T>(0, VoteOption::Yea);

        #[extrinsic_call]
        RingSigVoting::anonymous_vote(
            RawOrigin::Signed(caller),
            proposal_id,
            vote,
            challenge,
            responses,
            ring,
            key_images,
        );

        assert_eq!(PollVotes::<T>::get(proposal_id), (1, 0));
    }

    impl_benchmark_test_suite!(
        RingSigVoting,
        crate::mock::new_test_ext(),
        crate::mock::Test
    );
}
