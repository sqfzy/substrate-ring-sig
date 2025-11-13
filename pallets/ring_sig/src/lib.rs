#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;
use weights::WeightInfo;

mod types;
pub use types::*;

mod utils;

#[frame::pallet]
pub mod pallet {
    use super::*;
    use frame::prelude::*;
    use scale_info::prelude::vec::Vec;

    use nazgul::{clsag::CLSAG, traits::Verify};
    use sha2::Sha512;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    // Configuration trait for the pallet.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        // Defines the event type for the pallet.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        // The number of members (rows) in the ring matrix. It means how many members are in the ring.
        #[pallet::constant]
        type NumRingMembers: Get<u32>;

        // The number of columns in the ring matrix. It means how many keys each member has.
        #[pallet::constant]
        type NumRingLayers: Get<u32>;

        /// 提案描述的最大长度
        #[pallet::constant]
        type MaxDescriptionLength: Get<u32>;

        type WeightInfo: WeightInfo;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// 一张匿名选票已成功计入
        VoteTallied {
            proposal_id: ProposalId,
            vote: VoteOption,
            key_image: CompressedRistrettoWrapper,
        },
        /// 一个新提案已创建
        ProposalCreated {
            proposal_id: ProposalId,
            creator: T::AccountId,
        },
        /// 一个提案已关闭
        ProposalClosed { proposal_id: ProposalId },
    }

    /// 提案 ID
    pub type ProposalId = u32;

    /// 投票选项
    #[derive(
        Encode, Decode, TypeInfo, Clone, PartialEq, Eq, Debug, MaxEncodedLen, DecodeWithMemTracking,
    )]
    pub enum VoteOption {
        Yea,
        Nay,
    }

    /// 提案计数器，用于生成新的 ProposalId
    #[pallet::storage]
    #[pallet::getter(fn proposal_count)]
    pub type ProposalCount<T: Config> = StorageValue<_, ProposalId, ValueQuery>;

    /// 存储所有提案的详细信息
    #[pallet::storage]
    #[pallet::getter(fn proposals)]
    pub type Proposals<T: Config> = StorageMap<
        _,
        Twox64Concat,
        ProposalId,
        Proposal<T>,
        OptionQuery, // 提案可能不存在
    >;

    /// 提案的计票结果
    /// Key: ProposalId
    /// Value: (赞成票数, 反对票数)
    #[pallet::storage]
    #[pallet::getter(fn proposal_votes)]
    pub type ProposalVotes<T: Config> = StorageMap<
        _,
        Twox64Concat,
        ProposalId,
        (u32, u32), // (Yea, Nay)
        ValueQuery, // 默认返回 (0, 0)，非常完美
    >;

    /// 存储已使用的密钥镜像 (Key Images)，用于防止双花。
    /// Key: (ProposalId, KeyImage)
    /// Value: ()
    #[pallet::storage]
    #[pallet::getter(fn used_key_images)]
    pub type UsedKeyImages<T: Config> = StorageDoubleMap<
        _,
        Twox64Concat,
        ProposalId,
        Blake2_128,
        CompressedRistrettoWrapper,
        (),
        OptionQuery,
    >;

    #[pallet::error]
    pub enum Error<T> {
        /// 签名验证失败。
        InvalidSignature,
        /// 提供的元数据格式错误。
        BadMetadata,
        /// 投票人已对此提案投过票 (密钥镜像已使用)
        AlreadyVoted,
        /// 提案未找到
        ProposalNotFound,
        /// 提案已关闭，无法投票
        ProposalClosed,
        /// 提案已被关闭
        ProposalAlreadyClosed,
        /// (如果需要权限) 只有提案创建者才能关闭
        NotProposalCreator,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// 创建一个新提案
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::create_proposal())]
        pub fn create_proposal(
            origin: OriginFor<T>,
            description: BoundedVec<u8, T::MaxDescriptionLength>,
        ) -> DispatchResult {
            let creator = ensure_signed(origin)?;

            // 1. 获取新 ID
            let proposal_id = <ProposalCount<T>>::get();

            // 2. 创建提案对象
            let new_proposal = Proposal {
                creator: creator.clone(),
                description,
                status: ProposalStatus::Voting,
            };

            // 3. 存储提案
            <Proposals<T>>::insert(proposal_id, new_proposal);

            // 4. 初始化投票计数
            <ProposalVotes<T>>::insert(proposal_id, (0, 0));

            // 5. 递增 ID 计数器
            <ProposalCount<T>>::put(proposal_id.saturating_add(1));

            // 6. 发送事件
            Self::deposit_event(Event::ProposalCreated {
                proposal_id,
                creator,
            });

            Ok(())
        }

        /// 关闭一个提案
        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::close_proposal())]
        pub fn close_proposal(origin: OriginFor<T>, proposal_id: ProposalId) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // 1. 查找提案
            let mut proposal =
                <Proposals<T>>::get(proposal_id).ok_or(Error::<T>::ProposalNotFound)?;

            // 2. 检查权限：只有创建者能关闭
            ensure!(proposal.creator == who, Error::<T>::NotProposalCreator);

            // 3. 检查状态
            ensure!(
                proposal.status == ProposalStatus::Voting,
                Error::<T>::ProposalAlreadyClosed
            );

            // 4. 更新状态
            proposal.status = ProposalStatus::Closed;

            // 5. 写回存储
            <Proposals<T>>::insert(proposal_id, proposal);

            // 6. 发送事件
            Self::deposit_event(Event::ProposalClosed { proposal_id });

            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::anonymous_vote())]
        pub fn anonymous_vote(
            origin: OriginFor<T>,
            proposal_id: ProposalId,
            vote: VoteOption,
            challenge: H256,
            responses: BoundedVec<H256, T::NumRingMembers>,
            ring: BoundedVec<BoundedVec<H256, T::NumRingLayers>, T::NumRingMembers>,
            key_images: BoundedVec<H256, T::NumRingLayers>,
        ) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            ensure!(
                responses.len() as u32 == T::NumRingMembers::get(),
                Error::<T>::BadMetadata
            );
            ensure!(
                ring.len() as u32 == T::NumRingMembers::get(),
                Error::<T>::BadMetadata
            );
            for row in &ring {
                ensure!(
                    row.len() as u32 == T::NumRingLayers::get(),
                    Error::<T>::BadMetadata
                );
            }
            ensure!(
                key_images.len() as u32 == T::NumRingLayers::get(),
                Error::<T>::BadMetadata
            );

            let proposal = <Proposals<T>>::get(proposal_id).ok_or(Error::<T>::ProposalNotFound)?;
            ensure!(
                proposal.status == ProposalStatus::Voting,
                Error::<T>::ProposalClosed
            );

            let message = {
                let mut msg = proposal_id.encode();
                msg.extend(vote.encode());
                msg
            };

            let challenge = ScalarWrapper(challenge.0);

            let responses: BoundedVec<ScalarWrapper, T::NumRingMembers> = responses
                .into_iter()
                .map(|h| ScalarWrapper(h.0))
                .collect::<Vec<ScalarWrapper>>()
                .try_into()
                .map_err(|_| Error::<T>::BadMetadata)?;

            let ring: BoundedVec<
                BoundedVec<CompressedRistrettoWrapper, T::NumRingLayers>,
                T::NumRingMembers,
            > = ring
                .into_iter()
                .map(|arr| {
                    let wrapped_arr: BoundedVec<CompressedRistrettoWrapper, T::NumRingLayers> = arr
                        .into_iter()
                        .map(|h| CompressedRistrettoWrapper(h.0))
                        .collect::<Vec<CompressedRistrettoWrapper>>()
                        .try_into()
                        .map_err(|_| Error::<T>::BadMetadata)?;
                    Ok(wrapped_arr)
                })
                .collect::<Result<Vec<BoundedVec<CompressedRistrettoWrapper, T::NumRingLayers>>, Error<T>>>()?
                .try_into()
                .map_err(|_| Error::<T>::BadMetadata)?;

            let key_images: BoundedVec<CompressedRistrettoWrapper, T::NumRingLayers> = key_images
                .into_iter()
                .map(|h| CompressedRistrettoWrapper(h.0))
                .collect::<Vec<CompressedRistrettoWrapper>>()
                .try_into()
                .map_err(|_| Error::<T>::BadMetadata)?;
            let main_key_image = key_images[0].clone();

            let signature: CLSAGWrapper<T> = CLSAGWrapper {
                challenge,
                responses,
                ring,
                key_images,
            };

            // --- 4. 验证签名 ---
            let signature = CLSAG::from(signature);
            let is_valid = CLSAG::verify::<Sha512>(signature, &message);
            ensure!(is_valid, Error::<T>::InvalidSignature);

            // --- 5. 检查双重投票 ---
            ensure!(
                !<UsedKeyImages<T>>::contains_key(&proposal_id, &main_key_image),
                Error::<T>::AlreadyVoted
            );
            <UsedKeyImages<T>>::insert(&proposal_id, &main_key_image, ());

            // --- 6. 计票 (Tally) ---
            ProposalVotes::<T>::mutate(proposal_id, |(yea, nay)| match vote {
                VoteOption::Yea => *yea += 1,
                VoteOption::Nay => *nay += 1,
            });

            Self::deposit_event(Event::VoteTallied {
                proposal_id,
                vote,
                key_image: main_key_image,
            });

            Ok(())
        }
    }
}

// 我们需要在区块链上执行签名的验证算法，这是确定性算法，但`nazgul`作为完整的签名库包含了签名及其它算法，
// 这些算法依赖于 `getrandom` 来生成随机数。在区块链环境中，不允许出现外部随机源，因此使用`nazgul`时我们需要
// 为 `getrandom` 提供一个自定义的实现。
// 生产环境中，我们绝不会用到`getrandom`，默认backends实现为空（若调用相关代码，会报错）。
// 但在测试环境中，我们需要使用`nazgul`来生成签名，因此这里提供一个简单的伪随机数生成器 (PRNG) 实现。
#[cfg(any(test, feature = "runtime-benchmarks"))]
mod getrandom_impl {
    use getrandom::Error;

    use core::sync::atomic::{AtomicU64, Ordering};

    // LCG (线性同余生成器) 的参数
    const LCG_A: u64 = 6364136223846793005;
    const LCG_C: u64 = 1442695040888963407;

    /// 用于测试的固定种子
    const INITIAL_SEED: u64 = 0xDEADBEEFCAFEBABEu64;

    static RNG_STATE: AtomicU64 = AtomicU64::new(INITIAL_SEED);

    /// 这是一个用于 **测试** 的确定性、无锁 (lock-free) 伪随机数生成器 (PRNG).
    pub fn getrandom_runtime(dest: &mut [u8]) -> Result<(), Error> {
        for chunk in dest.chunks_mut(8) {
            let update_fn = |state: u64| {
                let new_state = state.wrapping_mul(LCG_A).wrapping_add(LCG_C);
                Some(new_state)
            };

            let old_state = RNG_STATE
                .fetch_update(
                    Ordering::AcqRel,  // 成功时: 获取-释放 语义
                    Ordering::Relaxed, // 失败时: 松散 语义
                    update_fn,
                )
                .expect("PRNG update closure should never fail");

            let new_state = old_state.wrapping_mul(LCG_A).wrapping_add(LCG_C);

            let rand_bytes = new_state.to_ne_bytes();
            let len_to_copy = chunk.len();
            chunk.copy_from_slice(&rand_bytes[..len_to_copy]);
        }

        Ok(())
    }

    // 使用 getrandom 宏来注册我们的自定义实现
    getrandom::register_custom_getrandom!(getrandom_runtime);
}
