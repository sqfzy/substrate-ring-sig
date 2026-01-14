#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(any(test, feature = "runtime-benchmarks"))]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod types;

pub use pallet::*;
pub use types::*;


#[frame::pallet]
pub mod pallet {
    use super::*;
    use frame::prelude::*;
    use frame::traits::{schedule, QueryPreimage, StorePreimage};
    use scale_info::prelude::vec::Vec;

    use ark_bls12_381::Fr;
    use ark_std::rand::SeedableRng;
    use nazgul::blsag::BLSAG;
    use nazgul::traits::Verify;
    use rand_chacha::{ChaCha20Rng};
    use ark_serialize::CanonicalSerialize;
    use blake2::{Blake2s, Digest};


    #[pallet::pallet]
    pub struct Pallet<T>(_);

    // Configuration trait for the pallet.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        // Defines the event type for the pallet.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        type RuntimeCall: Parameter
            + Dispatchable<RuntimeOrigin = Self::RuntimeOrigin>
            + From<Call<Self>>
            + IsType<<Self as frame_system::Config>::RuntimeCall>
            + From<frame_system::Call<Self>>;

        type Scheduler: schedule::v3::Anon<
                BlockNumberFor<Self>,
                RuntimeCallFor<Self>,
                <<Self as frame_system::Config>::RuntimeOrigin as OriginTrait>::PalletsOrigin,
                Hasher = Self::Hashing,
            > + schedule::v3::Named<
                BlockNumberFor<Self>,
                RuntimeCallFor<Self>,
                <<Self as frame_system::Config>::RuntimeOrigin as OriginTrait>::PalletsOrigin,
                Hasher = Self::Hashing,
            >;

        type Preimages: QueryPreimage<H = Self::Hashing> + StorePreimage;

        /// Maximum description length
        #[pallet::constant]
        type MaxDescriptionLength: Get<u32>;

        /// Maximum ring size
        #[pallet::constant]
        type MaxRingSize: Get<u32>;

        /// Maximum verification key length
        #[pallet::constant]
        type MaxVkLength: Get<u32>;

        /// Maximum ciphertext length
        #[pallet::constant]
        type MaxCiphertextLength: Get<u32>;

        /// Maximum number of votes per poll
        #[pallet::constant]
        type MaxVoteNum: Get<u32>;

        /// Admin origin for privileged operations
        type AdminOrigin: EnsureOrigin<Self::RuntimeOrigin>;

        /// 验证密钥的二进制数据 (Hardcoded from Trusted SRS + Circuit)
        #[pallet::constant]
        type TallyVkBytes: Get<&'static [u8]>;
    }

    /// Storage for ring signature groups
    #[pallet::storage]
    #[pallet::getter(fn rings)]
    pub type Rings<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        u32,
        BoundedVec<CompressedRistrettoWrapper, T::MaxRingSize>,
        OptionQuery,
    >;

    /// Storage for polls
    #[pallet::storage]
    #[pallet::getter(fn polls)]
    pub type Polls<T: Config> = StorageMap<_, Blake2_128Concat, u32, Poll<T>, OptionQuery>;

    /// Storage for encrypted votes (poll_id -> vote_index -> encrypted_vote)
    #[pallet::storage]
    #[pallet::getter(fn encrypted_votes)]
    pub type EncryptedVotes<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        u32,
        BoundedVec<EncryptedVote<T::MaxCiphertextLength>, T::MaxVoteNum>,
        ValueQuery,
    >;

    /// Storage for used key images to prevent double voting
    #[pallet::storage]
    #[pallet::getter(fn used_key_images)]
    pub type UsedKeyImages<T: Config> =
        StorageMap<_, Blake2_128Concat, CompressedRistrettoWrapper, (), OptionQuery>;

    /// Poll counter
    #[pallet::storage]
    #[pallet::getter(fn poll_count)]
    pub type PollCount<T: Config> = StorageValue<_, u32, ValueQuery>;

    /// Ring counter
    #[pallet::storage]
    #[pallet::getter(fn ring_count)]
    pub type RingCount<T: Config> = StorageValue<_, u32, ValueQuery>;

    /// Events
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A ring group has been registered
        RingRegistered { ring_id: PollId },
        /// A poll has been created
        PollCreated {
            poll_id: PollId,
        },
        /// A vote has been submitted
        VoteSubmitted { poll_id: PollId, vote_index: u32 },
        /// A poll has been tallied
        PollCompleted { poll_id: PollId, tally: Tally },
        /// A poll is been tallying
        PollTallying { poll_id: PollId },
        /// A poll has been cancelled
        PollCancelled { poll_id: PollId },
        /// A poll has been paused
        PollPaused { poll_id: PollId },
        /// A poll has been actived
        PollActive { poll_id: PollId },
        /// A poll deadline has been updated
        PollDeadlineUpdated {
            poll_id: PollId,
            new_deadline: BlockNumberFor<T>,
        },
    }

    /// Errors
    #[pallet::error]
    pub enum Error<T> {
        /// No permission to perform this operation
        NoPermission,
        /// Invalid deadline
        InvalidDeadline,
        /// Ring not found
        RingNotFound,
        /// Poll not found
        PollNotFound,
        /// Invalid poll status
        InvalidPollStatus,
        /// Invalid signature
        InvalidSignature,
        /// Key image already used
        KeyImageAlreadyUsed,
        /// Invalid proof
        InvalidProof,
        /// Invalid status transition
        InvalidStatusTransition,
        /// Ring too large
        RingTooLarge,
        /// Description too long
        DescriptionTooLong,
        /// Verification key too large
        VkTooLarge,
        /// Ciphertext too large
        CiphertextTooLarge,
        /// Overflow in counter
        Overflow,
        /// Call is too large to be scheduled inline
        CallTooLarge,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Register a ring signature group
        ///
        /// # Arguments
        /// * `origin` - Must be admin
        /// * `ring` - Vector of public keys forming the ring
        #[pallet::call_index(0)]
        #[pallet::weight(10_000 + (ring.len() as u64) * 1_000)]
        pub fn register_ring(
            origin: OriginFor<T>,
            ring: Vec<CompressedRistrettoWrapper>,
        ) -> DispatchResult {
            // Permission check
            T::AdminOrigin::ensure_origin(origin)?;

            // Convert to bounded vec
            let bounded_ring: BoundedVec<CompressedRistrettoWrapper, T::MaxRingSize> =
                ring.try_into().map_err(|_| Error::<T>::RingTooLarge)?;

            // Get next ring ID
            let ring_id = RingCount::<T>::get();
            let next_ring_id = ring_id.wrapping_add(1);

            // Store ring
            Rings::<T>::insert(ring_id, bounded_ring);
            RingCount::<T>::put(next_ring_id);

            // Emit event
            Self::deposit_event(Event::RingRegistered { ring_id });

            Ok(())
        }

        /// Create a new poll
        ///
        /// # Arguments
        /// * `origin` - Must be admin
        /// * `ring_id` - ID of the ring that can vote
        /// * `description` - Poll description
        /// * `metadata_hash` - Hash of off-chain metadata
        /// * `deadline` - Block number when voting ends
        /// * `poll_public_key` - Public key for vote encryption
        #[pallet::call_index(1)]
        #[pallet::weight(50_000)]
        pub fn create_poll(
            origin: OriginFor<T>,
            ring_id: RingId,
            description: Vec<u8>,
            metadata: Vec<u8>,
            deadline: BlockNumberFor<T>,
            poll_public_key: CompressedRistrettoWrapper,
        ) -> DispatchResult {
            // Permission check
            T::AdminOrigin::ensure_origin(origin)?;

            // Check if ring exists
            ensure!(Rings::<T>::contains_key(ring_id), Error::<T>::RingNotFound);

            let bounded_description: BoundedVec<u8, T::MaxDescriptionLength> = description
                .try_into()
                .map_err(|_| Error::<T>::DescriptionTooLong)?;

            // Get next poll ID
            let poll_id = PollCount::<T>::get();
            let next_poll_id = poll_id.wrapping_add(1);

            let metadata_hash =
                T::Preimages::note(scale_info::prelude::borrow::Cow::Borrowed(&metadata))?;

            // Create poll
            let poll = Poll::new(
                poll_id,
                ring_id,
                bounded_description,
                metadata_hash,
                deadline,
                poll_public_key,
            )?;

            // Store poll
            Polls::<T>::insert(poll_id, poll);
            PollCount::<T>::put(next_poll_id);

            // Emit event
            Self::deposit_event(Event::PollCreated { poll_id });

            Ok(())
        }

        /// Submit a vote
        ///
        /// # Arguments
        /// * `origin` - Any signed origin
        /// * `poll_id` - ID of the poll to vote on
        /// * `ephemeral_public_key` - Ephemeral public key for encryption
        /// * `ciphertext` - Encrypted vote
        /// * `challenge` - BLSAG challenge
        /// * `responses` - BLSAG responses
        /// * `key_image` - Key image to prevent double voting
        #[pallet::call_index(2)]
        #[pallet::weight(100_000 + (responses.len() as u64) * 5_000)]
        pub fn vote(
            origin: OriginFor<T>,
            poll_id: PollId,
            ephemeral_public_key: CompressedRistrettoWrapper,
            ciphertext: Vec<u8>,
            challenge: ScalarWrapper,
            responses: Vec<ScalarWrapper>,
            key_image: CompressedRistrettoWrapper,
        ) -> DispatchResult {
            ensure_signed(origin)?;

            // Get poll and ensure it's active
            let mut poll = Polls::<T>::get(poll_id).ok_or(Error::<T>::PollNotFound)?;
            let status = poll.get_status();
            ensure!(status == PollStatus::Active, Error::<T>::InvalidPollStatus);

            // Get ring
            let ring = Rings::<T>::get(poll.ring_id).ok_or(Error::<T>::RingNotFound)?;

            // Convert ciphertext to bounded vec
            let bounded_ciphertext: BoundedVec<u8, T::MaxCiphertextLength> = ciphertext
                .clone()
                .try_into()
                .map_err(|_| Error::<T>::CiphertextTooLarge)?;

            // Create encrypted vote
            let encrypted_vote = EncryptedVote {
                ephemeral_public_key,
                ciphertext: bounded_ciphertext,
            };

            // Message to verify
            let message = encrypted_vote.to_bytes();

            // Create BLSAG signature
            let blsag_wrapper = BLSAGWrapper::<T::MaxRingSize> {
                challenge,
                responses: responses.try_into().map_err(|_| Error::<T>::RingTooLarge)?,
                ring: ring.clone(),
                key_image: key_image.clone(),
            };

            // Verify BLSAG signature
            let is_valid = BLSAG::verify::<Blake2s>(blsag_wrapper.into(), &message);
            ensure!(is_valid, Error::<T>::InvalidSignature);

            // Check if key image has been used
            ensure!(
                !UsedKeyImages::<T>::contains_key(&key_image),
                Error::<T>::KeyImageAlreadyUsed
            );

            // Store key image
            UsedKeyImages::<T>::insert(&key_image, ());
            // Store encrypted vote
            let vote_index =
                EncryptedVotes::<T>::try_mutate(poll_id, |votes| -> Result<u32, DispatchError> {
                    votes
                        .try_push(encrypted_vote)
                        .map_err(|_| Error::<T>::Overflow)?;

                    Ok(votes.len() as u32 - 1)
                })?;

            // Emit event
            Self::deposit_event(Event::VoteSubmitted {
                poll_id,
                vote_index,
            });

            Ok(())
        }

        /// Tally the votes
        ///
        /// # Arguments
        /// * `origin` - Must be admin
        /// * `poll_id` - ID of the poll to tally
        /// * `tally` - Tally results
        #[pallet::call_index(3)]
        #[pallet::weight(500_000)]
        pub fn tally(
            origin: OriginFor<T>,
            poll_id: PollId,
            tally: Tally,
            proof: ProofWrapper,
        ) -> DispatchResult {
            // Permission check
            T::AdminOrigin::ensure_origin(origin)?;

            // Get poll and ensure it's in tallying status
            let mut poll = Polls::<T>::get(poll_id).ok_or(Error::<T>::PollNotFound)?;
            let status = poll.get_status();
            ensure!(
                status == PollStatus::Tallying,
                Error::<T>::InvalidPollStatus
            );

            // Compute encrypted votes hash
            let encrypted_votes_hash = Self::compute_encrypted_votes_hash(poll_id);

            // Create public inputs
            let public_inputs = PublicInputs {
                poll_id,
                encrypted_votes_hash,
                tally: tally.clone(),
            };

            // Verify zero-knowledge proof
            let mut vk_bytes = T::TallyVkBytes::get();
            let vk_wrapper = VkWrapper::decode(&mut vk_bytes)
                            .expect("Runtime configuration error: Invalid Tally Verification Key!");
            let vk = vk_wrapper.0;

            let public_inputs: Vec<Fr> = public_inputs.into();
            let seed = hash_proof_and_inputs(&proof.0, &public_inputs);

            let mut rng = ChaCha20Rng::from_seed(seed.0);
            let is_valid =
                MarlinInst::verify(&vk, &public_inputs, &proof.0, &mut rng)
                    .unwrap_or(false);
            ensure!(is_valid, Error::<T>::InvalidProof);

            // Update poll status
            poll.set_status(PollStatus::Completed)?;
            poll.tally = Some(tally.clone());
            Polls::<T>::insert(poll_id, poll);

            // Emit event
            Self::deposit_event(Event::PollCompleted { poll_id, tally });

            Ok(())
        }

        /// Tally a poll
        ///
        /// # Arguments
        /// * `origin` - Must be admin
        /// * `poll_id` - ID of the poll to cancel
        #[pallet::call_index(4)]
        #[pallet::weight(10_000)]
        pub fn tally_poll(origin: OriginFor<T>, poll_id: u32) -> DispatchResult {
            // Permission check
            T::AdminOrigin::ensure_origin(origin)?;

            Polls::<T>::try_mutate(poll_id, |poll_opt| -> DispatchResult {
                let poll = poll_opt.as_mut().ok_or(Error::<T>::PollNotFound)?;
                poll.set_status(PollStatus::Tallying)?;
                Ok(())
            })?;

            // Emit event
            Self::deposit_event(Event::PollTallying { poll_id });

            Ok(())
        }

        /// Cancel a poll
        ///
        /// # Arguments
        /// * `origin` - Must be admin
        /// * `poll_id` - ID of the poll to cancel
        #[pallet::call_index(5)]
        #[pallet::weight(10_000)]
        pub fn cancel_poll(origin: OriginFor<T>, poll_id: u32) -> DispatchResult {
            // Permission check
            T::AdminOrigin::ensure_origin(origin)?;

            Polls::<T>::try_mutate(poll_id, |poll_opt| -> DispatchResult {
                let poll = poll_opt.as_mut().ok_or(Error::<T>::PollNotFound)?;
                poll.set_status(PollStatus::Cancelled)?;
                Ok(())
            })?;

            // Emit event
            Self::deposit_event(Event::PollCancelled { poll_id });

            Ok(())
        }

        /// Pause a poll
        ///
        /// # Arguments
        /// * `origin` - Must be admin
        /// * `poll_id` - ID of the poll to pause
        #[pallet::call_index(6)]
        #[pallet::weight(10_000)]
        pub fn pause_poll(origin: OriginFor<T>, poll_id: u32) -> DispatchResult {
            // Permission check
            T::AdminOrigin::ensure_origin(origin)?;

            Polls::<T>::try_mutate(poll_id, |poll_opt| -> DispatchResult {
                let poll = poll_opt.as_mut().ok_or(Error::<T>::PollNotFound)?;
                poll.set_status(PollStatus::Paused)?;
                Ok(())
            })?;

            // Emit event
            Self::deposit_event(Event::PollPaused { poll_id });

            Ok(())
        }

        /// Active a poll
        ///
        /// # Arguments
        /// * `origin` - Must be admin
        /// * `poll_id` - ID of the poll to pause
        #[pallet::call_index(7)]
        #[pallet::weight(10_000)]
        pub fn active_poll(origin: OriginFor<T>, poll_id: u32) -> DispatchResult {
            // Permission check
            T::AdminOrigin::ensure_origin(origin)?;

            Polls::<T>::try_mutate(poll_id, |poll_opt| -> DispatchResult {
                let poll = poll_opt.as_mut().ok_or(Error::<T>::PollNotFound)?;
                poll.set_status(PollStatus::Active)?;
                Ok(())
            })?;

            // Emit event
            Self::deposit_event(Event::PollActive { poll_id });

            Ok(())
        }

        /// Set deadline for a poll
        ///
        /// # Arguments
        /// * `origin` - Must be admin
        /// * `poll_id` - ID of the poll
        /// * `new_deadline` - New deadline block number
        #[pallet::call_index(8)]
        #[pallet::weight(10_000)]
        pub fn set_deadline(
            origin: OriginFor<T>,
            poll_id: u32,
            new_deadline: BlockNumberFor<T>,
        ) -> DispatchResult {
            // Permission check
            T::AdminOrigin::ensure_origin(origin)?;

            // Set deadline
            Polls::<T>::try_mutate(poll_id, |poll_opt| -> DispatchResult {
                let poll = poll_opt.as_mut().ok_or(Error::<T>::PollNotFound)?;
                poll.set_deadline(new_deadline)?;
                Ok(())
            })?;

            // Emit event
            Self::deposit_event(Event::PollDeadlineUpdated {
                poll_id,
                new_deadline,
            });

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Compute hash of all encrypted votes for a poll
        fn compute_encrypted_votes_hash(poll_id: u32) -> H256 {
            let votes = EncryptedVotes::<T>::get(poll_id);

            let mut hasher = Blake2s::new();
            for vote in votes {
                hasher.update(vote.to_bytes());
            }

            H256(hasher.finalize().into())
        }
    }


    fn hash_proof_and_inputs<P, I>(
        proof: &P,
        public_inputs: &I,
    ) -> H256 
    where
        P: CanonicalSerialize,
        I: CanonicalSerialize,
    {
        let mut hasher = Blake2s::new();
        
        // 增加域分离标签
        hasher.update(b"ZKP_VERIFIER_CHALLENGE_SEED");

        proof.serialize(&mut HashWriter(&mut hasher))
            .expect("Serialization failed");
        
        public_inputs.serialize(&mut HashWriter(&mut hasher))
            .expect("Serialization failed");

        H256(hasher.finalize().into())
    }
}

// 我们需要在区块链上执行签名的验证算法，这是确定性算法，但`nazgul`作为完整的签名库包含了签名及其它算法，
// 这些算法依赖于 `getrandom` 来生成随机数。在区块链环境中，不允许出现外部随机源，因此使用`nazgul`时我们需要
// 为 `getrandom` 提供一个自定义的实现。
// 生产环境中，我们绝不会用到`getrandom`，默认backends实现为空（若调用相关代码，会报错）。
// 但在测试环境中，我们需要使用`nazgul`来生成签名，因此这里提供一个简单的伪随机数生成器 (PRNG) 实现。
#[cfg(all(not(feature = "std"), target_arch = "wasm32"))]
mod wasm_rng_impl {
    use getrandom::{register_custom_getrandom, Error};

    fn custom_getrandom_impl(dest: &mut [u8]) -> Result<(), Error> {
        for i in dest.iter_mut() {
            *i = 0;
        }
        Ok(())
    }

    register_custom_getrandom!(custom_getrandom_impl);
}
