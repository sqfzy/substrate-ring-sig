use super::*;
use crate::{mock::*, types::*};
use curve25519_dalek::{ristretto::RistrettoPoint, scalar::Scalar};
use frame::{deps::frame_benchmarking::v2::*, prelude::*};
use rand_core::OsRng;

#[benchmarks]
mod benchmarks {
    use super::*;
    use crate::Pallet as RingSigVoting;
    use frame_system::RawOrigin;

fn get_admin<T: Config>() -> T::AccountId {
        whitelisted_caller()
    }

    #[benchmark]
    fn register_ring() {
        let caller = get_admin::<T>();
        let ring = generate_test_ring(RING_SIZE);

        #[extrinsic_call]
        _(RawOrigin::Signed(caller), ring);

        assert_eq!(RingSigVoting::<T>::ring_count(), 1);
    }

    #[benchmark]
    fn create_poll() {
        let caller = get_admin::<T>();
        let ring = generate_test_ring(RING_SIZE);
        RingSigVoting::<T>::register_ring(RawOrigin::Signed(caller.clone()).into(), ring).unwrap();

        let description = POLL_DESCRIPTION.to_vec();
        let metadata = POLL_METADATA.to_vec();
        let deadline = DEADLINE.into();
        let poll_public_key = CompressedRistrettoWrapper::from(RistrettoPoint::random(&mut OsRng));

        let tally_vk = create_dummy_vk();

        #[extrinsic_call]
        _(
            RawOrigin::Signed(caller),
            0,
            description,
            metadata,
            deadline,
            poll_public_key,
            tally_vk,
        );

        assert_eq!(RingSigVoting::<T>::poll_count(), 1);
    }

    #[benchmark]
    fn vote() {
        let caller = get_admin::<T>();

        let (ephemeral_public_key, ciphertext, challenge, responses, sig_ring, key_image) =
            create_vote_with_signature::<T>(
                RING_SIZE,
                SECRET_INDEX,
                SIMPLE_CIPHERTEXT,
            );

        RingSigVoting::<T>::register_ring(RawOrigin::Signed(caller.clone()).into(), sig_ring).unwrap();

        let tally_vk = create_dummy_vk();
        RingSigVoting::<T>::create_poll(
            RawOrigin::Signed(caller).into(),
            0,
            POLL_DESCRIPTION.to_vec(),
            POLL_METADATA.to_vec(),
            DEADLINE.into(),
            CompressedRistrettoWrapper::from(RistrettoPoint::random(&mut OsRng)),
            tally_vk,
        )
        .unwrap();

        let caller: T::AccountId = whitelisted_caller();

        #[extrinsic_call]
        _(
            RawOrigin::Signed(caller),
            0,
            ephemeral_public_key,
            ciphertext,
            challenge,
            responses,
            key_image,
        );

        assert_eq!(RingSigVoting::<T>::encrypted_votes(0).len(), 1);
    }

    #[benchmark]
    fn tally_poll() {
        let caller = get_admin::<T>();
        let ring = generate_test_ring(RING_SIZE);
        RingSigVoting::<T>::register_ring(RawOrigin::Signed(caller.clone()).into(), ring).unwrap();

        let tally_vk = create_dummy_vk();
        RingSigVoting::<T>::create_poll(
            RawOrigin::Signed(caller.clone()).into(),
            0,
            POLL_DESCRIPTION.to_vec(),
            POLL_METADATA.to_vec(),
            DEADLINE.into(),
            CompressedRistrettoWrapper::from(RistrettoPoint::random(&mut OsRng)),
            tally_vk,
        )
        .unwrap();

        #[extrinsic_call]
        _(RawOrigin::Signed(caller), 0);

        let poll = RingSigVoting::<T>::polls(0).unwrap();
        assert_eq!(poll.status, PollStatus::Tallying);
    }

    #[benchmark]
    fn cancel_poll() {
        let caller = get_admin::<T>();
        let ring = generate_test_ring(RING_SIZE);
        RingSigVoting::<T>::register_ring(RawOrigin::Signed(caller.clone()).into(), ring).unwrap();

        let tally_vk = create_dummy_vk();
        RingSigVoting::<T>::create_poll(
            RawOrigin::Signed(caller.clone()).into(),
            0,
            POLL_DESCRIPTION.to_vec(),
            POLL_METADATA.to_vec(),
            DEADLINE.into(),
            CompressedRistrettoWrapper::from(RistrettoPoint::random(&mut OsRng)),
            tally_vk,
        )
        .unwrap();

        #[extrinsic_call]
        _(RawOrigin::Signed(caller), 0);

        let poll = RingSigVoting::<T>::polls(0).unwrap();
        assert_eq!(poll.status, PollStatus::Cancelled);
    }

    #[benchmark]
    fn pause_poll() {
        let caller = get_admin::<T>();
        let ring = generate_test_ring(RING_SIZE);
        RingSigVoting::<T>::register_ring(RawOrigin::Signed(caller.clone()).into(), ring).unwrap();

        let tally_vk = create_dummy_vk();
        RingSigVoting::<T>::create_poll(
            RawOrigin::Signed(caller.clone()).into(),
            0,
            POLL_DESCRIPTION.to_vec(),
            POLL_METADATA.to_vec(),
            DEADLINE.into(),
            CompressedRistrettoWrapper::from(RistrettoPoint::random(&mut OsRng)),
            tally_vk,
        )
        .unwrap();

        #[extrinsic_call]
        _(RawOrigin::Signed(caller), 0);

        let poll = RingSigVoting::<T>::polls(0).unwrap();
        assert_eq!(poll.status, PollStatus::Paused);
    }

    #[benchmark]
    fn set_deadline() {
        let caller = get_admin::<T>();
        let ring = generate_test_ring(RING_SIZE);
        RingSigVoting::<T>::register_ring(RawOrigin::Signed(caller.clone()).into(), ring).unwrap();

        let tally_vk = create_dummy_vk();
        RingSigVoting::<T>::create_poll(
            RawOrigin::Signed(caller.clone()).into(),
            0,
            POLL_DESCRIPTION.to_vec(),
            POLL_METADATA.to_vec(),
            DEADLINE.into(),
            CompressedRistrettoWrapper::from(RistrettoPoint::random(&mut OsRng)),
            tally_vk,
        )
        .unwrap();

        let new_deadline = (DEADLINE * 2).into();

        #[extrinsic_call]
        _(RawOrigin::Signed(caller), 0, new_deadline);

        let poll = RingSigVoting::<T>::polls(0).unwrap();
        assert_eq!(poll.deadline, new_deadline);
    }

    #[benchmark]
    fn tally() {
        let caller = get_admin::<T>();
        let ring = generate_test_ring(RING_SIZE);
        RingSigVoting::<T>::register_ring(RawOrigin::Signed(caller.clone()).into(), ring).unwrap();

        let tally_vk = create_dummy_vk();
        RingSigVoting::<T>::create_poll(
            RawOrigin::Signed(caller.clone()).into(),
            0,
            POLL_DESCRIPTION.to_vec(),
            POLL_METADATA.to_vec(),
            DEADLINE.into(),
            CompressedRistrettoWrapper::from(RistrettoPoint::random(&mut OsRng)),
            tally_vk,
        )
        .unwrap();

        RingSigVoting::<T>::tally_poll(RawOrigin::Signed(caller.clone()).into(), 0).unwrap();

        let tally_result = 42u32;
        let proof = create_dummy_proof();

        #[extrinsic_call]
        _(RawOrigin::Signed(caller), 0, tally_result, proof);

        let poll = RingSigVoting::<T>::polls(0).unwrap();
        assert_eq!(poll.status, PollStatus::Completed);
        assert_eq!(poll.tally, Some(tally_result));
    }

    impl_benchmark_test_suite!(
        RingSigVoting,
        crate::mock::new_test_ext(),
        crate::mock::Test
    );
}
