use super::*;
use crate::types::*;
use curve25519_dalek::{ristretto::RistrettoPoint, scalar::Scalar};
use frame::{deps::frame_benchmarking::v2::*, prelude::*};
use nazgul::blsag::BLSAG;
use nazgul::traits::Sign;
use rand_core::OsRng;
use scale_info::prelude::vec;
use sha2::Sha512;

#[benchmarks]
mod benchmarks {
    use super::*;
    use crate::Pallet as RingSigVoting;
    use frame_system::RawOrigin;

    // Helper to generate test ring
    fn generate_ring<T: Config>(size: u32) -> Vec<CompressedRistrettoWrapper> {
        let mut csprng = OsRng;
        (0..size)
            .map(|_| CompressedRistrettoWrapper::from(RistrettoPoint::random(&mut csprng)))
            .collect()
    }

    // Helper to generate test signature
    fn generate_signature(
        ring_size: usize,
        secret_index: usize,
        message: &[u8],
    ) -> (
        ScalarWrapper,
        Vec<ScalarWrapper>,
        CompressedRistrettoWrapper,
    ) {
        let mut csprng = OsRng;
        let secret_key = Scalar::random(&mut csprng);
        let mut ring: Vec<RistrettoPoint> = (0..ring_size)
            . map(|_| RistrettoPoint::random(&mut csprng))
            .collect();
        
        let public_key = secret_key * &curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
        ring[secret_index] = public_key;

        let signature = BLSAG::sign::<Sha512, OsRng>(
            vec![secret_key],
            ring.clone(),
            secret_index,
            message,
        );

        let challenge = ScalarWrapper::from(signature.challenge);
        let responses: Vec<ScalarWrapper> = signature
            .responses
            .into_iter()
            .map(Into::into)
            .collect();
        let key_image = CompressedRistrettoWrapper::from(signature.key_image);

        (challenge, responses, key_image)
    }

    fn create_dummy_vk() -> VkWrapper {
        use ark_bls12_381::{Bls12_381, Fr, G1Affine, G2Affine};
        use ark_groth16::VerifyingKey;
        use ark_ec::AffineRepr;
        
        let vk = VerifyingKey::<Bls12_381> {
            alpha_g1: G1Affine::generator(),
            beta_g2: G2Affine::generator(),
            gamma_g2: G2Affine::generator(),
            delta_g2: G2Affine::generator(),
            gamma_abc_g1: vec![G1Affine::generator(); 3],
        };
        
        VkWrapper::from(vk)
    }

    fn create_dummy_proof() -> ProofWrapper {
        use ark_bls12_381::{Bls12_381, G1Affine, G2Affine};
        use ark_groth16::Proof;
        use ark_ec::AffineRepr;

        let proof = Proof::<Bls12_381> {
            a: G1Affine::generator(),
            b: G2Affine::generator(),
            c: G1Affine::generator(),
        };

        ProofWrapper::from(proof)
    }

    #[benchmark]
    fn register_ring_group() {
        let ring_size = 10u32;
        let ring = generate_ring::<T>(ring_size);

        #[extrinsic_call]
        _(RawOrigin::Root, ring);

        assert_eq!(RingSigVoting::<T>::ring_count(), 1);
    }

    #[benchmark]
    fn create_poll() {
        let ring = generate_ring::<T>(10);
        RingSigVoting::<T>::register_ring_group(RawOrigin::Root. into(), ring). unwrap();

        let description = b"Benchmark Poll".to_vec();
        let metadata_hash = H256::random();
        let deadline = 1000u32. into();
        let tally_public_key =
            CompressedRistrettoWrapper::from(RistrettoPoint::random(&mut OsRng));
        let tally_vk = create_dummy_vk();

        #[extrinsic_call]
        _(
            RawOrigin::Root,
            0,
            description,
            metadata_hash,
            deadline,
            tally_public_key,
            tally_vk,
        );

        assert_eq!(RingSigVoting::<T>::poll_count(), 1);
    }

    #[benchmark]
    fn vote() {
        // Setup
        let ring = generate_ring::<T>(10);
        RingSigVoting::<T>::register_ring_group(RawOrigin::Root.into(), ring. clone()).unwrap();

        let tally_vk = create_dummy_vk();
        RingSigVoting::<T>::create_poll(
            RawOrigin::Root.into(),
            0,
            b"Test".to_vec(),
            H256::random(),
            1000u32.into(),
            CompressedRistrettoWrapper::from(RistrettoPoint::random(&mut OsRng)),
            tally_vk,
        )
        .unwrap();

        // Create vote
        let ephemeral_public_key =
            CompressedRistrettoWrapper::from(RistrettoPoint::random(&mut OsRng));
        let ciphertext = vec![1u8; 64];

        let encrypted_vote = EncryptedVote {
            ephemeral_public_key: ephemeral_public_key. clone(),
            ciphertext: BoundedVec::try_from(ciphertext.clone()). unwrap(),
        };
        let message = encrypted_vote.to_bytes();

        let (challenge, responses, key_image) = generate_signature(10, 3, &message);
        let responses_bounded: BoundedVec<ScalarWrapper, T::MaxRingSize> =
            responses.try_into().unwrap();

        let caller: T::AccountId = whitelisted_caller();

        #[extrinsic_call]
        _(
            RawOrigin::Signed(caller),
            0,
            ephemeral_public_key,
            ciphertext,
            challenge,
            responses_bounded. into_inner(),
            key_image,
        );

        assert_eq!(RingSigVoting::<T>::encrypted_votes(0). len(), 1);
    }

    #[benchmark]
    fn tally_poll() {
        // Setup
        let ring = generate_ring::<T>(10);
        RingSigVoting::<T>::register_ring_group(RawOrigin::Root.into(), ring). unwrap();

        let tally_vk = create_dummy_vk();
        RingSigVoting::<T>::create_poll(
            RawOrigin::Root.into(),
            0,
            b"Test".to_vec(),
            H256::random(),
            1000u32.into(),
            CompressedRistrettoWrapper::from(RistrettoPoint::random(&mut OsRng)),
            tally_vk,
        )
        .unwrap();

        #[extrinsic_call]
        _(RawOrigin::Root, 0);

        let poll = RingSigVoting::<T>::polls(0). unwrap();
        assert_eq!(poll.status, PollStatus::Tallying);
    }

    #[benchmark]
    fn cancel_poll() {
        // Setup
        let ring = generate_ring::<T>(10);
        RingSigVoting::<T>::register_ring_group(RawOrigin::Root.into(), ring).unwrap();

        let tally_vk = create_dummy_vk();
        RingSigVoting::<T>::create_poll(
            RawOrigin::Root.into(),
            0,
            b"Test".to_vec(),
            H256::random(),
            1000u32.into(),
            CompressedRistrettoWrapper::from(RistrettoPoint::random(&mut OsRng)),
            tally_vk,
        )
        .unwrap();

        #[extrinsic_call]
        _(RawOrigin::Root, 0);

        let poll = RingSigVoting::<T>::polls(0). unwrap();
        assert_eq!(poll.status, PollStatus::Cancelled);
    }

    #[benchmark]
    fn pause_poll() {
        // Setup
        let ring = generate_ring::<T>(10);
        RingSigVoting::<T>::register_ring_group(RawOrigin::Root.into(), ring).unwrap();

        let tally_vk = create_dummy_vk();
        RingSigVoting::<T>::create_poll(
            RawOrigin::Root.into(),
            0,
            b"Test".to_vec(),
            H256::random(),
            1000u32.into(),
            CompressedRistrettoWrapper::from(RistrettoPoint::random(&mut OsRng)),
            tally_vk,
        )
        .unwrap();

        #[extrinsic_call]
        _(RawOrigin::Root, 0);

        let poll = RingSigVoting::<T>::polls(0).unwrap();
        assert_eq!(poll. status, PollStatus::Paused);
    }

    #[benchmark]
    fn set_deadline() {
        // Setup
        let ring = generate_ring::<T>(10);
        RingSigVoting::<T>::register_ring_group(RawOrigin::Root.into(), ring).unwrap();

        let tally_vk = create_dummy_vk();
        RingSigVoting::<T>::create_poll(
            RawOrigin::Root.into(),
            0,
            b"Test". to_vec(),
            H256::random(),
            1000u32.into(),
            CompressedRistrettoWrapper::from(RistrettoPoint::random(&mut OsRng)),
            tally_vk,
        )
        .unwrap();

        let new_deadline = 2000u32.into();

        #[extrinsic_call]
        _(RawOrigin::Root, 0, new_deadline);

        let poll = RingSigVoting::<T>::polls(0).unwrap();
        assert_eq!(poll.deadline, new_deadline);
    }

    #[benchmark]
    fn tally() {
        // Setup
        let ring = generate_ring::<T>(10);
        RingSigVoting::<T>::register_ring_group(RawOrigin::Root.into(), ring).unwrap();

        let tally_vk = create_dummy_vk();
        RingSigVoting::<T>::create_poll(
            RawOrigin::Root.into(),
            0,
            b"Test".to_vec(),
            H256::random(),
            1000u32. into(),
            CompressedRistrettoWrapper::from(RistrettoPoint::random(&mut OsRng)),
            tally_vk. clone(),
        )
        . unwrap();

        RingSigVoting::<T>::tally_poll(RawOrigin::Root.into(), 0).unwrap();

        let tally_result = 42u32;
        let proof = create_dummy_proof();

        #[extrinsic_call]
        _(RawOrigin::Root, 0, tally_result, proof);

        let poll = RingSigVoting::<T>::polls(0).unwrap();
        assert_eq!(poll.status, PollStatus::Completed);
        assert_eq!(poll.tally, Some(tally_result));
    }

    impl_benchmark_test_suite!(RingSigVoting, crate::mock::new_test_ext(), crate::mock::Test);
}
