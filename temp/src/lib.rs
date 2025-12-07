#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod types;

pub use types::*;
pub use pallet::*;

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

    // Configuration trait for the pallet.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        // Defines the event type for the pallet.
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

}
