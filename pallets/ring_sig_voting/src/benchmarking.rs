use super::*;
use frame::{deps::frame_benchmarking::v2::*, prelude::*};
use scale_info::prelude::vec;
use crate::types::simple_voting::*;

#[benchmarks(
    where
        T: pallet::Config<
            Vote = Vote,
            Tally = Tally
        >
)]
mod benchmarks {
    use super::*;
    use crate::pallet::Pallet as RingSigVoting;
    use crate::{mock::*};
    use frame::deps::frame_support::traits::Currency;
    use frame_system::RawOrigin;
    use curve25519_dalek::{constants::RISTRETTO_BASEPOINT_POINT, scalar::Scalar};
    use rand_core::OsRng;
    use primitive_types::H128;

    const BIG_ENOUGH: u32 = 1_000_000_000;

    #[benchmark]
    fn register_ring_group() {
        let caller: T::AccountId = whitelisted_caller();
        let ring = gen_ring::<T>();

        #[extrinsic_call]
        RingSigVoting::register_ring_group(RawOrigin::Signed(caller), ring);

        assert!(RingGroups::<T>::get(0).is_some());
    }

    #[benchmark]
    fn create_poll() {
        let caller: T::AccountId = whitelisted_caller();
        let poll_id = 0;
        let description = b"Poll 0".to_vec();
        let ring_id = 0;
        let ring = gen_ring::<T>();

        // 生成加密密钥对
        let mut csprng = OsRng;
        let private_key_scalar = Scalar::random(&mut csprng);
        let public_key_point = private_key_scalar * RISTRETTO_BASEPOINT_POINT;
        let encryption_pubkey: H256 = public_key_point.compress().to_bytes().into();

        let balance = T::Currency::minimum_balance() * BIG_ENOUGH. into();
        T::Currency::make_free_balance_be(&caller, balance);

        RingSigVoting::<T>::register_ring_group(RawOrigin::Signed(caller. clone()). into(), ring)
            .unwrap();
        assert!(RingGroups::<T>::get(ring_id).is_some());

        #[extrinsic_call]
        RingSigVoting::create_poll(
            RawOrigin::Signed(caller),
            description.clone().try_into().unwrap(),
            ring_id,
            None,
            None,
            encryption_pubkey,
        );

        let poll = Polls::<T>::get(poll_id).unwrap();
        assert_eq!(poll.description. into_inner(), description);
        assert_eq!(poll.status, PollStatus::Voting);
    }

    #[benchmark]
    fn close_poll() {
        let caller: T::AccountId = whitelisted_caller();
        let poll_id = 0;
        let description = b"Poll 0".to_vec();
        let ring_id = 0;
        let ring = gen_ring::<T>();

        // 生成真实的密钥对
        let mut csprng = OsRng;
        let private_key_scalar = Scalar::random(&mut csprng);
        let public_key_point = private_key_scalar * RISTRETTO_BASEPOINT_POINT;
        
        let encryption_privkey: H256 = private_key_scalar.to_bytes().into();
        let encryption_pubkey: H256 = public_key_point.compress().to_bytes().into();
        let tally = (5u32, 3u32);

        let balance = T::Currency::minimum_balance() * BIG_ENOUGH. into();
        T::Currency::make_free_balance_be(&caller, balance);

        RingSigVoting::<T>::register_ring_group(RawOrigin::Signed(caller.clone()).into(), ring)
            . unwrap();
        assert!(RingGroups::<T>::get(ring_id).is_some());

        RingSigVoting::<T>::create_poll(
            RawOrigin::Signed(caller. clone()). into(),
            description.clone(). try_into().unwrap(),
            ring_id,
            None,
            None,
            encryption_pubkey,
        )
        .unwrap();
        let poll = Polls::<T>::get(poll_id).unwrap();
        assert_eq!(poll.status, PollStatus::Voting);

        #[extrinsic_call]
        RingSigVoting::close_poll(RawOrigin::Root, poll_id, encryption_privkey, tally);
        
        assert_eq!(Polls::<T>::get(poll_id).unwrap().status, PollStatus::Closed);
    }

    #[benchmark]
    fn anonymous_vote() {
        let caller: T::AccountId = whitelisted_caller();
        let poll_id = 0;
        let description = b"Poll 0".to_vec();
        let ring_id = 0;
        
        // 生成加密密钥对
        let mut csprng = OsRng;
        let private_key_scalar = Scalar::random(&mut csprng);
        let public_key_point = private_key_scalar * RISTRETTO_BASEPOINT_POINT;
        let encryption_pubkey: H256 = public_key_point.compress().to_bytes(). into();
        
        // 模拟加密数据
        let ephemeral_pubkey: H256 = [2u8; 32].into();
        let ciphertext: BoundedVec<u8, <T as crate::Config>::MaxVoteSize> = 
            vec![1, 2, 3, 4].try_into().unwrap();
        let auth_tag: H128 = [0u8; 16].into();
        
        let vote = Vote::Yea;
        let (challenge, responses, ring, key_images) = gen_signature_for_encrypted::<T>(
            poll_id,
            vote,
            ephemeral_pubkey.0,
            &ciphertext,
            auth_tag.0,
        );

        let balance = T::Currency::minimum_balance() * BIG_ENOUGH.into();
        T::Currency::make_free_balance_be(&caller, balance);

        RingSigVoting::<T>::register_ring_group(RawOrigin::Signed(caller. clone()).into(), ring. clone())
            .unwrap();
        assert!(RingGroups::<T>::get(ring_id).is_some());

        RingSigVoting::<T>::create_poll(
            RawOrigin::Signed(caller.clone()).into(),
            description.clone().try_into().unwrap(),
            ring_id,
            None,
            None,
            encryption_pubkey,
        )
        .unwrap();
        let poll = Polls::<T>::get(poll_id).unwrap();
        assert_eq!(poll.status, PollStatus::Voting);

        #[extrinsic_call]
        RingSigVoting::anonymous_vote(
            RawOrigin::Signed(caller),
            poll_id,
            ephemeral_pubkey,
            ciphertext,
            auth_tag,
            challenge,
            responses,
            key_images,
        );

        let encrypted_votes = EncryptedVotes::<T>::get(poll_id);
        assert_eq!(encrypted_votes.len(), 1);
    }
}

// #[benchmarks(
//     where
//         T: pallet::Config<
//             Vote = Vote,
//             Tally = Tally
//         >
// )]
// mod benchmarks {
//     use super::*;
//     use crate::pallet::Pallet as RingSigVoting;
//     use crate::{mock::*, types::simple_voting::*};
//     use frame::deps::frame_support::traits::Currency;
//     use frame_system::RawOrigin;
//
//     const BIG_ENOUGH: u32 = 1_000_000_000;
//
//     #[benchmark]
//     fn register_ring_group() {
//         let caller: T::AccountId = whitelisted_caller();
//         let ring = gen_ring::<T>();
//
//         #[extrinsic_call]
//         RingSigVoting::register_ring_group(RawOrigin::Signed(caller), ring);
//
//         assert!(RingGroups::<T>::get(0).is_some());
//     }
//
//     #[benchmark]
//     fn create_poll() {
//         let caller: T::AccountId = whitelisted_caller();
//         let poll_id = 0;
//         let description = b"Poll 0".to_vec();
//         let ring_id = 0;
//         let ring = gen_ring::<T>();
//
//         let balance = T::Currency::minimum_balance() * BIG_ENOUGH.into();
//         T::Currency::make_free_balance_be(&caller, balance);
//
//         RingSigVoting::<T>::register_ring_group(RawOrigin::Signed(caller.clone()).into(), ring)
//             .unwrap();
//         assert!(RingGroups::<T>::get(ring_id).is_some());
//
//         #[extrinsic_call]
//         RingSigVoting::create_poll(
//             RawOrigin::Signed(caller),
//             description.clone().try_into().unwrap(),
//             poll_id,
//             None,
//         );
//
//         let poll = Polls::<T>::get(poll_id).unwrap();
//         assert_eq!(poll.description.into_inner(), description);
//         assert_eq!(poll.status, PollStatus::Voting);
//     }
//
//     #[benchmark]
//     fn close_poll() {
//         let caller: T::AccountId = whitelisted_caller();
//         let poll_id = 0;
//         let description = b"Poll 0".to_vec();
//         let ring_id = 0;
//         let ring = gen_ring::<T>();
//
//         let balance = T::Currency::minimum_balance() * BIG_ENOUGH.into();
//         T::Currency::make_free_balance_be(&caller, balance);
//
//         RingSigVoting::<T>::register_ring_group(RawOrigin::Signed(caller.clone()).into(), ring)
//             .unwrap();
//         assert!(RingGroups::<T>::get(ring_id).is_some());
//
//         RingSigVoting::<T>::create_poll(
//             RawOrigin::Signed(caller.clone()).into(),
//             description.clone().try_into().unwrap(),
//             0,
//             None,
//         )
//         .unwrap();
//         let poll = Polls::<T>::get(poll_id).unwrap();
//         assert_eq!(poll.description.into_inner(), description);
//         assert_eq!(poll.status, PollStatus::Voting);
//
//         #[extrinsic_call]
//         RingSigVoting::close_poll(RawOrigin::Root, poll_id);
//         assert_eq!(Polls::<T>::get(poll_id).unwrap().status, PollStatus::Closed);
//     }
//
//     #[benchmark]
//     fn anonymous_vote() {
//         let caller: T::AccountId = whitelisted_caller();
//         let vote = Vote::Yea;
//         let poll_id = 0;
//         let description = b"Poll 0".to_vec();
//         let ring_id = 0;
//         let (challenge, responses, ring, key_images) = gen_signature::<T>(poll_id, vote.clone());
//
//         let balance = T::Currency::minimum_balance() * BIG_ENOUGH.into();
//         T::Currency::make_free_balance_be(&caller, balance);
//
//         RingSigVoting::<T>::register_ring_group(RawOrigin::Signed(caller.clone()).into(), ring)
//             .unwrap();
//         assert!(RingGroups::<T>::get(ring_id).is_some());
//
//         RingSigVoting::<T>::create_poll(
//             RawOrigin::Signed(caller.clone()).into(),
//             description.clone().try_into().unwrap(),
//             0,
//             None,
//         )
//         .unwrap();
//         let poll = Polls::<T>::get(poll_id).unwrap();
//         assert_eq!(poll.description.into_inner(), description);
//         assert_eq!(poll.status, PollStatus::Voting);
//
//         #[extrinsic_call]
//         RingSigVoting::anonymous_vote(
//             RawOrigin::Signed(caller),
//             poll_id,
//             vote,
//             challenge,
//             responses,
//             key_images,
//         );
//
//         assert_eq!(PollVotes::<T>::get(poll_id), (1, 0));
//     }
// }
