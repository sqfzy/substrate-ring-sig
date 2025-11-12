use super::*;
use frame::{deps::frame_benchmarking::v2::*, prelude::*};

#[benchmarks]
mod benchmarks {
    use super::*;
    #[cfg(test)]
    use crate::pallet::Pallet as RingSig;
    use crate::utils::gen_signature;
    use frame_system::RawOrigin;

    #[benchmark]
    fn anonymous_vote() {
        let caller: T::AccountId = whitelisted_caller();

        let (proposal_id, vote, challenge, responses, ring, key_images) =
            gen_signature(42, VoteOption::Yea);

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
