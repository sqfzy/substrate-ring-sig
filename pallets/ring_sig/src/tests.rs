use crate::{mock::*, Error, Event, VoteOption};
use frame::deps::sp_runtime;
use frame::testing_prelude::*;

use curve25519_dalek::constants;
use curve25519_dalek::ristretto::{CompressedRistretto, RistrettoPoint};
use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::traits::MultiscalarMul;
use nazgul::clsag::CLSAG;
use nazgul::traits::{KeyImageGen, Link, Sign, Verify};
use rand_core::{OsRng, RngCore};
use sha2::digest::consts::U64;
use sha2::{Digest, Sha512};

#[test]
fn call_verify_message() {
    let mut csprng = OsRng;
    let secret_index = 1;
    let nr = 16;
    let nc = 2;
    let ks: Vec<Scalar> = (0..nc).map(|_| Scalar::random(&mut csprng)).collect();
    let ring: Vec<Vec<RistrettoPoint>> = (0..(nr - 1))
        .map(|_| {
            (0..nc)
                .map(|_| RistrettoPoint::random(&mut csprng))
                .collect()
        })
        .collect();

    let proposal_id = 1;
    let vote = VoteOption::Yea;

    let message = {
        let mut msg = proposal_id.encode();
        msg.extend(vote.encode());
        msg
    };

    let signature = CLSAG::sign::<Sha512, OsRng>(ks.clone(), ring.clone(), secret_index, &message);
    let result = CLSAG::verify::<Sha512>(signature.clone(), &message);
    assert!(result);

    let challenge: H256 = signature.challenge.to_bytes().into();
    let responses: Vec<H256> = signature
        .responses
        .iter()
        .map(|r| r.to_bytes().into())
        .collect();
    let key_images: Vec<H256> = signature
        .key_images
        .iter()
        .map(|ki| ki.compress().to_bytes().into())
        .collect();
    let ring: Vec<Vec<H256>> = signature
        .ring
        .iter()
        .map(|row| row.iter().map(|p| p.compress().to_bytes().into()).collect())
        .collect();

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
    });
}
