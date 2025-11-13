use super::*;
use frame::{deps::frame_benchmarking::v2::*, prelude::*};

#[benchmarks]
mod benchmarks {
    use super::*;
    #[cfg(any(test, feature = "runtime-benchmarks"))]
    use crate::pallet::Pallet as RingSig;
    use crate::utils::gen_signature;
    use frame_system::RawOrigin;

    #[benchmark]
    fn create_proposal() {
        let caller: T::AccountId = whitelisted_caller();
        let description =
            b"Benchmark Proposal".to_vec();

        #[extrinsic_call]
        RingSig::create_proposal(RawOrigin::Signed(caller), description.clone().try_into().unwrap());

        let proposal = Proposals::<T>::get(0).unwrap();
        assert_eq!(proposal.description.into_inner(), description);
        assert_eq!(proposal.status, ProposalStatus::Voting);
    }

    #[benchmark]
    fn close_proposal() {
        let caller: T::AccountId = whitelisted_caller();
        let description =
            b"Benchmark Proposal".to_vec();

        RingSig::<T>::create_proposal(RawOrigin::Signed(caller.clone()).into(), description.clone().try_into().unwrap()).unwrap();

        #[extrinsic_call]
        RingSig::close_proposal(RawOrigin::Signed(caller), 0);

        let proposal = Proposals::<T>::get(0).unwrap();
        assert_eq!(proposal.status, ProposalStatus::Closed);
    }

    #[benchmark]
    fn anonymous_vote() {
        let caller: T::AccountId = whitelisted_caller();


        let description =
            b"Benchmark Proposal".to_vec();

        RingSig::<T>::create_proposal(RawOrigin::Signed(caller.clone()).into(), description.clone().try_into().unwrap()).unwrap();

        let proposal = Proposals::<T>::get(0).unwrap();
        assert_eq!(proposal.description.into_inner(), description);
        assert_eq!(proposal.status, ProposalStatus::Voting);

        let (proposal_id, vote, challenge, responses, ring, key_images) =
            gen_signature::<T>(0, VoteOption::Yea);

        #[extrinsic_call]
        RingSig::anonymous_vote(
            RawOrigin::Signed(caller),
            proposal_id,
            vote,
            challenge,
            responses,
            ring,
            key_images,
        );

        assert_eq!(ProposalVotes::<T>::get(proposal_id), (1, 0));
    }


    impl_benchmark_test_suite!(RingSig, crate::mock::new_test_ext(), crate::mock::Test);
}
