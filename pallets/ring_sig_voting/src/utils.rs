use crate::VoteOption;
use frame::prelude::*;
use scale_info::prelude::vec::Vec;

#[cfg(any(test, feature = "runtime-benchmarks"))]
pub fn gen_signature<T: crate::pallet::Config>(
    proposal_id: u32,
    vote: VoteOption,
) -> (
    u32,
    VoteOption,
    H256,
    BoundedVec<H256, T::NumRingMembers>,
    BoundedVec<BoundedVec<H256, T::NumRingLayers>, T::NumRingMembers>,
    BoundedVec<H256, T::NumRingLayers>,
) {
    use curve25519_dalek::ristretto::RistrettoPoint;
    use curve25519_dalek::scalar::Scalar;
    use nazgul::clsag::CLSAG;
    use nazgul::traits::{Sign, Verify};
    use rand_core::OsRng;
    use sha2::Sha512;

    let mut csprng = OsRng;
    let secret_index = 1;
    let nr = T::NumRingMembers::get() as usize;
    let nc = T::NumRingLayers::get() as usize;

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

    let responses: BoundedVec<H256, T::NumRingMembers> = signature
        .responses
        .iter()
        .map(|r| r.to_bytes().into())
        .collect::<Vec<H256>>()
        .try_into()
        .unwrap();

    let key_images: BoundedVec<H256, T::NumRingLayers> = signature
        .key_images
        .iter()
        .map(|ki| ki.compress().to_bytes().into())
        .collect::<Vec<H256>>()
        .try_into()
        .unwrap();

    let ring: BoundedVec<BoundedVec<H256, T::NumRingLayers>, T::NumRingMembers> = signature
        .ring
        .iter()
        .map(|layer| {
            layer
                .iter()
                .map(|pk| pk.compress().to_bytes().into())
                .collect::<Vec<H256>>()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<BoundedVec<H256, T::NumRingLayers>>>()
        .try_into()
        .unwrap();

    (proposal_id, vote, challenge, responses, ring, key_images)
}
