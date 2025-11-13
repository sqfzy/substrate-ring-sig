use crate::{mock::*, utils::*, Error, Event, ProposalVotes, Proposals, VoteOption, ProposalStatus};
use frame::deps::sp_runtime;
use frame::testing_prelude::*;

#[test]
fn call_create_proposal() {
    let description: BoundedVec<u8, <Test as pallet::Config>::MaxDescriptionLength> = b"Proposal 0".to_vec().try_into().unwrap();

    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(RingSig::create_proposal(
            RuntimeOrigin::signed(1),
            description.clone()
        ));

        assert_eq!(
            Proposals::<Test>::get(0).unwrap().description,
            description
        );
    });
}

#[test]
fn call_close_proposal() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let description = b"Proposal 0".to_vec();
        assert_ok!(RingSig::create_proposal(
            RuntimeOrigin::signed(1),
            description.clone().try_into().unwrap()
        ));
        assert_eq!(
            Proposals::<Test>::get(0).unwrap().description.into_inner(),
            description
        );

        assert_ok!(RingSig::close_proposal(RuntimeOrigin::signed(1), 0));
        assert_eq!(Proposals::<Test>::get(0).unwrap().status, ProposalStatus::Closed);
    });
}

#[test]
fn call_verify_message() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let description = b"Proposal 0".to_vec();
        assert_ok!(RingSig::create_proposal(
            RuntimeOrigin::signed(1),
            description.clone().try_into().unwrap()
        ));
        assert_eq!(
            Proposals::<Test>::get(0).unwrap().description.into_inner(),
            description
        );

        let (proposal_id, vote, challenge, responses, ring, key_images) =
            gen_signature::<Test>(0, VoteOption::Yea);

        assert_ok!(RingSig::anonymous_vote(
            RuntimeOrigin::signed(1),
            proposal_id,
            vote,
            challenge,
            responses,
            ring,
            key_images,
        ));

        assert_eq!(ProposalVotes::<Test>::get(proposal_id), (1, 0));
    });
}
