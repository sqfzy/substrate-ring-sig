use crate::{mock::*, utils::*, Error, Event, VoteOption};
use frame::deps::sp_runtime;
use frame::testing_prelude::*;

#[test]
fn call_verify_message() {
    let (proposal_id, vote, challenge, responses, ring, key_images) =
        gen_signature(42, VoteOption::Aye);

    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(RingSig::anonymous_vote(
            RuntimeOrigin::signed(1),
            proposal_id,
            vote,
            challenge,
            responses,
            ring,
            key_images,
        ));

        assert_eq!(ProposalVotes::<Test>::get(proposal_id), (0, 1));
    });
}
