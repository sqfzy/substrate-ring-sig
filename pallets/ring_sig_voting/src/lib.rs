#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(any(test, feature = "runtime-benchmarks"))]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;
use weights::WeightInfo;

mod types;
pub use types::*;

#[frame::pallet]
pub mod pallet {
    use super::*;
    use crate::types::BalanceOf;
    use frame::deps::frame_support::traits::{
        EnsureOrigin, Get, QueryPreimage, ReservableCurrency, StorePreimage,
    };
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

        // /// 调度器
        // type Scheduler: ScheduleAnon<
        //     BlockNumberFor<Self>,
        //     RuntimeCallFor<Self>,
        //     <<Self as frame_system::Config>::RuntimeOrigin as OriginTrait>::PalletsOrigin,
        // >;

        /// 货币系统，用于处理押金
        type Currency: ReservableCurrency<Self::AccountId>;

        /// Preimage 存储，用于元数据
        type Preimages: QueryPreimage<H = Self::Hashing> + StorePreimage;

        /// 提交一个投票所需的押金，以防止垃圾信息
        #[pallet::constant]
        type SubmissionDeposit: Get<BalanceOf<Self>>;

        /// 谁有权创建新的投票
        type CreatePollOrigin: EnsureOrigin<Self::RuntimeOrigin, Success = Self::AccountId>;

        /// 谁有权关闭一个投票
        type ClosePollOrigin: EnsureOrigin<Self::RuntimeOrigin>;

        /// 谁有权注册和管理公钥环
        type RingAdminOrigin: EnsureOrigin<Self::RuntimeOrigin, Success = Self::AccountId>;

        /// 提案描述的最大长度
        #[pallet::constant]
        type MaxDescriptionLength: Get<u32>;

        /// 每个环（投票）的最大成员数
        #[pallet::constant]
        type MaxMembersInRing: Get<u32>;

        // The number of columns in the ring matrix. It means how many keys each member has.
        #[pallet::constant]
        type NumRingLayers: Get<u32>;

        type WeightInfo: WeightInfo;
    }

    #[pallet::error]
    pub enum Error<T> {
        /// 签名验证失败。
        InvalidSignature,
        /// 提供的元数据格式错误。
        BadMetadata,
        /// 投票人已对此投票投过票 (密钥镜像已使用)
        AlreadyVoted,
        /// 投票未找到
        PollNotFound,
        /// 投票已关闭
        PollAlreadyClosed,
        /// 投票未开放
        PollNotOpen,
        /// 提供的元数据哈希未在 Preimage pallet 中注册
        PreimageNotExist,
        /// 尝试使用一个不存在的公钥环 ID
        RingGroupNotFound,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// 一张匿名选票已成功计入
        VoteTallied {
            poll_id: PollId,
            vote: VoteOption,
            key_image: CompressedRistrettoWrapper,
        },
        /// 一个新投票已创建
        PollCreated {
            poll_id: PollId,
            ring_id: RingId,
            creator: T::AccountId,
        },
        /// 一个投票已关闭
        PollClosed { poll_id: PollId },
        /// 一个新的公钥环被注册
        RingGroupRegistered {
            ring_id: RingId,
            admin: T::AccountId,
        },
    }

    /// 提案 ID
    pub type PollId = u64;

    /// 可重用公钥环的 ID
    pub type RingId = u64;

    /// 投票选项
    #[derive(
        Encode,
        Decode,
        TypeInfo,
        Clone,
        Copy,
        PartialEq,
        Eq,
        Debug,
        MaxEncodedLen,
        DecodeWithMemTracking,
    )]
    pub enum VoteOption {
        Yea,
        Nay,
    }

    /// 提案计数器，用于生成新的 PollId
    #[pallet::storage]
    #[pallet::getter(fn poll_count)]
    pub type PollCount<T: Config> = StorageValue<_, PollId, ValueQuery>;

    /// 存储所有投票的详细信息
    #[pallet::storage]
    #[pallet::getter(fn polls)]
    pub type Polls<T: Config> = StorageMap<_, Twox64Concat, PollId, Poll<T>, OptionQuery>;

    /// 投票的计票结果
    /// Key: PollId
    /// Value: (赞成票数, 反对票数)
    #[pallet::storage]
    #[pallet::getter(fn poll_votes)]
    pub type PollVotes<T: Config> = StorageMap<
        _,
        Twox64Concat,
        PollId,
        (u32, u32), // (Yea, Nay)
        ValueQuery, // 默认返回 (0, 0)，非常完美
    >;

    /// 存储已使用的密钥镜像 (Key Images)，用于防止双花。
    /// Key: (PollId, KeyImage)
    /// Value: ()
    #[pallet::storage]
    #[pallet::getter(fn used_key_images)]
    pub type UsedKeyImages<T: Config> = StorageDoubleMap<
        _,
        Twox64Concat,
        PollId,
        Blake2_128,
        CompressedRistrettoWrapper,
        (),
        OptionQuery,
    >;

    /// 公钥环计数器
    #[pallet::storage]
    #[pallet::getter(fn ring_group_count)]
    pub type RingGroupCount<T: Config> = StorageValue<_, RingId, ValueQuery>;

    /// 存储可重用的公钥环
    #[pallet::storage]
    #[pallet::getter(fn ring_groups)]
    pub type RingGroups<T: Config> = StorageMap<
        _,
        Twox64Concat,
        RingId,
        RingMatrix<T>, // 存储完整的 2D 公钥矩阵
        OptionQuery,
    >;

    /// 存储每个投票 *所使用* 的公钥环 ID
    #[pallet::storage]
    #[pallet::getter(fn poll_ring_id)]
    pub type PollRingId<T: Config> = StorageMap<_, Twox64Concat, PollId, RingId, OptionQuery>;

    /// 存储投票的元数据哈希
    #[pallet::storage]
    #[pallet::getter(fn poll_metadata)]
    pub type PollMetadata<T: Config> = StorageMap<
        _,
        Twox64Concat,
        PollId,
        T::Hash, // 存储 Preimage 哈希
        OptionQuery,
    >;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// 注册一个可重用的公钥环
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::register_ring_group())]
        pub fn register_ring_group(
            origin: OriginFor<T>,
            ring: BoundedVec<BoundedVec<H256, T::NumRingLayers>, T::MaxMembersInRing>,
        ) -> DispatchResult {
            // 1. 权限检查
            let admin = T::RingAdminOrigin::ensure_origin(origin)?;

            // 2. 获取新 ID
            let ring_id = <RingGroupCount<T>>::get();

            // 3. 存储
            let ring: RingMatrix<T> = ring
                .into_iter()
                .map(|row| {
                    row.into_iter()
                        .map(|h| CompressedRistrettoWrapper(h.0))
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap()
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            <RingGroups<T>>::insert(ring_id, ring);

            // 4. 递增 ID
            <RingGroupCount<T>>::put(ring_id.saturating_add(1));

            // 5. 发送事件
            Self::deposit_event(Event::RingGroupRegistered { ring_id, admin });

            Ok(())
        }

        /// 创建一个新投票
        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::create_poll())]
        pub fn create_poll(
            origin: OriginFor<T>,
            description: BoundedVec<u8, T::MaxDescriptionLength>,
            ring_id: RingId,
            metadata_hash: Option<T::Hash>, // 元数据哈希
        ) -> DispatchResult {
            // 1. 权限检查
            let creator = T::CreatePollOrigin::ensure_origin(origin)?;

            // 2. 验证元数据哈希
            if let Some(hash) = metadata_hash {
                ensure!(
                    T::Preimages::len(&hash).is_some(),
                    Error::<T>::PreimageNotExist
                );
            }

            // 3. 检查 RingId 是否存在
            ensure!(
                <RingGroups<T>>::contains_key(ring_id),
                Error::<T>::RingGroupNotFound
            );

            // 4. 收取押金
            let deposit_amount = T::SubmissionDeposit::get();
            T::Currency::reserve(&creator, deposit_amount)?;
            let submission_deposit = Deposit {
                who: creator.clone(),
                amount: deposit_amount,
            };

            // 5. 获取新 ID
            let poll_id = <PollCount<T>>::get();

            // 6. 创建投票对象
            let new_poll = Poll {
                creator: creator.clone(),
                description,
                status: PollStatus::Voting,
                submission_deposit,
            };

            // 7. 存储
            <Polls<T>>::insert(poll_id, new_poll);
            <PollVotes<T>>::insert(poll_id, (0, 0));
            <PollRingId<T>>::insert(poll_id, ring_id);
            if let Some(hash) = metadata_hash {
                <PollMetadata<T>>::insert(poll_id, hash);
            }

            // 8. 递增 ID 计数器
            <PollCount<T>>::put(poll_id.saturating_add(1));

            // 9. 发送事件
            Self::deposit_event(Event::PollCreated {
                poll_id,
                ring_id,
                creator,
            });

            Ok(())
        }

        /// 关闭一个投票
        #[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::close_poll())]
        pub fn close_poll(origin: OriginFor<T>, poll_id: PollId) -> DispatchResult {
            // 1. 权限检查
            T::ClosePollOrigin::ensure_origin(origin)?;

            // 2. 查找投票
            let mut poll = <Polls<T>>::get(poll_id).ok_or(Error::<T>::PollNotFound)?;

            // 3. 检查状态
            ensure!(
                poll.status == PollStatus::Voting,
                Error::<T>::PollAlreadyClosed
            );

            // 4. 退还押金
            T::Currency::unreserve(&poll.creator, poll.submission_deposit.amount);

            // 5. 更新状态
            poll.status = PollStatus::Closed;
            <Polls<T>>::insert(poll_id, poll);

            // 6. 清理存储
            <PollRingId<T>>::remove(poll_id);
            <PollMetadata<T>>::remove(poll_id);
            // 注意：保留 PollVotes (计票结果) 和 UsedKeyImages (防双花记录)

            // 7. 发送事件
            Self::deposit_event(Event::PollClosed { poll_id });

            Ok(())
        }

        /// 提交匿名投票
        #[pallet::call_index(3)]
        #[pallet::weight(T::WeightInfo::anonymous_vote())]
        pub fn anonymous_vote(
            origin: OriginFor<T>,
            poll_id: PollId,
            vote: VoteOption,
            challenge: H256,
            responses: BoundedVec<H256, T::MaxMembersInRing>,
            key_images: BoundedVec<H256, T::NumRingLayers>,
        ) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            // 1. 检查投票状态
            let poll = <Polls<T>>::get(poll_id).ok_or(Error::<T>::PollNotFound)?;
            ensure!(poll.status == PollStatus::Voting, Error::<T>::PollNotOpen);

            // 2. 从存储中获取权威的公钥环
            let ring_id = <PollRingId<T>>::get(poll_id).ok_or(Error::<T>::PollNotFound)?;
            let ring_matrix = <RingGroups<T>>::get(ring_id).ok_or(Error::<T>::RingGroupNotFound)?;

            // 3. 验证输入长度
            ensure!(
                responses.len() as u32 == ring_matrix.len() as u32,
                Error::<T>::BadMetadata
            );
            ensure!(ring_matrix.len() as u32 > 0, Error::<T>::BadMetadata);
            for row in &ring_matrix {
                ensure!(
                    row.len() as u32 == T::NumRingLayers::get(),
                    Error::<T>::BadMetadata
                );
            }
            ensure!(
                key_images.len() as u32 == T::NumRingLayers::get(),
                Error::<T>::BadMetadata
            );

            // 4. 构建消息
            let message = {
                let mut msg = poll_id.encode();
                msg.extend(vote.encode());
                msg
            };

            // 5. 转换类型 (Responses, Ring, KeyImages)
            let challenge = ScalarWrapper(challenge.0);

            let responses: BoundedVec<ScalarWrapper, T::MaxMembersInRing> = responses
                .into_iter()
                .map(|h| ScalarWrapper(h.0))
                .collect::<Vec<ScalarWrapper>>()
                .try_into()
                .map_err(|_| Error::<T>::BadMetadata)?;

            let ring: BoundedVec<
                BoundedVec<CompressedRistrettoWrapper, T::NumRingLayers>,
                T::MaxMembersInRing,
            > = ring_matrix;

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

            // 6. 验证签名
            let signature = CLSAG::from(signature);
            let is_valid = CLSAG::verify::<Sha512>(signature, &message);
            ensure!(is_valid, Error::<T>::InvalidSignature);

            // 7. 检查双重投票
            ensure!(
                !<UsedKeyImages<T>>::contains_key(&poll_id, &main_key_image),
                Error::<T>::AlreadyVoted
            );
            <UsedKeyImages<T>>::insert(&poll_id, &main_key_image, ());

            // 8. 计票
            PollVotes::<T>::mutate(poll_id, |(yea, nay)| match vote {
                VoteOption::Yea => *yea += 1,
                VoteOption::Nay => *nay += 1,
            });

            Self::deposit_event(Event::VoteTallied {
                poll_id,
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
