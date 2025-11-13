use super::*;
use curve25519_dalek::{
    ristretto::{CompressedRistretto, RistrettoPoint},
    scalar::Scalar,
};
use frame::prelude::*;
use nazgul::clsag::CLSAG;

// RistrettoPoint (公钥) 包装器
// RistrettoPoint 压缩后是 32 字节
#[derive(
    Clone, Debug, Encode, Decode, TypeInfo, PartialEq, Eq, MaxEncodedLen, DecodeWithMemTracking,
)]
pub struct CompressedRistrettoWrapper(pub [u8; 32]);

impl From<RistrettoPoint> for CompressedRistrettoWrapper {
    fn from(point: RistrettoPoint) -> Self {
        CompressedRistrettoWrapper(point.compress().to_bytes())
    }
}

impl From<CompressedRistrettoWrapper> for RistrettoPoint {
    fn from(key: CompressedRistrettoWrapper) -> Self {
        CompressedRistretto(key.0)
            .decompress()
            .expect("Invalid RistrettoPoint bytes")
    }
}

#[derive(
    Clone, Debug, Encode, Decode, TypeInfo, PartialEq, Eq, MaxEncodedLen, DecodeWithMemTracking,
)]
pub struct ScalarWrapper(pub [u8; 32]);

impl From<Scalar> for ScalarWrapper {
    fn from(scalar: Scalar) -> Self {
        ScalarWrapper(scalar.to_bytes())
    }
}

impl From<ScalarWrapper> for Scalar {
    fn from(wrapper: ScalarWrapper) -> Self {
        Scalar::from_canonical_bytes(wrapper.0).expect("Invalid Scalar bytes")
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo, DecodeWithMemTracking, MaxEncodedLen,
)]
#[scale_info(skip_type_params(T))]
pub struct CLSAGWrapper<T: Config> {
    /// This is the challenge generated non-interactievely
    pub challenge: ScalarWrapper,
    /// These responses are mostly fake, except one which is real.
    pub responses: BoundedVec<ScalarWrapper, T::NumRingMembers>,
    /// These are public keys most of which does not belong to the signer, except one which is the
    /// signer.
    pub ring:
        BoundedVec<BoundedVec<CompressedRistrettoWrapper, T::NumRingLayers>, T::NumRingMembers>,
    /// These are key images. Only the first one is linkable. If the keypair corresponding to the
    /// first key-image is ever used everyone will know.
    pub key_images: BoundedVec<CompressedRistrettoWrapper, T::NumRingLayers>,
}

impl<T: Config> From<CLSAGWrapper<T>> for CLSAG {
    fn from(wrapper: CLSAGWrapper<T>) -> Self {
        CLSAG {
            challenge: wrapper.challenge.into(),
            responses: wrapper.responses.into_iter().map(|r| r.into()).collect(),
            ring: wrapper
                .ring
                .into_iter()
                .map(|row| row.into_iter().map(|k| k.into()).collect())
                .collect(),
            key_images: wrapper.key_images.into_iter().map(|ki| ki.into()).collect(),
        }
    }
}

#[derive(
    Clone, Debug, Encode, Decode, TypeInfo, PartialEq, Eq, MaxEncodedLen, DecodeWithMemTracking,
)]
pub enum ProposalStatus {
    /// 正在投票
    Voting,
    /// 已关闭
    Closed,
}

#[derive(
    Clone, Debug, Encode, Decode, TypeInfo, PartialEq, Eq, MaxEncodedLen, DecodeWithMemTracking,
)]
#[scale_info(skip_type_params(T))]
pub struct Proposal<T: Config> {
    /// 创建者
    pub creator: T::AccountId,
    /// 描述
    pub description: BoundedVec<u8, T::MaxDescriptionLength>,
    /// 当前状态
    pub status: ProposalStatus,
}
