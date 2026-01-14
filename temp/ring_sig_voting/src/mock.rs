use crate as ring_sig_voting;
use crate::types::*;
use curve25519_dalek::{ristretto::RistrettoPoint, scalar::Scalar};
use frame::runtime::prelude::*;
use frame::traits::EqualPrivilegeOnly;
use nazgul::blsag::BLSAG;
use nazgul::traits::{Sign, Verify};
use rand_core::OsRng;
use scale_info::prelude::{vec, vec::Vec};
use sha2::Sha512;

#[cfg(test)]
pub use tests::*;


pub const RING_SIZE: usize = 10;
pub const SECRET_INDEX: usize = 3;
#[cfg(test)]
pub const DEADLINE: u64 = 100;
#[cfg(feature = "runtime-benchmarks")]
pub const DEADLINE: u32 = 100;
pub const POLL_DESCRIPTION: &[u8] = b"Test Poll";
pub const POLL_METADATA: &[u8] = b"Poll metadata";
pub const SIMPLE_CIPHERTEXT: &[u8] = &[1, 2, 3, 4, 5];


#[cfg(test)]
pub mod tests {
    use super::*;
    use frame::testing_prelude::*;
    use polkadot_sdk::{pallet_preimage, pallet_scheduler};

    pub const ALICE: u64 = 1;
    pub const BOB: u64 = 2;

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
            RuntimeTask,
            RuntimeViewFunction
        )]
        pub struct Test;

        #[runtime::pallet_index(0)]
        pub type System = frame_system;

        #[runtime::pallet_index(1)]
        pub type Preimage = pallet_preimage;

        #[runtime::pallet_index(2)]
        pub type Scheduler = pallet_scheduler;

        #[runtime::pallet_index(3)]
        pub type RingSigVoting = ring_sig_voting;
    }

    parameter_types! {
        pub MaxWeight: Weight = Weight::from_parts(2_000_000_000_000, u64::MAX);
        pub const TallyVkBytes: &'static [u8] = include_bytes!("../../srs12.bin");
    }
    ord_parameter_types! {
        pub const AliceAccount: u64 = ALICE;
    }

    // System pallet configuration
    #[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
    impl frame_system::Config for Test {
        type Block = MockBlock<Test>;
    }

    impl pallet_preimage::Config for Test {
        type RuntimeEvent = RuntimeEvent;
        type WeightInfo = ();
        type Currency = ();
        type ManagerOrigin = EnsureRoot<u64>;
        type Consideration = ();
    }
    impl pallet_scheduler::Config for Test {
        type RuntimeEvent = RuntimeEvent;
        type RuntimeOrigin = RuntimeOrigin;
        type PalletsOrigin = OriginCaller;
        type RuntimeCall = RuntimeCall;
        type MaximumWeight = MaxWeight;
        type ScheduleOrigin = EnsureRoot<u64>;
        type MaxScheduledPerBlock = ConstU32<100>;
        type WeightInfo = ();
        type OriginPrivilegeCmp = EqualPrivilegeOnly;
        type Preimages = Preimage;
        type BlockNumberProvider = frame_system::Pallet<Test>;
    }

    impl ring_sig_voting::Config for Test {
        type RuntimeEvent = RuntimeEvent;
        type RuntimeCall = RuntimeCall;
        type Scheduler = Scheduler;
        type Preimages = Preimage;
        type MaxDescriptionLength = ConstU32<256>;
        type MaxRingSize = ConstU32<16>;
        type MaxVkLength = ConstU32<2048>;
        type MaxCiphertextLength = ConstU32<128>;
        type MaxVoteNum = ConstU32<1000>;
        type AdminOrigin = EnsureSignedBy<AliceAccount, u64>;
    }

    // Test externalities initialization
    pub fn new_test_ext() -> TestExternalities {
        let storage = frame_system::GenesisConfig::<Test>::default()
            .build_storage()
            .unwrap();

        storage.into()
    }

    // Helper to register a ring and return ring_id
    pub fn register_ring(
        origin: RuntimeOrigin,
        ring: Vec<CompressedRistrettoWrapper>,
    ) -> Result<u32, DispatchError> {
        let ring_id = RingSigVoting::ring_count();
        RingSigVoting::register_ring(origin, ring)?;
        Ok(ring_id)
    }

    // Helper to setup a basic poll
    pub fn setup_poll(ring_size: usize, deadline: u64) -> u32 {
        let ring = generate_test_ring(ring_size);
        register_ring(RuntimeOrigin::signed(ALICE), ring).expect("Ring registration failed");

        RingSigVoting::create_poll(
            RuntimeOrigin::signed(ALICE),
            0, // ring_id
            POLL_DESCRIPTION.to_vec(),
            POLL_METADATA.to_vec(),
            deadline,
            random_tally_key(),
            create_dummy_vk(),
        )
        .expect("Poll creation failed");

        0 // poll_id
    }

    // Helper to submit a vote
    pub fn submit_vote(
        origin: RuntimeOrigin,
        poll_id: u32,
        ephemeral_public_key: CompressedRistrettoWrapper,
        ciphertext: Vec<u8>,
        challenge: ScalarWrapper,
        responses: Vec<ScalarWrapper>,
        key_image: CompressedRistrettoWrapper,
    ) -> DispatchResult {
        RingSigVoting::vote(
            origin,
            poll_id,
            ephemeral_public_key,
            ciphertext,
            challenge,
            responses,
            key_image,
        )
    }

    // Helper to assert poll status
    pub fn assert_poll_status(poll_id: u32, expected_status: PollStatus) {
        let poll = RingSigVoting::polls(poll_id).unwrap();
        assert_eq!(poll.status, expected_status);
    }
}

// // Helper function to run to block n
// pub fn run_to_block(n: u64) {
//     while System::block_number() < n {
//         if System::block_number() > 0 {
//             System::on_finalize(System::block_number());
//         }
//         System::set_block_number(System::block_number() + 1);
//         System::on_initialize(System::block_number());
//         Scheduler::on_initialize(System::block_number());
//     }
// }

// Helper function to generate a test ring
pub fn generate_test_ring(size: usize) -> Vec<CompressedRistrettoWrapper> {
    (0..size)
        .map(|_| CompressedRistrettoWrapper::from(RistrettoPoint::random(&mut OsRng)))
        .collect()
}

// Helper function to generate a BLSAG signature for testing
pub fn generate_test_signature(
    ring_size: usize,
    secret_index: usize,
    message: &[u8],
) -> (
    ScalarWrapper,
    Vec<ScalarWrapper>,
    Vec<CompressedRistrettoWrapper>,
    CompressedRistrettoWrapper,
) {
    let secret_key = Scalar::random(&mut OsRng);

    let mut ring: Vec<RistrettoPoint> = (0..ring_size - 1)
        .map(|_| RistrettoPoint::random(&mut OsRng))
        .collect();

    let public_key = secret_key * &curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
    ring[secret_index] = public_key;

    let signature = BLSAG::sign::<Sha512, OsRng>(secret_key, ring.clone(), secret_index, message);
    assert!(BLSAG::verify::<Sha512>(signature.clone(), message));

    (
        ScalarWrapper::from(signature.challenge),
        signature.responses.into_iter().map(Into::into).collect(),
        signature.ring.into_iter().map(|p| p.into()).collect(),
        CompressedRistrettoWrapper::from(signature.key_image),
    )
}

// Helper function to create a dummy verification key
pub fn create_dummy_vk() -> VkWrapper {
    use ark_bls12_381::Bls12_381;
    use ark_bls12_381::{G1Affine, G2Affine};
    use ark_ec::AffineRepr;
    use ark_groth16::VerifyingKey;

    VkWrapper::from(VerifyingKey::<Bls12_381> {
        alpha_g1: G1Affine::generator(),
        beta_g2: G2Affine::generator(),
        gamma_g2: G2Affine::generator(),
        delta_g2: G2Affine::generator(),
        gamma_abc_g1: vec![G1Affine::generator(); 3],
    })
}

pub fn create_dummy_proof() -> ProofWrapper {
    use ark_bls12_381::{Bls12_381, G1Affine, G2Affine};
    use ark_ec::AffineRepr;
    use ark_groth16::Proof;

    let proof = Proof::<Bls12_381> {
        a: G1Affine::generator(),
        b: G2Affine::generator(),
        c: G1Affine::generator(),
    };
    ProofWrapper::from(proof)
}

// Helper to generate random tally public key
pub fn random_tally_key() -> CompressedRistrettoWrapper {
    CompressedRistrettoWrapper::from(RistrettoPoint::random(&mut OsRng))
}

    // Helper to create encrypted vote and signature
    pub fn create_vote_with_signature<T: crate::Config>(
        ring_size: usize,
        secret_index: usize,
        ciphertext_data: &[u8],
    ) -> (
        CompressedRistrettoWrapper,
        Vec<u8>,
        ScalarWrapper,
        Vec<ScalarWrapper>,
        Vec<CompressedRistrettoWrapper>,
        CompressedRistrettoWrapper,
    ) {
        let ephemeral_public_key = random_tally_key();
        let ciphertext = ciphertext_data.to_vec();

        let encrypted_vote = EncryptedVote::<T::MaxCiphertextLength> {
            ephemeral_public_key: ephemeral_public_key.clone(),
            ciphertext: BoundedVec::try_from(ciphertext.clone()).unwrap(),
        };
        let message = encrypted_vote.to_bytes();

        let (challenge, responses, sig_ring, key_image) =
            generate_test_signature(ring_size, secret_index, &message);

        (
            ephemeral_public_key,
            ciphertext,
            challenge,
            responses,
            sig_ring,
            key_image,
        )
    }

