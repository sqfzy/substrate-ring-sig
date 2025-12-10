use ark_bls12_381::{Bls12_381, Fr};
use ark_ff::Field;
use ark_groth16::{Proof, VerifyingKey};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use codec::{Decode, DecodeWithMemTracking, Encode, EncodeLike, MaxEncodedLen};
use curve25519_dalek::{
    ristretto::{CompressedRistretto, RistrettoPoint},
    scalar::Scalar,
};
use frame::prelude::*;
use frame::traits::schedule::{
    v3::{Named, TaskName},
    DispatchTime,
};
use nazgul::blsag::BLSAG;
use scale_info::prelude::vec::Vec;

pub type PollId = u32;
pub type RingId = u32;
pub type Tally = u32;

/// Wrapper for CompressedRistretto to make it compatible with Substrate storage
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompressedRistrettoWrapper(pub CompressedRistretto);

impl Encode for CompressedRistrettoWrapper {
    fn size_hint(&self) -> usize {
        32
    }

    fn encode_to<T: codec::Output + ?Sized>(&self, dest: &mut T) {
        dest.write(&self.0.to_bytes());
    }
}

impl EncodeLike for CompressedRistrettoWrapper {}

impl Decode for CompressedRistrettoWrapper {
    fn decode<I: codec::Input>(input: &mut I) -> Result<Self, codec::Error> {
        let mut bytes = [0u8; 32];
        input.read(&mut bytes)?;
        Ok(Self(CompressedRistretto(bytes)))
    }
}

impl DecodeWithMemTracking for CompressedRistrettoWrapper {}

impl MaxEncodedLen for CompressedRistrettoWrapper {
    fn max_encoded_len() -> usize {
        32
    }
}

impl scale_info::TypeInfo for CompressedRistrettoWrapper {
    type Identity = Self;

    fn type_info() -> scale_info::Type {
        scale_info::Type::builder()
            .path(scale_info::Path::new(
                "CompressedRistrettoWrapper",
                module_path!(),
            ))
            .composite(
                scale_info::build::Fields::unnamed()
                    .field(|f| f.ty::<[u8; 32]>().type_name("CompressedRistretto")),
            )
    }
}

impl From<H256> for CompressedRistrettoWrapper {
    fn from(h: H256) -> Self {
        Self(CompressedRistretto(h.0))
    }
}

impl From<RistrettoPoint> for CompressedRistrettoWrapper {
    fn from(point: RistrettoPoint) -> Self {
        Self(point.compress())
    }
}

impl From<CompressedRistrettoWrapper> for RistrettoPoint {
    fn from(wrapper: CompressedRistrettoWrapper) -> Self {
        wrapper
            .0
            .decompress()
            .expect("Invalid compressed ristretto point")
    }
}

impl From<CompressedRistretto> for CompressedRistrettoWrapper {
    fn from(compressed: CompressedRistretto) -> Self {
        Self(compressed)
    }
}

impl From<CompressedRistrettoWrapper> for CompressedRistretto {
    fn from(wrapper: CompressedRistrettoWrapper) -> Self {
        wrapper.0
    }
}

impl AsRef<CompressedRistretto> for CompressedRistrettoWrapper {
    fn as_ref(&self) -> &CompressedRistretto {
        &self.0
    }
}

impl core::ops::Deref for CompressedRistrettoWrapper {
    type Target = CompressedRistretto;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Wrapper for Scalar to make it compatible with Substrate storage
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScalarWrapper(pub Scalar);

impl Encode for ScalarWrapper {
    fn size_hint(&self) -> usize {
        32
    }

    fn encode_to<T: codec::Output + ?Sized>(&self, dest: &mut T) {
        dest.write(&self.0.to_bytes());
    }
}

impl EncodeLike for ScalarWrapper {}

impl Decode for ScalarWrapper {
    fn decode<I: codec::Input>(input: &mut I) -> Result<Self, codec::Error> {
        let mut bytes = [0u8; 32];
        input.read(&mut bytes)?;
        let scalar = Scalar::from_canonical_bytes(bytes)
            .into_option()
            .ok_or("Invalid scalar encoding: not canonical")?;
        Ok(Self(scalar))
    }
}

impl DecodeWithMemTracking for ScalarWrapper {}

impl MaxEncodedLen for ScalarWrapper {
    fn max_encoded_len() -> usize {
        32
    }
}

impl scale_info::TypeInfo for ScalarWrapper {
    type Identity = Self;

    fn type_info() -> scale_info::Type {
        scale_info::Type::builder()
            .path(scale_info::Path::new("ScalarWrapper", module_path!()))
            .composite(
                scale_info::build::Fields::unnamed()
                    .field(|f| f.ty::<[u8; 32]>().type_name("Scalar")),
            )
    }
}

impl From<H256> for ScalarWrapper {
    fn from(h: H256) -> Self {
        Self(Scalar::from_bytes_mod_order(h.0))
    }
}

impl From<Scalar> for ScalarWrapper {
    fn from(scalar: Scalar) -> Self {
        Self(scalar)
    }
}

impl From<ScalarWrapper> for Scalar {
    fn from(wrapper: ScalarWrapper) -> Self {
        wrapper.0
    }
}

impl AsRef<Scalar> for ScalarWrapper {
    fn as_ref(&self) -> &Scalar {
        &self.0
    }
}

impl core::ops::Deref for ScalarWrapper {
    type Target = Scalar;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Wrapper for BLSAG signature to make it compatible with Substrate storage
#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(MaxRingSize))]
pub struct BLSAGWrapper<MaxRingSize: Get<u32>> {
    pub challenge: ScalarWrapper,
    pub responses: BoundedVec<ScalarWrapper, MaxRingSize>,
    pub ring: BoundedVec<CompressedRistrettoWrapper, MaxRingSize>,
    pub key_image: CompressedRistrettoWrapper,
}

impl<MaxRingSize: Get<u32>> TryFrom<BLSAG> for BLSAGWrapper<MaxRingSize> {
    type Error = &'static str;

    fn try_from(blsag: BLSAG) -> Result<Self, Self::Error> {
        let challenge = blsag.challenge.into();
        let responses: Vec<ScalarWrapper> = blsag.responses.into_iter().map(Into::into).collect();
        let ring_wrapped: Vec<CompressedRistrettoWrapper> =
            blsag.ring.into_iter().map(Into::into).collect();
        let key_image = blsag.key_image.into();

        Ok(Self {
            challenge,
            responses: BoundedVec::try_from(responses).map_err(|_| "Too many responses")?,
            ring: BoundedVec::try_from(ring_wrapped).map_err(|_| "Ring too large")?,
            key_image,
        })
    }
}

impl<MaxRingSize: Get<u32>> From<BLSAGWrapper<MaxRingSize>> for BLSAG {
    fn from(wrapper: BLSAGWrapper<MaxRingSize>) -> Self {
        let challenge = wrapper.challenge.into();
        let responses: Vec<Scalar> = wrapper.responses.into_iter().map(Into::into).collect();
        let ring = wrapper.ring.into_iter().map(Into::into).collect();
        let key_image = wrapper.key_image.into();

        BLSAG {
            challenge,
            responses,
            key_image,
            ring,
        }
    }
}

/// Poll status enumeration
#[derive(Clone, Copy, Encode, Decode, TypeInfo, PartialEq, Eq, MaxEncodedLen, RuntimeDebug)]
pub enum PollStatus {
    /// Active status, voting is allowed
    Active,
    /// Tallying status, voting is not allowed
    Tallying,
    /// Completed
    Completed,
    /// Paused
    Paused,
    /// Cancelled
    Cancelled,
}

/// Poll structure
#[derive(
    CloneNoBound, DebugNoBound, PartialEqNoBound, EqNoBound, Encode, Decode, TypeInfo, MaxEncodedLen,
)]
#[scale_info(skip_type_params(T))]
pub struct Poll<T: crate::Config> {
    pub poll_id: PollId,
    pub ring_id: RingId,
    pub description: BoundedVec<u8, T::MaxDescriptionLength>,
    pub metadata_hash: <T::Hashing as Hash>::Output,
    pub deadline: BlockNumberFor<T>,
    pub poll_public_key: CompressedRistrettoWrapper,
    pub tally_vk: VkWrapper,
    pub tally: Option<Tally>,
    pub status: PollStatus,
}

impl<T: crate::Config> Poll<T> {
    /// Create a new poll
    pub fn new(
        poll_id: PollId,
        ring_id: RingId,
        description: BoundedVec<u8, T::MaxDescriptionLength>,
        metadata_hash: <T::Hashing as Hash>::Output,
        deadline: BlockNumberFor<T>,
        poll_public_key: CompressedRistrettoWrapper,
        tally_vk: VkWrapper,
    ) -> Result<Self, DispatchError> {
        // Check if deadline is valid
        let now = frame_system::Pallet::<T>::block_number();
        if deadline <= now {
            return Err(crate::Error::<T>::InvalidDeadline.into());
        }

        schedule_deadline_task::<T>(poll_id, deadline)?;

        Ok(Self {
            poll_id,
            ring_id,
            description,
            metadata_hash,
            deadline,
            poll_public_key,
            tally_vk,
            tally: None,
            status: PollStatus::Active,
        })
    }

    /// Get poll status with automatic state transition
    pub fn get_status(&mut self) -> PollStatus {
        self.status
    }

    pub fn set_status(&mut self, new_status: PollStatus) -> DispatchResult {
        let now = frame_system::Pallet::<T>::block_number();
        let old_status = self.status;

        // 验证状态转换
        if new_status == PollStatus::Active && now > self.deadline {
            return Err(crate::Error::<T>::InvalidDeadline.into());
        }

        let valid_transition = match (&old_status, &new_status) {
            (PollStatus::Active, PollStatus::Tallying)
            | (PollStatus::Active, PollStatus::Paused)
            | (PollStatus::Active, PollStatus::Cancelled)
            | (PollStatus::Tallying, PollStatus::Completed)
            | (PollStatus::Tallying, PollStatus::Paused)
            | (PollStatus::Paused, PollStatus::Active)
            | (PollStatus::Paused, PollStatus::Tallying)
            | (PollStatus::Paused, PollStatus::Cancelled) => true,
            _ => false,
        };

        if !valid_transition {
            return Err(crate::Error::<T>::InvalidStatusTransition.into());
        }

        self.status = new_status;

        // 如果设置为 Active，尝试调度 deadline 任务
        if new_status == PollStatus::Active {
            schedule_deadline_task::<T>(self.poll_id, self.deadline)?;
        }

        // 如果从 Active 切换到其他状态，取消任务
        if old_status == PollStatus::Active && new_status != PollStatus::Active {
            cancel_deadline_task::<T>(self.poll_id).ok();
        }

        Ok(())
    }

    /// Set deadline
    pub fn set_deadline(&mut self, deadline: BlockNumberFor<T>) -> DispatchResult {
        if self.status != PollStatus::Active {
            return Err(crate::Error::<T>::InvalidPollStatus.into());
        }

        let now = frame_system::Pallet::<T>::block_number();
        if deadline <= now {
            return Err(crate::Error::<T>::InvalidDeadline.into());
        }

        self.deadline = deadline;
        schedule_deadline_task::<T>(self.poll_id, deadline)?;
        Ok(())
    }
}

fn schedule_deadline_task<T: crate::Config>(
    poll_id: PollId,
    deadline: BlockNumberFor<T>,
) -> DispatchResult {
    let task_name = task_name(poll_id);

    // 先尝试取消可能存在的旧任务
    T::Scheduler::cancel_named(task_name).ok();

    // 构造调用
    let call: <T as crate::Config>::RuntimeCall = crate::Call::<T>::tally_poll { poll_id }.into();
    let bounded_call = BoundedVec::try_from(call.encode()).expect("Call must be at most 128 bytes");

    // 调度任务
    T::Scheduler::schedule_named(
        task_name,
        DispatchTime::At(deadline),
        None, // no periodic execution
        0,    // priority
        frame_system::RawOrigin::Root.into(),
        frame::deps::frame_support::traits::Bounded::Inline(bounded_call),
    )?;

    Ok(())
}

fn cancel_deadline_task<T: crate::Config>(poll_id: PollId) -> DispatchResult {
    let task_name = task_name(poll_id);
    T::Scheduler::cancel_named(task_name)
}

/// b"poll_deadline_{poll_id}"
fn task_name(poll_id: u32) -> TaskName {
    let mut name = [
        b'p', b'o', b'l', b'l', b'_', b'd', b'e', b'a', b'd', b'l', b'i', b'n', b'e', b'_', 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    name[14..18].copy_from_slice(&poll_id.to_be_bytes());
    TaskName::from(name)
}

/// Wrapper for Groth16 proof
#[derive(Clone, Debug, PartialEq)]
pub struct ProofWrapper(pub Proof<Bls12_381>);

impl Encode for ProofWrapper {
    fn size_hint(&self) -> usize {
        // Approximation: most proofs are around 192 bytes
        200
    }

    fn encode_to<T: codec::Output + ?Sized>(&self, dest: &mut T) {
        let mut bytes = Vec::with_capacity(self.size_hint());
        self.0
            .serialize_compressed(&mut bytes)
            .expect("Proof serialization should not fail");
        bytes.encode_to(dest);
    }
}

impl EncodeLike for ProofWrapper {}

impl Decode for ProofWrapper {
    fn decode<I: codec::Input>(input: &mut I) -> Result<Self, codec::Error> {
        let bytes: Vec<u8> = Vec::decode(input)?;
        let proof = Proof::<Bls12_381>::deserialize_compressed(&bytes[..])
            .map_err(|_| codec::Error::from("Failed to deserialize proof"))?;
        Ok(Self(proof))
    }
}

impl DecodeWithMemTracking for ProofWrapper {}

impl MaxEncodedLen for ProofWrapper {
    fn max_encoded_len() -> usize {
        // Vec compact length + actual proof size
        codec::Compact(256u32).encoded_size() + 256
    }
}

impl scale_info::TypeInfo for ProofWrapper {
    type Identity = Self;

    fn type_info() -> scale_info::Type {
        scale_info::Type::builder()
            .path(scale_info::Path::new("ProofWrapper", module_path!()))
            .composite(
                scale_info::build::Fields::unnamed()
                    .field(|f| f.ty::<Vec<u8>>().type_name("Proof<Bls12_381>")),
            )
    }
}

impl From<Proof<Bls12_381>> for ProofWrapper {
    fn from(proof: Proof<Bls12_381>) -> Self {
        Self(proof)
    }
}

impl From<ProofWrapper> for Proof<Bls12_381> {
    fn from(wrapper: ProofWrapper) -> Self {
        wrapper.0
    }
}

impl AsRef<Proof<Bls12_381>> for ProofWrapper {
    fn as_ref(&self) -> &Proof<Bls12_381> {
        &self.0
    }
}

impl core::ops::Deref for ProofWrapper {
    type Target = Proof<Bls12_381>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Wrapper for Groth16 verifying key
#[derive(Clone, Debug, PartialEq)]
pub struct VkWrapper(pub VerifyingKey<Bls12_381>);

impl Encode for VkWrapper {
    fn size_hint(&self) -> usize {
        // VKs are variable but typically 1-2KB
        1024
    }

    fn encode_to<T: codec::Output + ?Sized>(&self, dest: &mut T) {
        let mut bytes = Vec::new();
        self.0
            .serialize_compressed(&mut bytes)
            .expect("VK serialization should not fail");
        bytes.encode_to(dest);
    }
}

impl EncodeLike for VkWrapper {}

impl Decode for VkWrapper {
    fn decode<I: codec::Input>(input: &mut I) -> Result<Self, codec::Error> {
        let bytes: Vec<u8> = Vec::decode(input)?;
        let vk = VerifyingKey::<Bls12_381>::deserialize_compressed(&bytes[..])
            .map_err(|_| codec::Error::from("Failed to deserialize verification key"))?;
        Ok(Self(vk))
    }
}

impl DecodeWithMemTracking for VkWrapper {}

impl MaxEncodedLen for VkWrapper {
    fn max_encoded_len() -> usize {
        // Vec compact length + max VK size
        codec::Compact(2048u32).encoded_size() + 2048
    }
}

impl scale_info::TypeInfo for VkWrapper {
    type Identity = Self;

    fn type_info() -> scale_info::Type {
        scale_info::Type::builder()
            .path(scale_info::Path::new("VkWrapper", module_path!()))
            .composite(
                scale_info::build::Fields::unnamed()
                    .field(|f| f.ty::<Vec<u8>>().type_name("VerifyingKey<Bls12_381>")),
            )
    }
}

impl From<VerifyingKey<Bls12_381>> for VkWrapper {
    fn from(vk: VerifyingKey<Bls12_381>) -> Self {
        Self(vk)
    }
}

impl From<VkWrapper> for VerifyingKey<Bls12_381> {
    fn from(wrapper: VkWrapper) -> Self {
        wrapper.0
    }
}

impl AsRef<VerifyingKey<Bls12_381>> for VkWrapper {
    fn as_ref(&self) -> &VerifyingKey<Bls12_381> {
        &self.0
    }
}

impl core::ops::Deref for VkWrapper {
    type Target = VerifyingKey<Bls12_381>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Public inputs for zero-knowledge proof
#[derive(Clone, Encode, Decode, TypeInfo, PartialEq, Eq, MaxEncodedLen, RuntimeDebug)]
pub struct PublicInputs {
    pub poll_id: u32,
    /// Hash of all encrypted votes (commitment)
    pub encrypted_votes_hash: H256,
    /// Tally result
    pub tally: Tally,
}

impl From<PublicInputs> for Vec<Fr> {
    fn from(inputs: PublicInputs) -> Vec<Fr> {
        let mut elements = Vec::new();

        // Add poll_id
        elements.push(Fr::from(inputs.poll_id));

        // Add encrypted_votes_hash (split into field elements)
        let hash_bytes = inputs.encrypted_votes_hash.0;
        for chunk in hash_bytes.chunks(31) {
            let mut bytes = [0u8; 32];
            bytes[..chunk.len()].copy_from_slice(chunk);
            if let Some(fe) = Fr::from_random_bytes(&bytes) {
                elements.push(fe);
            }
        }

        elements.push(Fr::from(inputs.tally));

        elements
    }
}

/// Encrypted vote structure
#[derive(Clone, Encode, Decode, TypeInfo, PartialEq, Eq, MaxEncodedLen, RuntimeDebug)]
#[scale_info(skip_type_params(MaxCiphertextLength))]
pub struct EncryptedVote<MaxCiphertextLength: Get<u32>> {
    pub ephemeral_public_key: CompressedRistrettoWrapper,
    pub ciphertext: BoundedVec<u8, MaxCiphertextLength>,
}

impl<MaxCiphertextLength: Get<u32>> EncryptedVote<MaxCiphertextLength> {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(32 + self.ciphertext.len());
        bytes.extend_from_slice(&self.ephemeral_public_key.to_bytes());
        bytes.extend_from_slice(&self.ciphertext);
        bytes
    }
}
