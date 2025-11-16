use super::*;
use curve25519_dalek::{
    ristretto::{CompressedRistretto, RistrettoPoint},
    scalar::Scalar,
};
use frame::deps::frame_support::traits::Currency;
use frame::prelude::*;
use nazgul::clsag::CLSAG;

/// 货币余额
pub type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
/// 押金对象
pub type DepositOf<T> = Deposit<<T as frame_system::Config>::AccountId, BalanceOf<T>>;

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

impl From<H256> for CompressedRistrettoWrapper {
    fn from(hash: H256) -> Self {
        CompressedRistrettoWrapper(hash.0)
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

impl From<H256> for ScalarWrapper {
    fn from(hash: H256) -> Self {
        ScalarWrapper(hash.0)
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
    pub responses: BoundedVec<ScalarWrapper, T::MaxMembersInRing>,
    /// These are public keys most of which does not belong to the signer, except one which is the
    /// signer.
    pub ring:
        BoundedVec<BoundedVec<CompressedRistrettoWrapper, T::NumRingLayers>, T::MaxMembersInRing>,
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
    Encode,
    Decode,
    DecodeWithMemTracking,
    Clone,
    PartialEq,
    Eq,
    RuntimeDebug,
    TypeInfo,
    MaxEncodedLen,
)]
pub struct Deposit<AccountId, Balance> {
    pub who: AccountId,
    pub amount: Balance,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Encode,
    Decode,
    TypeInfo,
    PartialEq,
    Eq,
    MaxEncodedLen,
    DecodeWithMemTracking,
)]
pub enum PollStatus {
    /// 正在投票
    Voting,
    /// 已关闭
    Closed,
}

/// 投票（Poll）的详细信息
#[derive(
    Clone, Debug, Encode, Decode, TypeInfo, PartialEq, Eq, MaxEncodedLen, DecodeWithMemTracking,
)]
#[scale_info(skip_type_params(T))]
pub struct Poll<T: Config> {
    /// 创建者
    pub creator: T::AccountId,
    /// 描述
    pub description: BoundedVec<u8, T::MaxDescriptionLength>,
    /// 当前状态
    pub status: PollStatus,
    /// 创建者支付的押金
    pub submission_deposit: DepositOf<T>,
}

/// 用于存储的合格投票者（成员）的公钥环矩阵
pub type RingMatrix<T> = BoundedVec<
    BoundedVec<CompressedRistrettoWrapper, <T as Config>::NumRingLayers>,
    <T as Config>::MaxMembersInRing,
>;

// 简单的赞成/反对投票实现
pub mod simple_voting {
    use super::*;

    // 定义投票类型
    #[derive(
        Encode,
        Decode,
        Clone,
        PartialEq,
        Eq,
        RuntimeDebug,
        TypeInfo,
        MaxEncodedLen,
        DecodeWithMemTracking,
    )]
    pub enum Vote {
        Yea,
        Nay,
    }

    // 定义计票存储类型
    pub type Tally = (u32, u32);

    // 定义计票逻辑
    pub struct TallyHandler;
    impl TallyLogic<Vote, Tally> for TallyHandler {
        fn update_tally(vote: &Vote, tally: &mut Tally) -> DispatchResult {
            match vote {
                Vote::Yea => tally.0 += 1,
                Vote::Nay => tally.1 += 1,
            }
            Ok(())
        }
    }
}

// 投票评分实现
pub mod evaluative_voting {
    use super::*;
    use scale_info::prelude::{vec, vec::Vec};

    const MAX_QUESTIONS: u32 = 10;

    /// 评分等级
    #[derive(
        Encode,
        Decode,
        Clone,
        PartialEq,
        Eq,
        RuntimeDebug,
        TypeInfo,
        MaxEncodedLen,
        DecodeWithMemTracking,
    )]
    pub enum Score {
        One,
        Two,
        Three,
        Four,
        Five,
    }

    pub type Vote = BoundedVec<Score, ConstU32<MAX_QUESTIONS>>;

    /// 一个问题的五个评分等级的计数
    #[derive(
        Encode,
        Decode,
        Clone,
        PartialEq,
        Eq,
        RuntimeDebug,
        TypeInfo,
        MaxEncodedLen,
        DecodeWithMemTracking,
        Default,
    )]
    pub struct QuestionStats {
        score_1: u32,
        score_2: u32,
        score_3: u32,
        score_4: u32,
        score_5: u32,
    }

    pub type Tally = BoundedVec<QuestionStats, ConstU32<MAX_QUESTIONS>>;

    pub struct TallyHandler;
    impl TallyLogic<Vote, Tally> for TallyHandler {
        fn update_tally(vote: &Vote, tally: &mut Tally) -> DispatchResult {
            // 如果这是此 Poll 的第一张票，tally (BoundedVec) 是空的。
            // 我们需要根据 vote 的长度来初始化它。
            if tally.is_empty() {
                let new_stats: Vec<QuestionStats> = vec![QuestionStats::default(); vote.len()];
                
                *tally = BoundedVec::try_from(new_stats).map_err(|_| {
                    DispatchError::Other("Failed to initialize tally: exceeds maximum questions")
                })?;
            }

            ensure!(vote.len() == tally.len(), "Vote and Tally dimensions mismatch");

            for (i, score) in vote.iter().enumerate() {
                let stats = &mut tally
                    .get_mut(i)
                    .expect("Index within bounds due to previous checks");
                match score {
                    Score::One => stats.score_1 += 1,
                    Score::Two => stats.score_2 += 1,
                    Score::Three => stats.score_3 += 1,
                    Score::Four => stats.score_4 += 1,
                    Score::Five => stats.score_5 += 1,
                }
            }
            Ok(())
        }
    }
}
