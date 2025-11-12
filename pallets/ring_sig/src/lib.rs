#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

mod types;
pub use types::*;

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
        type NumRingLayers: Get<u32>;
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

    /// 提案的计票结果
    /// Key: ProposalId
    /// Value: (赞成票数, 反对票数)
    #[pallet::storage]
    #[pallet::getter(fn proposal_votes)]
    pub type ProposalVotes<T: Config> = StorageMap<
        _,
        Blake2_128Concat, // 可以遍历所有提案的投票结果
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
        Blake2_128Concat,
        ProposalId,
        Blake2_128Concat,
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
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn anonymous_vote(
            origin: OriginFor<T>,
            proposal_id: ProposalId,
            vote: VoteOption,
            challenge: H256,
            responses: Vec<H256>,
            ring: Vec<Vec<H256>>,
            key_images: Vec<H256>,
        ) -> DispatchResult {
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

            let _who = ensure_signed(origin)?;

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

// 当在 no_std (Wasm runtime) 环境下编译时,
// 我们为 `getrandom` 注册一个自定义实现。
#[cfg(not(feature = "std"))]
mod getrandom_impl {
    use getrandom::Error;

    /// 这是一个“虚拟”的 getrandom 实现.
    /// Substrate runtime 必须是确定性的, 绝不能生成随机数.
    /// 签名 (Sign) 操作必须在客户端 (链下) 完成.
    /// 如果 Wasm runtime 中的任何代码 (错误地) 尝试调用此函数...
    /// ...它将 panic, 这是一个安全的设计.
    pub fn getrandom_runtime(_dest: &mut [u8]) -> Result<(), Error> {
        // 我们返回一个错误或 panic. Panic 更能暴露逻辑错误.
        panic!(
            "CRITICAL: getrandom() was called in the Substrate runtime! 
                This environment must be deterministic. 
                All signing operations must be performed client-side."
        );
    }

    // 使用 getrandom 宏来注册我们的自定义实现
    getrandom::register_custom_getrandom!(getrandom_runtime);
}
