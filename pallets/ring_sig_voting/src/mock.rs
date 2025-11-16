use crate::{types::simple_voting::*, PollId};
use frame::prelude::*;
use scale_info::prelude::vec::Vec;

use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar;
use nazgul::clsag::CLSAG;
use nazgul::traits::{Sign, Verify};
use rand_core::OsRng;
use sha2::Sha512;

#[cfg(test)]
pub use tests::*;

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate as ring_sig_voting;
    use crate::RingId;
    use frame::{runtime::prelude::*, testing_prelude::*};
    use polkadot_sdk::{pallet_balances, pallet_preimage};

    pub const ALICE: u64 = 1;
    pub const BOB: u64 = 2;
    pub const INITIAL_BALANCE: u64 = 1_000_000_000_000_000;

    type Block = frame_system::mocking::MockBlock<Test>;

    // Configure a mock runtime to test the pallet.
    #[frame_construct_runtime]
    mod runtime {
        #[runtime::runtime]
        #[runtime::derive(
            RuntimeCall,
            RuntimeEvent,
            RuntimeError,
            RuntimeOrigin,
            RuntimeFreezeReason,
            RuntimeHoldReason,
            RuntimeSlashReason,
            RuntimeLockId,
            RuntimeTask
        )]
        pub struct Test;

        #[runtime::pallet_index(0)]
        pub type System = frame_system;

        #[runtime::pallet_index(1)]
        pub type Balances = pallet_balances;

        #[runtime::pallet_index(2)]
        pub type Preimage = pallet_preimage;

        #[runtime::pallet_index(3)]
        pub type RingSigVoting = ring_sig_voting;
    }

    // System pallet configuration
    #[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
    impl frame_system::Config for Test {
        type Block = Block;
        type AccountData = pallet_balances::AccountData<u64>;
    }

    #[derive_impl(pallet_balances::config_preludes::TestDefaultConfig)]
    impl pallet_balances::Config for Test {
        type AccountStore = System;
    }

    impl pallet_preimage::Config for Test {
        type RuntimeEvent = RuntimeEvent;
        type WeightInfo = ();
        type Currency = Balances;
        type ManagerOrigin = EnsureRoot<u64>;
        type Consideration = ();
    }

    impl ring_sig_voting::Config for Test {
        type RuntimeEvent = RuntimeEvent;
        type Currency = Balances;
        type Preimages = Preimage;
        type SubmissionDeposit = ConstU64<100>;
        type CreatePollOrigin = frame_system::EnsureSigned<u64>;
        type ClosePollOrigin = EnsureRoot<u64>;
        type RingAdminOrigin = frame_system::EnsureSigned<u64>;
        type Vote = Vote;
        type Tally = Tally;
        type TallyHandler = TallyHandler;
        type MaxDescriptionLength = ConstU32<256>;
        type MaxMembersInRing = ConstU32<16>;
        type NumRingLayers = ConstU32<2>;
        type WeightInfo = ();
    }

    // Test externalities initialization
    pub fn new_test_ext() -> TestExternalities {
        let mut storage = frame_system::GenesisConfig::<Test>::default()
            .build_storage()
            .unwrap();

        pallet_balances::GenesisConfig::<Test> {
            balances: vec![(ALICE, INITIAL_BALANCE), (BOB, INITIAL_BALANCE)],
            ..Default::default()
        }
        .assimilate_storage(&mut storage)
        .unwrap();

        storage.into()
    }
}

pub fn gen_ring<T: crate::pallet::Config>(
) -> BoundedVec<BoundedVec<H256, T::NumRingLayers>, T::MaxMembersInRing> {
    let mut csprng = OsRng;
    let nr = T::MaxMembersInRing::get() as usize;
    let nc = T::NumRingLayers::get() as usize;

    let ring: Vec<Vec<RistrettoPoint>> = (0..nr)
        .map(|_| {
            (0..nc)
                .map(|_| RistrettoPoint::random(&mut csprng))
                .collect()
        })
        .collect();

    let ring: BoundedVec<BoundedVec<H256, T::NumRingLayers>, T::MaxMembersInRing> = ring
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

    ring
}

pub fn gen_signature<T: crate::pallet::Config>(
    poll_id: PollId,
    vote: Vote,
) -> (
    H256,
    BoundedVec<H256, T::MaxMembersInRing>,
    BoundedVec<BoundedVec<H256, T::NumRingLayers>, T::MaxMembersInRing>,
    BoundedVec<H256, T::NumRingLayers>,
) {
    let mut csprng = OsRng;
    let secret_index = 1;
    let nr = T::MaxMembersInRing::get() as usize;
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
        let mut msg = poll_id.encode();
        msg.extend(vote.encode());
        msg
    };

    let signature = CLSAG::sign::<Sha512, OsRng>(ks.clone(), ring.clone(), secret_index, &message);
    let result = CLSAG::verify::<Sha512>(signature.clone(), &message);
    assert!(result);

    let challenge: H256 = signature.challenge.to_bytes().into();

    let responses: BoundedVec<H256, T::MaxMembersInRing> = signature
        .responses
        .iter()
        .map(|r| r.to_bytes().into())
        .collect::<Vec<H256>>()
        .try_into()
        .unwrap();

    let ring: BoundedVec<BoundedVec<H256, T::NumRingLayers>, T::MaxMembersInRing> = signature
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

    let key_images: BoundedVec<H256, T::NumRingLayers> = signature
        .key_images
        .iter()
        .map(|ki| ki.compress().to_bytes().into())
        .collect::<Vec<H256>>()
        .try_into()
        .unwrap();

    (challenge, responses, ring, key_images)
}
