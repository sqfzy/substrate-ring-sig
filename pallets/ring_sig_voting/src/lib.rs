#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(any(test, feature = "runtime-benchmarks"))]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod types;
pub mod weights;

pub use pallet::*;
pub use types::*;

#[frame::pallet]
pub mod pallet {
    use super::*;
    use frame::prelude::*;
    use frame::traits::{schedule, QueryPreimage, StorePreimage, ValidateUnsigned};
    use scale_info::prelude::vec::Vec;
    use weights::WeightInfo;

    use curve25519_dalek::ristretto::RistrettoPoint;
    use nazgul::blsag::BLSAG;
    use nazgul::traits::Verify; // 用于私钥验证

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

        /// Maximum ciphertext length
        #[pallet::constant]
        type MaxCiphertextLength: Get<u32>;

        /// Maximum number of votes per poll
        #[pallet::constant]
        type MaxVoteNum: Get<u32>;

        /// Admin origin for privileged operations
        type AdminOrigin: EnsureOrigin<Self::RuntimeOrigin>;

        type WeightInfo: weights::WeightInfo;
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

    /// 记录已使用的密钥镜像 (Key Image)
    /// Key1: PollId, Key2: KeyImage -> Value: ()
    #[pallet::storage]
    pub type UsedKeyImages<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        u32, // PollId
        Blake2_128Concat,
        CompressedRistrettoWrapper, // Key Image
        (),
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

    /// 认证为老师的账户
    /// Key: AccountId, Value: ()
    #[pallet::storage]
    #[pallet::getter(fn teachers)]
    pub type Teachers<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, (), OptionQuery>;

    /// Events
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A ring group has been registered
        RingRegistered { ring_id: PollId },
        /// A poll has been created
        PollCreated { poll_id: PollId },
        /// A vote has been submitted
        VoteSubmitted { poll_id: PollId, vote_index: u32 },
        /// A poll has been revealed and completed [Changed]
        PollCompleted {
            poll_id: PollId,
            tally: Tally,
            private_key: ScalarWrapper,
        },
        /// A poll is been tallying
        PollTallying { poll_id: PollId },
        /// A poll has been cancelled
        PollCancelled { poll_id: PollId, reason: Vec<u8> },
        /// A poll has been paused
        PollPaused { poll_id: PollId, reason: Vec<u8> },
        /// A poll has been actived
        PollActive { poll_id: PollId },
        /// A poll deadline has been updated
        PollDeadlineUpdated {
            poll_id: PollId,
            new_deadline: BlockNumberFor<T>,
        },
        /// 被授权创建投票
        TeacherAuthorized { who: T::AccountId },
        /// 被取消授权
        TeacherRevoked { who: T::AccountId },
        /// 投票所有权已转移
        PollOwnershipTransferred {
            poll_id: PollId,
            from: T::AccountId,
            to: T::AccountId,
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
        /// Invalid status transition
        InvalidStatusTransition,
        /// Ring too large
        RingTooLarge,
        /// Description too long
        DescriptionTooLong,
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
        /// *输入构造*：
        ///
        /// 用户需要使用 `curve25519-dalek` 生成公钥。
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::register_ring())]
        pub fn register_ring(
            origin: OriginFor<T>,
            // 包含多个 32 字节公钥的向量。
            ring: Vec<CompressedRistrettoWrapper>,
        ) -> DispatchResult {
            let who = ensure_signed(origin.clone())?;
            ensure!(
                Teachers::<T>::contains_key(&who) || T::AdminOrigin::ensure_origin(origin).is_ok(),
                Error::<T>::NoPermission
            );

            let bounded_ring: BoundedVec<CompressedRistrettoWrapper, T::MaxRingSize> =
                ring.try_into().map_err(|_| Error::<T>::RingTooLarge)?;

            let ring_id = RingCount::<T>::get();
            let next_ring_id = ring_id.wrapping_add(1);

            Rings::<T>::insert(ring_id, bounded_ring);
            RingCount::<T>::put(next_ring_id);

            Self::deposit_event(Event::RingRegistered { ring_id });
            Ok(())
        }

        /// Create a new poll
        ///
        /// 初始化一个投票场次，规定谁能投（ring_id）、什么时候截止（deadline）以及加密选票用的公钥。
        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::create_poll())]
        pub fn create_poll(
            origin: OriginFor<T>,
            ring_id: RingId,
            description: Vec<u8>,
            // 投票说明的原始字节，链上会通过 Preimage 模块计算其哈希以节省空间。
            metadata: Vec<u8>,
            deadline: BlockNumberFor<T>,
            // 这是管理员生成的临时公钥，用户需用它加密选票。
            poll_public_key: CompressedRistrettoWrapper,
        ) -> DispatchResult {
            let who = ensure_signed(origin.clone())?;
            ensure!(
                Teachers::<T>::contains_key(&who) || T::AdminOrigin::ensure_origin(origin).is_ok(),
                Error::<T>::NoPermission
            );

            ensure!(Rings::<T>::contains_key(ring_id), Error::<T>::RingNotFound);

            let bounded_description: BoundedVec<u8, T::MaxDescriptionLength> = description
                .try_into()
                .map_err(|_| Error::<T>::DescriptionTooLong)?;

            let poll_id = PollCount::<T>::get();
            let next_poll_id = poll_id.wrapping_add(1);

            let metadata_hash =
                T::Preimages::note(scale_info::prelude::borrow::Cow::Borrowed(&metadata))?;

            // 内部包含自动调度逻辑：到达 deadline 时自动触发状态切换。
            let poll = Poll::new(
                poll_id,
                ring_id,
                who,
                bounded_description,
                metadata_hash,
                deadline,
                poll_public_key,
            )?;

            Polls::<T>::insert(poll_id, poll);
            PollCount::<T>::put(next_poll_id);

            Self::deposit_event(Event::PollCreated { poll_id });
            Ok(())
        }

        /// Submit a vote
        ///
        /// 同时验证身份（环签名）和唯一性（Key Image）。
        ///
        /// *输入构造*：
        ///
        /// 用户需在链下完成以下步骤：
        /// 1. 生成随机临时公钥 `ephemeral_public_key`。
        /// 2. 使用 `poll_public_key` 对选票进行 ECIES 加密，得到 `ciphertext`。
        /// 3. 构造待签名消息为 `message = ephemeral_public_key || ciphertext`。
        /// 4. 使用自己的私钥 + 整个 Ring 的公钥列表，通过 `nazgul` 生成 BLSAG 签名（得到 `challenge`, `responses`, `key_image`）。
        /// 5. 其中 AAD 绑定：`aad = genesis_hash || poll_id || key_image` 以防重放攻击。
        #[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::vote())]
        pub fn vote(
            origin: OriginFor<T>,
            poll_id: PollId,
            ephemeral_public_key: CompressedRistrettoWrapper,
            ciphertext: Vec<u8>,
            challenge: ScalarWrapper,
            responses: Vec<ScalarWrapper>,
            key_image: CompressedRistrettoWrapper,
        ) -> DispatchResult {
            ensure_none(origin)?;

            let mut poll = Polls::<T>::get(poll_id).ok_or(Error::<T>::PollNotFound)?;
            let status = poll.get_status();
            ensure!(status == PollStatus::Active, Error::<T>::InvalidPollStatus);

            ensure!(
                !UsedKeyImages::<T>::contains_key(poll_id, &key_image),
                Error::<T>::KeyImageAlreadyUsed
            );

            let ring = Rings::<T>::get(poll.ring_id).ok_or(Error::<T>::RingNotFound)?;

            let bounded_ciphertext: BoundedVec<u8, T::MaxCiphertextLength> = ciphertext
                .clone()
                .try_into()
                .map_err(|_| Error::<T>::CiphertextTooLarge)?;

            let encrypted_vote = EncryptedVote {
                ephemeral_public_key,
                ciphertext: bounded_ciphertext,
                key_image: key_image.clone(),
            };

            // 为了确保 ciphertext 能被 ephemeral_private_key 解密，签名消息需要包含 ephemeral_public_key。
            let message = encrypted_vote.to_bytes();

            let blsag_wrapper = BLSAGWrapper::<T::MaxRingSize> {
                challenge,
                responses: responses.try_into().map_err(|_| Error::<T>::RingTooLarge)?,
                ring: ring.clone(),
                key_image: key_image.clone(),
            };

            let is_valid = BLSAG::verify::<blake2::Blake2b512>(blsag_wrapper.into(), &message);
            ensure!(is_valid, Error::<T>::InvalidSignature);

            UsedKeyImages::<T>::insert(poll_id, &key_image, ());
            let vote_index =
                EncryptedVotes::<T>::try_mutate(poll_id, |votes| -> Result<u32, DispatchError> {
                    votes
                        .try_push(encrypted_vote)
                        .map_err(|_| Error::<T>::Overflow)?;

                    Ok(votes.len() as u32 - 1)
                })?;

            Self::deposit_event(Event::VoteSubmitted {
                poll_id,
                vote_index,
            });
            Ok(())
        }

        /// Reveal the poll outcome
        ///
        /// 投票结束后，管理员（或持有私钥的人）公布私钥和统计结果。
        #[pallet::call_index(3)]
        #[pallet::weight(T::WeightInfo::tally())]
        pub fn tally(
            origin: OriginFor<T>,
            poll_id: PollId,
            claimed_tally: Tally,
            private_key: ScalarWrapper,
        ) -> DispatchResult {
            ensure_signed(origin)?;

            // Get poll and ensure it's in tallying status
            let mut poll = Polls::<T>::get(poll_id).ok_or(Error::<T>::PollNotFound)?;
            let status = poll.get_status();
            ensure!(
                status == PollStatus::Tallying,
                Error::<T>::InvalidPollStatus
            );

            // --- Core Security Check ---
            // Verify if Private Key matches Public Key
            // Derived_Pub = Private_Key * G
            // 简单验证私钥是否对应于之前存储的公钥
            let derived_pub = RistrettoPoint::mul_base(&private_key.0);
            let stored_pub: RistrettoPoint = poll.poll_public_key.clone().into();

            // If mismatch, admin provided wrong key
            ensure!(derived_pub == stored_pub, Error::<T>::InvalidSignature);

            // --- Update Status ---
            poll.set_status(PollStatus::Completed)?;
            poll.tally = Some(claimed_tally.clone());
            poll.poll_private_key = Some(private_key.clone());

            Polls::<T>::insert(poll_id, poll);

            // Emit event so everyone can verify off-chain
            Self::deposit_event(Event::PollCompleted {
                poll_id,
                tally: claimed_tally,
                private_key,
            });

            Ok(())
        }

        /// Manually trigger tallying state (Stop Voting)
        #[pallet::call_index(4)]
        #[pallet::weight(T::WeightInfo::tally_poll())]
        pub fn tally_poll(origin: OriginFor<T>, poll_id: u32) -> DispatchResult {
            T::AdminOrigin::ensure_origin(origin.clone())?;

            Polls::<T>::try_mutate(poll_id, |poll_opt| -> DispatchResult {
                let poll = poll_opt.as_mut().ok_or(Error::<T>::PollNotFound)?;
                // Only allow transition from Active
                if poll.status == PollStatus::Active {
                    poll.set_status(PollStatus::Tallying)?;
                }
                Ok(())
            })?;

            Self::deposit_event(Event::PollTallying { poll_id });
            Ok(())
        }

        /// Cancel a poll
        #[pallet::call_index(5)]
        #[pallet::weight(T::WeightInfo::cancel_poll())]
        pub fn cancel_poll(origin: OriginFor<T>, poll_id: u32, reason: Vec<u8>) -> DispatchResult {
            let who = ensure_signed(origin.clone())?;

            Polls::<T>::try_mutate(poll_id, |poll_opt| -> DispatchResult {
                let poll = poll_opt.as_mut().ok_or(Error::<T>::PollNotFound)?;
                ensure!(
                    who == poll.owner || T::AdminOrigin::ensure_origin(origin).is_ok(),
                    Error::<T>::NoPermission
                );
                poll.set_status(PollStatus::Cancelled)?;
                Ok(())
            })?;

            Self::deposit_event(Event::PollCancelled { poll_id, reason });
            Ok(())
        }

        /// Pause a poll
        #[pallet::call_index(6)]
        #[pallet::weight(T::WeightInfo::pause_poll())]
        pub fn pause_poll(origin: OriginFor<T>, poll_id: u32, reason: Vec<u8>) -> DispatchResult {
            let who = ensure_signed(origin.clone())?;

            Polls::<T>::try_mutate(poll_id, |poll_opt| -> DispatchResult {
                let poll = poll_opt.as_mut().ok_or(Error::<T>::PollNotFound)?;
                ensure!(
                    who == poll.owner || T::AdminOrigin::ensure_origin(origin).is_ok(),
                    Error::<T>::NoPermission
                );
                poll.set_status(PollStatus::Paused)?;
                Ok(())
            })?;

            Self::deposit_event(Event::PollPaused { poll_id, reason });
            Ok(())
        }

        /// Active a poll
        #[pallet::call_index(7)]
        #[pallet::weight(T::WeightInfo::active_poll())]
        pub fn active_poll(origin: OriginFor<T>, poll_id: u32) -> DispatchResult {
            let who = ensure_signed(origin.clone())?;

            Polls::<T>::try_mutate(poll_id, |poll_opt| -> DispatchResult {
                let poll = poll_opt.as_mut().ok_or(Error::<T>::PollNotFound)?;
                ensure!(
                    who == poll.owner || T::AdminOrigin::ensure_origin(origin).is_ok(),
                    Error::<T>::NoPermission
                );
                poll.set_status(PollStatus::Active)?;
                Ok(())
            })?;

            Self::deposit_event(Event::PollActive { poll_id });
            Ok(())
        }

        /// Set deadline for a poll
        #[pallet::call_index(8)]
        #[pallet::weight(T::WeightInfo::set_deadline())]
        pub fn set_deadline(
            origin: OriginFor<T>,
            poll_id: u32,
            new_deadline: BlockNumberFor<T>,
        ) -> DispatchResult {
            let who = ensure_signed(origin.clone())?;

            Polls::<T>::try_mutate(poll_id, |poll_opt| -> DispatchResult {
                let poll = poll_opt.as_mut().ok_or(Error::<T>::PollNotFound)?;
                ensure!(
                    who == poll.owner || T::AdminOrigin::ensure_origin(origin).is_ok(),
                    Error::<T>::NoPermission
                );
                poll.set_deadline(new_deadline)?;
                Ok(())
            })?;

            Self::deposit_event(Event::PollDeadlineUpdated {
                poll_id,
                new_deadline,
            });
            Ok(())
        }

        /// 授权给某个账户（老师）创建投票的权限
        #[pallet::call_index(9)]
        #[pallet::weight(T::WeightInfo::authorize_teacher())]
        pub fn authorize_teacher(origin: OriginFor<T>, who: T::AccountId) -> DispatchResult {
            // 1. 只有最高管理员可以执行授权操作
            T::AdminOrigin::ensure_origin(origin)?;

            // 2. 检查是否已经授权，避免重复写入
            if Teachers::<T>::contains_key(&who) {
                return Ok(());
            }

            // 3. 写入存储
            Teachers::<T>::insert(&who, ());

            // 4. 发出事件
            Self::deposit_event(Event::TeacherAuthorized { who });
            Ok(())
        }

        /// 撤销某个账户（老师）的权限
        #[pallet::call_index(10)]
        #[pallet::weight(T::WeightInfo::revoke_teacher())]
        pub fn revoke_teacher(origin: OriginFor<T>, who: T::AccountId) -> DispatchResult {
            // 1. 只有最高管理员可以执行撤销操作
            T::AdminOrigin::ensure_origin(origin)?;

            // 2. 检查是否存在
            if !Teachers::<T>::contains_key(&who) {
                return Err(Error::<T>::NoPermission.into());
            }

            // 3. 移除存储
            Teachers::<T>::remove(&who);

            // 4. 发出事件
            Self::deposit_event(Event::TeacherRevoked { who });
            Ok(())
        }

        /// 更改所有权
        /// 场景：紧急接管、账号丢失恢复、恶意老师处理
        #[pallet::call_index(31)]
        #[pallet::weight(T::WeightInfo::change_owner())]
        pub fn change_owner(
            origin: OriginFor<T>,
            poll_id: PollId,
            new_owner: T::AccountId,
        ) -> DispatchResult {
            // 1. 只有最高管理员可调用
            T::AdminOrigin::ensure_origin(origin)?;

            Polls::<T>::try_mutate(poll_id, |poll_opt| -> DispatchResult {
                let poll = poll_opt.as_mut().ok_or(Error::<T>::PollNotFound)?;

                let old_owner = poll.owner.clone();

                // 2. 强制覆盖 Owner
                poll.owner = new_owner.clone();

                Self::deposit_event(Event::PollOwnershipTransferred {
                    poll_id,
                    from: old_owner,
                    to: new_owner,
                });
                Ok(())
            })
        }
    }

    #[pallet::validate_unsigned]
    impl<T: Config> ValidateUnsigned for Pallet<T> {
        type Call = Call<T>;

        /// 在交易进入内存池前执行。
        /// 这里的代码运行在每一个节点上，用于拦截垃圾交易。
        fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
            // 1. 模式匹配：只拦截 vote 调用
            if let Call::vote {
                poll_id,
                ephemeral_public_key,
                ciphertext,
                challenge,
                responses,
                key_image,
            } = call
            {
                // 2. 数据准备 (读取 Storage)
                // 注意：在 validate_unsigned 中读存储是允许的，但要尽量快
                let mut poll = Polls::<T>::get(poll_id).ok_or(InvalidTransaction::Call)?;
                let status = poll.get_status();
                if status != PollStatus::Active {
                    return InvalidTransaction::Call.into();
                }

                // 检查双花 (Key Image) - 这是最重要的防 DDoS 检查！
                if UsedKeyImages::<T>::contains_key(poll_id, key_image) {
                    return InvalidTransaction::Stale.into(); // Stale 表示已过期/已存在
                }

                // 3. 密码学验证 (Ring Signature)
                let Some(ring) = Rings::<T>::get(poll.ring_id) else {
                    return InvalidTransaction::Call.into();
                };

                // 构造数据结构进行验证
                let bounded_ciphertext =
                    BoundedVec::<u8, T::MaxCiphertextLength>::try_from(ciphertext.clone())
                        .map_err(|_| InvalidTransaction::Call)?;

                let encrypted_vote = EncryptedVote {
                    ephemeral_public_key: ephemeral_public_key.clone(),
                    ciphertext: bounded_ciphertext,
                    key_image: key_image.clone(),
                };
                let message = encrypted_vote.to_bytes();

                // 转换 responses
                let bounded_responses: BoundedVec<ScalarWrapper, T::MaxRingSize> = responses
                    .clone()
                    .try_into()
                    .map_err(|_| InvalidTransaction::Call)?;

                let blsag_wrapper = BLSAGWrapper::<T::MaxRingSize> {
                    challenge: challenge.clone(),
                    responses: bounded_responses,
                    ring: ring,
                    key_image: key_image.clone(),
                };

                // 执行验证
                let is_valid = BLSAG::verify::<blake2::Blake2b512>(blsag_wrapper.into(), &message);

                if !is_valid {
                    return InvalidTransaction::BadProof.into(); // 签名错误，踢出
                }

                // 4. 返回通过凭证
                ValidTransaction::with_tag_prefix("RingSigVoting")
                    // Priority: 优先级。我们可以给高一点。
                    .priority(T::WeightInfo::vote().ref_time() as u64)
                    // Requires: 依赖关系。这里暂无。
                    // Provides: 这笔交易提供了什么？提供了这个 KeyImage 的消耗。
                    // 这样如果有第二笔相同 KeyImage 的交易进来，会被池子自动排斥。
                    .and_provides((poll_id, key_image))
                    .longevity(64) // 交易在池子里的存活块数
                    .propagate(true) // 允许传播给其他节点
                    .build()
            } else {
                // 如果不是 vote 调用，则不支持无签名
                InvalidTransaction::Call.into()
            }
        }
    }
}

// Dummy RNG for nazgul/getrandom compilation in wasm
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
