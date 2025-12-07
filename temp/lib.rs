#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub mod types;
pub use types::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame::pallet]
pub mod pallet {
    use super::*;
    use frame::prelude::*;
    use scale_info::prelude::vec::Vec;

    use curve25519_dalek::ristretto::CompressedRistretto;
    use nazgul::blsag::BLSAG;
    use ark_groth16::Groth16;
    use ark_bls12_381::Bls12_381;
    use ark_serialize::CanonicalDeserialize;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Configuration trait
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

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

        /// Admin origin for privileged operations
        type AdminOrigin: EnsureOrigin<Self::RuntimeOrigin>;
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
    pub type Polls<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        u32,
        Poll<T>,
        OptionQuery,
    >;

    /// Storage for encrypted votes (poll_id -> vote_index -> encrypted_vote)
    #[pallet::storage]
    #[pallet::getter(fn encrypted_votes)]
    pub type EncryptedVotes<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        u32,
        Blake2_128Concat,
        u32,
        EncryptedVote<T::MaxCiphertextLength>,
        OptionQuery,
    >;

    /// Storage for used key images to prevent double voting
    #[pallet::storage]
    #[pallet::getter(fn used_key_images)]
    pub type UsedKeyImages<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        CompressedRistrettoWrapper,
        (),
        OptionQuery,
    >;

    /// Storage for tally results
    #[pallet::storage]
    #[pallet::getter(fn tallies)]
    pub type Tallies<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        u32,
        BoundedVec<u32, ConstU32<64>>,
        OptionQuery,
    >;

    /// Poll counter
    #[pallet::storage]
    #[pallet::getter(fn poll_count)]
    pub type PollCount<T: Config> = StorageValue<_, u32, ValueQuery>;

    /// Ring counter
    #[pallet::storage]
    #[pallet::getter(fn ring_count)]
    pub type RingCount<T: Config> = StorageValue<_, u32, ValueQuery>;

    /// Vote counter per poll
    #[pallet::storage]
    #[pallet::getter(fn vote_counts)]
    pub type VoteCounts<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        u32,
        u32,
        ValueQuery,
    >;

    /// Events
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A ring group has been registered
        RingRegistered { ring_id: u32 },
        /// A poll has been created
        PollCreated { poll_id: u32, creator: T::AccountId },
        /// A vote has been submitted
        VoteSubmitted { poll_id: u32 },
        /// A poll has been tallied
        PollTallied { poll_id: u32, tally: BoundedVec<u32, ConstU32<64>> },
        /// A poll has been cancelled
        PollCancelled { poll_id: u32 },
        /// A poll has been paused
        PollPaused { poll_id: u32 },
        /// A poll has been resumed
        PollResumed { poll_id: u32 },
        /// A poll deadline has been updated
        PollDeadlineUpdated { poll_id: u32, new_deadline: BlockNumberFor<T> },
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
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Register a ring signature group
        ///
        /// # Arguments
        /// * `origin` - Must be admin
        /// * `ring` - Vector of public keys forming the ring
        ///
        /// # Weight: O(ring_size)
        #[pallet::call_index(0)]
        #[pallet::weight(10_000 + (ring.len() as u64) * 1_000)]
        pub fn register_ring_group(
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
            let next_ring_id = ring_id.checked_add(1).ok_or(Error::<T>::Overflow)?;

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
        /// * `tally_public_key` - Public key for vote encryption
        /// * `tally_vk` - Groth16 verification key
        ///
        /// # Weight: O(1)
        #[pallet::call_index(1)]
        #[pallet::weight(50_000)]
        pub fn create_poll(
            origin: OriginFor<T>,
            ring_id: u32,
            description: Vec<u8>,
            metadata_hash: H256,
            deadline: BlockNumberFor<T>,
            tally_public_key: CompressedRistrettoWrapper,
            tally_vk: Vec<u8>,
        ) -> DispatchResult {
            // Permission check
            T::AdminOrigin::ensure_origin(origin. clone())?;
            let creator = ensure_signed(origin)?;

            // Check if ring exists
            ensure!(Rings::<T>::contains_key(ring_id), Error::<T>::RingNotFound);

            // Convert to bounded vecs
            let bounded_description: BoundedVec<u8, T::MaxDescriptionLength> =
                description.try_into().map_err(|_| Error::<T>::DescriptionTooLong)?;
            let bounded_vk: BoundedVec<u8, T::MaxVkLength> =
                tally_vk.try_into().map_err(|_| Error::<T>::VkTooLarge)?;

            // Create poll
            let poll = Poll::new(
                creator. clone(),
                ring_id,
                bounded_description,
                metadata_hash,
                deadline,
                tally_public_key,
                bounded_vk,
            )
            .map_err(|_| Error::<T>::InvalidDeadline)?;

            // Get next poll ID
            let poll_id = PollCount::<T>::get();
            let next_poll_id = poll_id. checked_add(1).ok_or(Error::<T>::Overflow)?;

            // Store poll
            Polls::<T>::insert(poll_id, poll);
            PollCount::<T>::put(next_poll_id);

            // Emit event
            Self::deposit_event(Event::PollCreated { poll_id, creator });

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
        ///
        /// # Weight: O(ring_size)
        #[pallet::call_index(2)]
        #[pallet::weight(100_000 + (responses.len() as u64) * 5_000)]
        pub fn vote(
            origin: OriginFor<T>,
            poll_id: u32,
            ephemeral_public_key: CompressedRistrettoWrapper,
            ciphertext: Vec<u8>,
            challenge: ScalarWrapper,
            responses: Vec<ScalarWrapper>,
            key_image: CompressedRistrettoWrapper,
        ) -> DispatchResult {
            ensure_signed(origin)?;

            // Get poll and ensure it's active
            let mut poll = Polls::<T>::get(poll_id).ok_or(Error::<T>::PollNotFound)?;
            let status = poll.get_status(poll_id);
            ensure!(status == PollStatus::Active, Error::<T>::InvalidPollStatus);

            // Get ring
            let ring = Rings::<T>::get(poll.ring_id).ok_or(Error::<T>::RingNotFound)? ;

            // Convert ciphertext to bounded vec
            let bounded_ciphertext: BoundedVec<u8, T::MaxCiphertextLength> =
                ciphertext.clone().try_into().map_err(|_| Error::<T>::CiphertextTooLarge)?;

            // Create encrypted vote
            let encrypted_vote = EncryptedVote {
                ephemeral_public_key: ephemeral_public_key. clone(),
                ciphertext: bounded_ciphertext,
            };

            // Get message to verify
            let message = encrypted_vote.get_message();

            // Create BLSAG signature
            let blsag_wrapper = BLSAGWrapper::<T::MaxRingSize> {
                challenge,
                responses: responses.try_into().map_err(|_| Error::<T>::RingTooLarge)?,
                ring: ring.clone(),
                key_image: key_image.clone(),
            };
            let blsag = blsag_wrapper.into();

            // Verify BLSAG signature
            let is_valid = BLSAG::verify(blsag, &message);
            ensure!(is_valid, Error::<T>::InvalidSignature);

            // Check if key image has been used
            ensure!(
                ! UsedKeyImages::<T>::contains_key(&key_image),
                Error::<T>::KeyImageAlreadyUsed
            );

            // Store key image
            UsedKeyImages::<T>::insert(&key_image, ());

            // Store encrypted vote
            let vote_index = VoteCounts::<T>::get(poll_id);
            EncryptedVotes::<T>::insert(poll_id, vote_index, encrypted_vote);
            VoteCounts::<T>::insert(poll_id, vote_index + 1);

            // Emit event
            Self::deposit_event(Event::VoteSubmitted { poll_id });

            Ok(())
        }

        /// Tally the votes
        ///
        /// # Arguments
        /// * `origin` - Must be admin
        /// * `poll_id` - ID of the poll to tally
        /// * `tally` - Tally results
        /// * `zk_proof` - Zero-knowledge proof of correct tallying
        ///
        /// # Weight: O(proof_verification)
        #[pallet::call_index(3)]
        #[pallet::weight(500_000)]
        pub fn tally(
            origin: OriginFor<T>,
            poll_id: u32,
            tally: Vec<u32>,
            zk_proof: Vec<u8>,
        ) -> DispatchResult {
            // Permission check
            T::AdminOrigin::ensure_origin(origin)? ;

            // Get poll and ensure it's in tallying status
            let mut poll = Polls::<T>::get(poll_id).ok_or(Error::<T>::PollNotFound)? ;
            let status = poll. get_status(poll_id);
            ensure!(status == PollStatus::Tallying, Error::<T>::InvalidPollStatus);

            // Convert tally to bounded vec
            let bounded_tally: BoundedVec<u32, ConstU32<64>> =
                tally.clone().try_into().map_err(|_| Error::<T>::InvalidProof)?;

            // Compute encrypted votes hash
            let encrypted_votes_hash = Self::compute_encrypted_votes_hash(poll_id);

            // Create public inputs
            let public_inputs = PublicInputs {
                poll_id,
                encrypted_votes_hash,
                tally: bounded_tally.clone(),
            };

            // Verify zero-knowledge proof
            let vk_wrapper = VkWrapper(poll.tally_vk);
            let vk = ark_groth16::VerifyingKey::<Bls12_381>::try_from(vk_wrapper)
                .map_err(|_| Error::<T>::InvalidProof)?;

            Self::verify_groth16_proof(&poll.tally_vk, &public_inputs, &zk_proof)?;

            // Update poll status
            poll.set_status(PollStatus::Completed)
                .map_err(|_| Error::<T>::InvalidStatusTransition)?;
            Polls::<T>::insert(poll_id, poll);

            // Store tally
            Tallies::<T>::insert(poll_id, bounded_tally. clone());

            // Emit event
            Self::deposit_event(Event::PollTallied { poll_id, tally: bounded_tally });

            Ok(())
        }

        /// Tally a poll
        ///
        /// # Arguments
        /// * `origin` - Must be admin
        /// * `poll_id` - ID of the poll to cancel
        ///
        /// # Weight: O(1)
        #[pallet::call_index(4)]
        #[pallet::weight(10_000)]
        pub fn tally_poll(origin: OriginFor<T>, poll_id: u32) -> DispatchResult {
            // Permission check
            T::AdminOrigin::ensure_origin(origin)? ;

            // Get poll
            let mut poll = Polls::<T>::get(poll_id).ok_or(Error::<T>::PollNotFound)?;

            // Set status to cancelled
            poll.set_status(PollStatus::Tallying)
                .map_err(|_| Error::<T>::InvalidStatusTransition)?;
            Polls::<T>::insert(poll_id, poll);

            // Emit event
            Self::deposit_event(Event::PollCancelled { poll_id });

            Ok(())
        }

        /// Cancel a poll
        ///
        /// # Arguments
        /// * `origin` - Must be admin
        /// * `poll_id` - ID of the poll to cancel
        ///
        /// # Weight: O(1)
        #[pallet::call_index(5)]
        #[pallet::weight(10_000)]
        pub fn cancel_poll(origin: OriginFor<T>, poll_id: u32) -> DispatchResult {
            // Permission check
            T::AdminOrigin::ensure_origin(origin)? ;

            // Get poll
            let mut poll = Polls::<T>::get(poll_id).ok_or(Error::<T>::PollNotFound)?;

            // Set status to cancelled
            poll.set_status(PollStatus::Cancelled)
                .map_err(|_| Error::<T>::InvalidStatusTransition)?;
            Polls::<T>::insert(poll_id, poll);

            // Emit event
            Self::deposit_event(Event::PollCancelled { poll_id });

            Ok(())
        }

        /// Pause a poll
        ///
        /// # Arguments
        /// * `origin` - Must be admin
        /// * `poll_id` - ID of the poll to pause
        ///
        /// # Weight: O(1)
        #[pallet::call_index(6)]
        #[pallet::weight(10_000)]
        pub fn pause_poll(origin: OriginFor<T>, poll_id: u32) -> DispatchResult {
            // Permission check
            T::AdminOrigin::ensure_origin(origin)?;

            // Get poll
            let mut poll = Polls::<T>::get(poll_id).ok_or(Error::<T>::PollNotFound)?;

            // Set status to paused
            poll.set_status(PollStatus::Paused)
                .map_err(|_| Error::<T>::InvalidStatusTransition)?;
            Polls::<T>::insert(poll_id, poll);

            // Emit event
            Self::deposit_event(Event::PollPaused { poll_id });

            Ok(())
        }

        /// Resume a poll
        ///
        /// # Arguments
        /// * `origin` - Must be admin
        /// * `poll_id` - ID of the poll to resume
        ///
        /// # Weight: O(1)
        #[pallet::call_index(7)]
        #[pallet::weight(10_000)]
        pub fn resume_poll(origin: OriginFor<T>, poll_id: u32) -> DispatchResult {
            // Permission check
            T::AdminOrigin::ensure_origin(origin)? ;

            // Get poll
            let mut poll = Polls::<T>::get(poll_id). ok_or(Error::<T>::PollNotFound)?;

            // Determine which status to resume to
            let now = frame_system::Pallet::<T>::block_number();
            let new_status = if now <= poll.deadline {
                PollStatus::Active
            } else {
                PollStatus::Tallying
            };

            // Set status
            poll.set_status(new_status)
                .map_err(|_| Error::<T>::InvalidStatusTransition)?;
            Polls::<T>::insert(poll_id, poll);

            // Emit event
            Self::deposit_event(Event::PollResumed { poll_id });

            Ok(())
        }

        /// Set deadline for a poll
        ///
        /// # Arguments
        /// * `origin` - Must be admin
        /// * `poll_id` - ID of the poll
        /// * `new_deadline` - New deadline block number
        ///
        /// # Weight: O(1)
        #[pallet::call_index(8)]
        #[pallet::weight(10_000)]
        pub fn set_deadline(
            origin: OriginFor<T>,
            poll_id: u32,
            new_deadline: BlockNumberFor<T>,
        ) -> DispatchResult {
            // Permission check
            T::AdminOrigin::ensure_origin(origin)?;

            // Get poll
            let mut poll = Polls::<T>::get(poll_id).ok_or(Error::<T>::PollNotFound)?;

            // Set deadline
            poll.set_deadline(new_deadline)
                . map_err(|_| Error::<T>::InvalidDeadline)?;
            Polls::<T>::insert(poll_id, poll);

            // Emit event
            Self::deposit_event(Event::PollDeadlineUpdated { poll_id, new_deadline });

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Update poll status in storage (called from Poll::get_status)
        pub fn update_poll_status(poll_id: u32, poll: Poll<T>) {
            Polls::<T>::insert(poll_id, poll);
        }

        /// Compute hash of all encrypted votes for a poll
        fn compute_encrypted_votes_hash(poll_id: u32) -> H256 {
            let mut all_votes = Vec::new();
            let vote_count = VoteCounts::<T>::get(poll_id);

            for i in 0..vote_count {
                if let Some(vote) = EncryptedVotes::<T>::get(poll_id, i) {
                    all_votes.extend_from_slice(&vote.ephemeral_public_key. 0);
                    all_votes.extend_from_slice(&vote.ciphertext);
                }
            }

            T::Hashing::hash(&all_votes)
        }

        /// Verify Groth16 proof
        fn verify_groth16_proof(
            vk_bytes: &BoundedVec<u8, T::MaxVkLength>,
            public_inputs: &PublicInputs,
            proof_bytes: &[u8],
        ) -> DispatchResult {
            // Deserialize verification key
            let vk = ark_groth16::VerifyingKey::<Bls12_381>::deserialize_compressed(&vk_bytes[..])
                .map_err(|_| Error::<T>::InvalidProof)?;

            // Deserialize proof
            let proof = ark_groth16::Proof::<Bls12_381>::deserialize_compressed(proof_bytes)
                .map_err(|_| Error::<T>::InvalidProof)?;

            // Convert public inputs to field elements
            let public_input_fields = public_inputs.to_field_elements();

            // Verify proof
            use ark_groth16::Groth16;
            use ark_snark::SNARK;
            
            let is_valid = Groth16::<Bls12_381>::verify(&vk, &public_input_fields, &proof)
                .map_err(|_| Error::<T>::InvalidProof)?;

            ensure!(is_valid, Error::<T>::InvalidProof);

            Ok(())
        }
    }
}
