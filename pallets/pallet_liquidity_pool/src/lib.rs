#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

mod math;
mod types;

#[cfg(any(test, feature = "runtime-benchmarks"))]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use codec::HasCompact;
// use frame::{
//     pallet_prelude::*,
//     traits::{
//         fungibles::{Inspect, Mutate, Transfer},
//         tokens::Preservation,
//     },
//     PalletId,
// };
pub use pallet::*;
pub use types::*;

#[frame::pallet]
pub mod pallet {
    use super::*;
    use frame::prelude::*;
    use frame::traits::{
        fungibles::{Create, Inspect, Mutate},
        tokens::Preservation,
        AccountIdConversion, AtLeast32BitUnsigned, CheckedAdd, CheckedSub, One, Zero,
    };
    pub use math::ConstantProductMath;
    // use crate::pallet::nonfungible_v2::Transfer;
    use scale_info::prelude::{vec, vec::Vec};

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// 资产 ID 类型
        type AssetId: Member
            + Parameter
            + Copy
            + MaybeSerializeDeserialize
            + MaxEncodedLen
            + Ord
            + From<u32>;

        /// 余额类型
        type Balance: Member
            + Parameter
            + AtLeast32BitUnsigned
            + Default
            + Copy
            + MaybeSerializeDeserialize
            + MaxEncodedLen
            + HasCompact
            + From<u32>
            + Into<u128>;

        /// 池 ID 类型
        type PoolId: Member
            + Parameter
            + AtLeast32BitUnsigned
            + Default
            + Copy
            + MaybeSerializeDeserialize
            + MaxEncodedLen
            + HasCompact;

        /// 资产接口
        type Assets: Inspect<Self::AccountId, AssetId = Self::AssetId, Balance = Self::Balance>
            + Mutate<Self::AccountId>
            + Create<Self::AccountId>;

        /// Pallet ID
        #[pallet::constant]
        type PalletId: Get<PalletId>;

        /// 最小流动性 (防止操纵)
        #[pallet::constant]
        type MinimumLiquidity: Get<Self::Balance>;

        /// 默认手续费率 (基点)
        #[pallet::constant]
        type DefaultFeeRate: Get<u32>;

        /// 最大交易路径长度
        #[pallet::constant]
        type MaxPathLength: Get<u32>;
    }

    /// 池信息存储
    #[pallet::storage]
    #[pallet::getter(fn pools)]
    pub type Pools<T: Config> =
        StorageMap<_, Blake2_128Concat, T::PoolId, PoolInfo<T::AssetId, T::Balance>>;

    /// 交易对到池 ID 的映射
    #[pallet::storage]
    #[pallet::getter(fn trading_pairs)]
    pub type TradingPairs<T: Config> =
        StorageMap<_, Blake2_128Concat, (T::AssetId, T::AssetId), T::PoolId>;

    /// LP Token 持有量
    #[pallet::storage]
    #[pallet:: getter(fn lp_positions)]
    pub type LpPositions<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::PoolId,
        Blake2_128Concat,
        T::AccountId,
        LpPosition<T::Balance>,
    >;

    /// 下一个池 ID
    #[pallet::storage]
    #[pallet::getter(fn next_pool_id)]
    pub type NextPoolId<T: Config> = StorageValue<_, T::PoolId, ValueQuery>;

    #[pallet::event]
    #[pallet:: generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// 池创建 [pool_id, token_a, token_b, creator]
        PoolCreated {
            pool_id: T::PoolId,
            token_a: T::AssetId,
            token_b: T::AssetId,
            creator: T::AccountId,
        },
        /// 添加流动性 [who, pool_id, amount_a, amount_b, lp_tokens]
        LiquidityAdded {
            who: T::AccountId,
            pool_id: T::PoolId,
            amount_a: T::Balance,
            amount_b: T::Balance,
            lp_tokens: T::Balance,
        },
        /// 移除流动性 [who, pool_id, lp_tokens, amount_a, amount_b]
        LiquidityRemoved {
            who: T::AccountId,
            pool_id: T::PoolId,
            lp_tokens: T::Balance,
            amount_a: T::Balance,
            amount_b: T::Balance,
        },
        /// 交易执行 [who, path, amount_in, amount_out]
        Swap {
            who: T::AccountId,
            path: Vec<T::AssetId>,
            amount_in: T::Balance,
            amount_out: T::Balance,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// 池不存在
        PoolNotFound,
        /// 池已存在
        PoolAlreadyExists,
        /// 相同的资产
        IdenticalAssets,
        /// 金额为零
        ZeroAmount,
        /// 流动性不足
        InsufficientLiquidity,
        /// 输入金额不足
        InsufficientInputAmount,
        /// 输出金额不足
        InsufficientOutputAmount,
        /// 余额不足
        InsufficientBalance,
        /// 滑点过高
        SlippageExceeded,
        /// 无效的交易路径
        InvalidPath,
        /// 溢出错误
        Overflow,
        /// 池未激活
        PoolInactive,
        /// 最小流动性不足
        InsufficientMinimumLiquidity,
        /// LP Token 不足
        InsufficientLpTokens,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// 创建新的流动性池
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn create_pool(
            origin: OriginFor<T>,
            token_a: T::AssetId,
            token_b: T::AssetId,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(token_a != token_b, Error::<T>::IdenticalAssets);

            // 规范化顺序
            let (token_0, token_1) = if token_a < token_b {
                (token_a, token_b)
            } else {
                (token_b, token_a)
            };

            ensure!(
                !TradingPairs::<T>::contains_key((token_0, token_1)),
                Error::<T>::PoolAlreadyExists
            );

            let pool_id = NextPoolId::<T>::get();
            let next_id = pool_id
                .checked_add(&One::one())
                .ok_or(Error::<T>::Overflow)?;

            let pool_info = PoolInfo {
                token_a: token_0,
                token_b: token_1,
                reserve_a: Zero::zero(),
                reserve_b: Zero::zero(),
                total_lp_supply: Zero::zero(),
                fee_rate: T::DefaultFeeRate::get(),
                is_active: true,
            };

            Pools::<T>::insert(pool_id, pool_info);
            TradingPairs::<T>::insert((token_0, token_1), pool_id);
            NextPoolId::<T>::put(next_id);

            Self::deposit_event(Event::PoolCreated {
                pool_id,
                token_a: token_0,
                token_b: token_1,
                creator: who,
            });

            Ok(())
        }

        /// 添加流动性
        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn add_liquidity(
            origin: OriginFor<T>,
            pool_id: T::PoolId,
            amount_a_desired: T::Balance,
            amount_b_desired: T::Balance,
            amount_a_min: T::Balance,
            amount_b_min: T::Balance,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(!amount_a_desired.is_zero(), Error::<T>::ZeroAmount);
            ensure!(!amount_b_desired.is_zero(), Error::<T>::ZeroAmount);

            let mut pool = Pools::<T>::get(pool_id).ok_or(Error::<T>::PoolNotFound)?;
            ensure!(pool.is_active, Error::<T>::PoolInactive);

            let pool_account = Self::pool_account(pool_id);

            let (amount_a, amount_b, lp_tokens) = if pool.total_lp_supply.is_zero() {
                // 首次添加流动性
                let lp =
                    ConstantProductMath::calculate_initial_lp(amount_a_desired, amount_b_desired)
                        .ok_or(Error::<T>::Overflow)?;

                ensure!(
                    lp >= T::MinimumLiquidity::get(),
                    Error::<T>::InsufficientMinimumLiquidity
                );

                let lp_for_user = lp
                    .checked_sub(&T::MinimumLiquidity::get())
                    .ok_or(Error::<T>::Overflow)?;

                (amount_a_desired, amount_b_desired, lp_for_user)
            } else {
                // 后续添加流动性 - 按比例计算
                let amount_b_optimal =
                    ConstantProductMath::quote(amount_a_desired, pool.reserve_a, pool.reserve_b)
                        .ok_or(Error::<T>::Overflow)?;

                let (final_a, final_b) = if amount_b_optimal <= amount_b_desired {
                    ensure!(
                        amount_b_optimal >= amount_b_min,
                        Error::<T>::SlippageExceeded
                    );
                    (amount_a_desired, amount_b_optimal)
                } else {
                    let amount_a_optimal = ConstantProductMath::quote(
                        amount_b_desired,
                        pool.reserve_b,
                        pool.reserve_a,
                    )
                    .ok_or(Error::<T>::Overflow)?;

                    ensure!(
                        amount_a_optimal <= amount_a_desired,
                        Error::<T>::SlippageExceeded
                    );
                    ensure!(
                        amount_a_optimal >= amount_a_min,
                        Error::<T>::SlippageExceeded
                    );
                    (amount_a_optimal, amount_b_desired)
                };

                let lp = ConstantProductMath::calculate_lp_amount(
                    final_a,
                    final_b,
                    pool.reserve_a,
                    pool.reserve_b,
                    pool.total_lp_supply,
                )
                .ok_or(Error::<T>::Overflow)?;

                (final_a, final_b, lp)
            };

            // 转账代币到池账户
            T::Assets::transfer(
                pool.token_a,
                &who,
                &pool_account,
                amount_a,
                Preservation::Expendable,
            )?;
            T::Assets::transfer(
                pool.token_b,
                &who,
                &pool_account,
                amount_b,
                Preservation::Expendable,
            )?;

            // 更新池状态
            pool.reserve_a = pool
                .reserve_a
                .checked_add(&amount_a)
                .ok_or(Error::<T>::Overflow)?;
            pool.reserve_b = pool
                .reserve_b
                .checked_add(&amount_b)
                .ok_or(Error::<T>::Overflow)?;

            if pool.total_lp_supply.is_zero() {
                pool.total_lp_supply = lp_tokens
                    .checked_add(&T::MinimumLiquidity::get())
                    .ok_or(Error::<T>::Overflow)?;
            } else {
                pool.total_lp_supply = pool
                    .total_lp_supply
                    .checked_add(&lp_tokens)
                    .ok_or(Error::<T>::Overflow)?;
            }

            Pools::<T>::insert(pool_id, pool);

            // 更新 LP 持仓
            LpPositions::<T>::mutate(pool_id, &who, |position| {
                let mut pos = position.take().unwrap_or(LpPosition {
                    lp_amount: Zero::zero(),
                    last_update: 0,
                });
                pos.lp_amount = pos
                    .lp_amount
                    .checked_add(&lp_tokens)
                    .ok_or(Error::<T>::Overflow)?;
                pos.last_update = frame_system::Pallet::<T>::block_number().saturated_into();
                *position = Some(pos);
                Ok::<(), Error<T>>(())
            })?;

            Self::deposit_event(Event::LiquidityAdded {
                who,
                pool_id,
                amount_a,
                amount_b,
                lp_tokens,
            });

            Ok(())
        }

        /// 移除流动性
        #[pallet::call_index(2)]
        #[pallet::weight(10_000)]
        pub fn remove_liquidity(
            origin: OriginFor<T>,
            pool_id: T::PoolId,
            lp_tokens: T::Balance,
            amount_a_min: T::Balance,
            amount_b_min: T::Balance,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(!lp_tokens.is_zero(), Error::<T>::ZeroAmount);

            let mut pool = Pools::<T>::get(pool_id).ok_or(Error::<T>::PoolNotFound)?;
            let pool_account = Self::pool_account(pool_id);

            // 检查 LP Token 余额
            let mut position =
                LpPositions::<T>::get(pool_id, &who).ok_or(Error::<T>::InsufficientLpTokens)?;
            ensure!(
                position.lp_amount >= lp_tokens,
                Error::<T>::InsufficientLpTokens
            );

            // 计算返还金额
            let amount_a = lp_tokens
                .checked_mul(&pool.reserve_a)
                .and_then(|v| v.checked_div(&pool.total_lp_supply))
                .ok_or(Error::<T>::Overflow)?;

            let amount_b = lp_tokens
                .checked_mul(&pool.reserve_b)
                .and_then(|v| v.checked_div(&pool.total_lp_supply))
                .ok_or(Error::<T>::Overflow)?;

            ensure!(amount_a >= amount_a_min, Error::<T>::SlippageExceeded);
            ensure!(amount_b >= amount_b_min, Error::<T>::SlippageExceeded);

            // 转账代币给用户
            T::Assets::transfer(
                pool.token_a,
                &pool_account,
                &who,
                amount_a,
                Preservation::Expendable,
            )?;
            T::Assets::transfer(
                pool.token_b,
                &pool_account,
                &who,
                amount_b,
                Preservation::Expendable,
            )?;

            // 更新池状态
            pool.reserve_a = pool
                .reserve_a
                .checked_sub(&amount_a)
                .ok_or(Error::<T>::Overflow)?;
            pool.reserve_b = pool
                .reserve_b
                .checked_sub(&amount_b)
                .ok_or(Error::<T>::Overflow)?;
            pool.total_lp_supply = pool
                .total_lp_supply
                .checked_sub(&lp_tokens)
                .ok_or(Error::<T>::Overflow)?;

            Pools::<T>::insert(pool_id, pool);

            // 更新 LP 持仓
            position.lp_amount = position
                .lp_amount
                .checked_sub(&lp_tokens)
                .ok_or(Error::<T>::Overflow)?;
            if position.lp_amount.is_zero() {
                LpPositions::<T>::remove(pool_id, &who);
            } else {
                LpPositions::<T>::insert(pool_id, &who, position);
            }

            Self::deposit_event(Event::LiquidityRemoved {
                who,
                pool_id,
                lp_tokens,
                amount_a,
                amount_b,
            });

            Ok(())
        }

        /// 精确输入交易
        #[pallet::call_index(3)]
        #[pallet::weight(10_000)]
        pub fn swap_exact_tokens_for_tokens(
            origin: OriginFor<T>,
            amount_in: T::Balance,
            amount_out_min: T::Balance,
            path: Vec<T::AssetId>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(!amount_in.is_zero(), Error::<T>::ZeroAmount);
            ensure!(path.len() >= 2, Error::<T>::InvalidPath);
            ensure!(
                path.len() <= T::MaxPathLength::get() as usize,
                Error::<T>::InvalidPath
            );

            let amounts = Self::get_amounts_out(amount_in, &path)?;
            let amount_out = *amounts.last().ok_or(Error::<T>::InvalidPath)?;

            ensure!(amount_out >= amount_out_min, Error::<T>::SlippageExceeded);

            Self::execute_swap(&who, &amounts, &path)?;

            Self::deposit_event(Event::Swap {
                who,
                path,
                amount_in,
                amount_out,
            });

            Ok(())
        }

        /// 精确输出交易
        #[pallet::call_index(4)]
        #[pallet::weight(10_000)]
        pub fn swap_tokens_for_exact_tokens(
            origin: OriginFor<T>,
            amount_out: T::Balance,
            amount_in_max: T::Balance,
            path: Vec<T::AssetId>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(!amount_out.is_zero(), Error::<T>::ZeroAmount);
            ensure!(path.len() >= 2, Error::<T>::InvalidPath);

            let amounts = Self::get_amounts_in(amount_out, &path)?;
            let amount_in = *amounts.first().ok_or(Error::<T>::InvalidPath)?;

            ensure!(amount_in <= amount_in_max, Error::<T>::SlippageExceeded);

            Self::execute_swap(&who, &amounts, &path)?;

            Self::deposit_event(Event::Swap {
                who,
                path,
                amount_in,
                amount_out,
            });

            Ok(())
        }
    }

    // 辅助函数
    impl<T: Config> Pallet<T> {
        /// 获取池账户
        pub fn pool_account(pool_id: T::PoolId) -> T::AccountId {
            T::PalletId::get().into_sub_account_truncating(pool_id)
        }

        /// 计算输出金额数组
        pub fn get_amounts_out(
            amount_in: T::Balance,
            path: &[T::AssetId],
        ) -> Result<Vec<T::Balance>, Error<T>> {
            ensure!(path.len() >= 2, Error::<T>::InvalidPath);

            let mut amounts = Vec::new();
            amounts.push(amount_in);

            for i in 0..path.len() - 1 {
                let (token_0, token_1) = if path[i] < path[i + 1] {
                    (path[i], path[i + 1])
                } else {
                    (path[i + 1], path[i])
                };

                let pool_id =
                    TradingPairs::<T>::get((token_0, token_1)).ok_or(Error::<T>::PoolNotFound)?;
                let pool = Pools::<T>::get(pool_id).ok_or(Error::<T>::PoolNotFound)?;

                ensure!(pool.is_active, Error::<T>::PoolInactive);

                let (reserve_in, reserve_out) = if path[i] == token_0 {
                    (pool.reserve_a, pool.reserve_b)
                } else {
                    (pool.reserve_b, pool.reserve_a)
                };

                let amount_out = ConstantProductMath::get_amount_out(
                    *amounts.last().unwrap(),
                    reserve_in,
                    reserve_out,
                    pool.fee_rate,
                )
                .ok_or(Error::<T>::InsufficientLiquidity)?;

                amounts.push(amount_out);
            }

            Ok(amounts)
        }

        /// 计算输入金额数组
        pub fn get_amounts_in(
            amount_out: T::Balance,
            path: &[T::AssetId],
        ) -> Result<Vec<T::Balance>, Error<T>> {
            ensure!(path.len() >= 2, Error::<T>::InvalidPath);

            let mut amounts = vec![Zero::zero(); path.len()];
            amounts[path.len() - 1] = amount_out;

            for i in (0..path.len() - 1).rev() {
                let (token_0, token_1) = if path[i] < path[i + 1] {
                    (path[i], path[i + 1])
                } else {
                    (path[i + 1], path[i])
                };

                let pool_id =
                    TradingPairs::<T>::get((token_0, token_1)).ok_or(Error::<T>::PoolNotFound)?;
                let pool = Pools::<T>::get(pool_id).ok_or(Error::<T>::PoolNotFound)?;

                let (reserve_in, reserve_out) = if path[i] == token_0 {
                    (pool.reserve_a, pool.reserve_b)
                } else {
                    (pool.reserve_b, pool.reserve_a)
                };

                let amount_in = ConstantProductMath::get_amount_in(
                    amounts[i + 1],
                    reserve_in,
                    reserve_out,
                    pool.fee_rate,
                )
                .ok_or(Error::<T>::InsufficientLiquidity)?;

                amounts[i] = amount_in;
            }

            Ok(amounts)
        }

        /// 执行交易
        fn execute_swap(
            who: &T::AccountId,
            amounts: &[T::Balance],
            path: &[T::AssetId],
        ) -> DispatchResult {
            for i in 0..path.len() - 1 {
                let (token_0, token_1) = if path[i] < path[i + 1] {
                    (path[i], path[i + 1])
                } else {
                    (path[i + 1], path[i])
                };

                let pool_id =
                    TradingPairs::<T>::get((token_0, token_1)).ok_or(Error::<T>::PoolNotFound)?;
                let mut pool = Pools::<T>::get(pool_id).ok_or(Error::<T>::PoolNotFound)?;
                let pool_account = Self::pool_account(pool_id);

                let (amount_in, amount_out) = (amounts[i], amounts[i + 1]);

                // 第一笔转账从用户或上一个池到当前池
                let from = if i == 0 {
                    who.clone()
                } else {
                    // 从上一个池转入
                    let (prev_token_0, prev_token_1) = if path[i - 1] < path[i] {
                        (path[i - 1], path[i])
                    } else {
                        (path[i], path[i - 1])
                    };
                    let prev_pool_id = TradingPairs::<T>::get((prev_token_0, prev_token_1))
                        .ok_or(Error::<T>::PoolNotFound)?;
                    Self::pool_account(prev_pool_id)
                };

                let to = if i == path.len() - 2 {
                    // 最后一跳，转给用户
                    who.clone()
                } else {
                    // 中间跳，转给下一个池子
                    let (next_token_0, next_token_1) = if path[i + 1] < path[i + 2] {
                        (path[i + 1], path[i + 2])
                    } else {
                        (path[i + 2], path[i + 1])
                    };
                    let next_pool_id = TradingPairs::<T>::get((next_token_0, next_token_1))
                        .ok_or(Error::<T>::PoolNotFound)?;
                    Self::pool_account(next_pool_id)
                };

                // 更新储备量
                if path[i] == token_0 {
                    pool.reserve_a = pool
                        .reserve_a
                        .checked_add(&amount_in)
                        .ok_or(Error::<T>::Overflow)?;
                    pool.reserve_b = pool
                        .reserve_b
                        .checked_sub(&amount_out)
                        .ok_or(Error::<T>::Overflow)?;
                } else {
                    pool.reserve_b = pool
                        .reserve_b
                        .checked_add(&amount_in)
                        .ok_or(Error::<T>::Overflow)?;
                    pool.reserve_a = pool
                        .reserve_a
                        .checked_sub(&amount_out)
                        .ok_or(Error::<T>::Overflow)?;
                }

                Pools::<T>::insert(pool_id, pool);

                <T::Assets as Mutate<T::AccountId>>::transfer(
                    path[i],
                    &from,
                    &pool_account,
                    amount_in,
                    Preservation::Expendable,
                )?;
                <T::Assets as Mutate<T::AccountId>>::transfer(
                    path[i + 1],
                    &pool_account,
                    &to,
                    amount_out,
                    Preservation::Expendable,
                )?;
            }

            Ok(())
        }
    }
}
