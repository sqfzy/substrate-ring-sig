use frame::prelude::*;
use crate::VoteOption;
use scale_info::prelude::vec::Vec;

#[cfg(any(test, feature = "runtime-benchmarks"))]
pub fn gen_signature(
    proposal_id: u32,
    vote: VoteOption,
) -> (u32, VoteOption, H256, Vec<H256>, Vec<Vec<H256>>, Vec<H256>) {
    use curve25519_dalek::ristretto::RistrettoPoint;
    use curve25519_dalek::scalar::Scalar;
    use nazgul::clsag::CLSAG;
    use nazgul::traits::{Sign, Verify};
    use rand_core::OsRng;
    use sha2::Sha512;


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

    (proposal_id, vote, challenge, responses, ring, key_images)
}
