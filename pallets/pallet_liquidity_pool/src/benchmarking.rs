use super::*;
use crate::Pallet as LiquidityPool;
use frame::{deps::frame_benchmarking::v2::*, prelude::*};
use frame_system::RawOrigin;
use scale_info::prelude::vec;

#[benchmarks]
mod benchmarks {
    use super::*;

    // 使用 T::Assets trait 来操作资产
    fn setup_asset<T: Config>(
        asset_id: T::AssetId,
        owner: &T::AccountId,
        amount: u128,
    ) -> Result<(), &'static str> {
        // 使用 fungibles:: Create trait
        use frame::token::fungibles::Create;
        // 创建资产
        T::Assets::create(asset_id, owner.clone(), true, 1u32.into())
            .map_err(|_| "Failed to create asset")?;

        // 铸造资产
        use frame::token::fungibles::Mutate;
        let balance: T::Balance = amount.saturated_into();
        T::Assets::mint_into(asset_id, owner, balance).map_err(|_| "Failed to mint asset")?;

        Ok(())
    }

    #[benchmark]
    fn create_pool() {
        let caller: T::AccountId = whitelisted_caller();
        let asset_a: T::AssetId = 1u32.into();
        let asset_b: T::AssetId = 2u32.into();

        // 设置资产
        setup_asset::<T>(asset_a, &caller, 1_000_000_000).expect("Setup asset A failed");
        setup_asset::<T>(asset_b, &caller, 1_000_000_000).expect("Setup asset B failed");

        #[extrinsic_call]
        _(RawOrigin::Signed(caller), asset_a, asset_b);

        // 验证
        assert_eq!(LiquidityPool::<T>::next_pool_id(), 1u32.into());
    }

    #[benchmark]
    fn add_liquidity() {
        let caller: T::AccountId = whitelisted_caller();
        let asset_a: T::AssetId = 1u32.into();
        let asset_b: T::AssetId = 2u32.into();

        // 设置
        setup_asset::<T>(asset_a, &caller, 1_000_000_000).expect("Setup asset A failed");
        setup_asset::<T>(asset_b, &caller, 1_000_000_000).expect("Setup asset B failed");

        // 创建池
        LiquidityPool::<T>::create_pool(RawOrigin::Signed(caller.clone()).into(), asset_a, asset_b)
            .expect("Failed to create pool");

        let pool_id: T::PoolId = 0u32.into();
        let amount_a: T::Balance = 100_000u128.saturated_into();
        let amount_b: T::Balance = 100_000u128.saturated_into();

        #[extrinsic_call]
        _(
            RawOrigin::Signed(caller.clone()),
            pool_id,
            amount_a,
            amount_b,
            0u32.into(),
            0u32.into(),
        );

        // 验证
        let pool = LiquidityPool::<T>::pools(pool_id).expect("Pool should exist");
        assert!(pool.total_lp_supply > 0u32.into());
    }

    #[benchmark]
    fn remove_liquidity() {
        let caller: T::AccountId = whitelisted_caller();
        let asset_a: T::AssetId = 1u32.into();
        let asset_b: T::AssetId = 2u32.into();

        // 设置
        setup_asset::<T>(asset_a, &caller, 1_000_000_000).expect("Setup asset A failed");
        setup_asset::<T>(asset_b, &caller, 1_000_000_000).expect("Setup asset B failed");

        // 创建池
        LiquidityPool::<T>::create_pool(RawOrigin::Signed(caller.clone()).into(), asset_a, asset_b)
            .expect("Failed to create pool");

        let pool_id: T::PoolId = 0u32.into();
        let amount: T::Balance = 100_000u128.saturated_into();

        // 添���流动性
        LiquidityPool::<T>::add_liquidity(
            RawOrigin::Signed(caller.clone()).into(),
            pool_id,
            amount,
            amount,
            0u32.into(),
            0u32.into(),
        )
        .expect("Failed to add liquidity");

        let pool = LiquidityPool::<T>::pools(pool_id).expect("Pool should exist");
        let lp_tokens = pool.total_lp_supply / 2u32.into();

        #[extrinsic_call]
        _(
            RawOrigin::Signed(caller.clone()),
            pool_id,
            lp_tokens,
            0u32.into(),
            0u32.into(),
        );

        // 验证
        let position =
            LiquidityPool::<T>::lp_positions(pool_id, &caller).expect("Position should exist");
        assert!(position.lp_amount < pool.total_lp_supply);
    }

    #[benchmark]
    fn swap_exact_tokens_for_tokens() {
        let caller: T::AccountId = whitelisted_caller();
        let asset_a: T::AssetId = 1u32.into();
        let asset_b: T::AssetId = 2u32.into();

        // 设置
        setup_asset::<T>(asset_a, &caller, 1_000_000_000).expect("Setup asset A failed");
        setup_asset::<T>(asset_b, &caller, 1_000_000_000).expect("Setup asset B failed");

        // 创建池并添加流动性
        LiquidityPool::<T>::create_pool(RawOrigin::Signed(caller.clone()).into(), asset_a, asset_b)
            .expect("Failed to create pool");

        let pool_id: T::PoolId = 0u32.into();
        let liquidity: T::Balance = 100_000u128.saturated_into();

        LiquidityPool::<T>::add_liquidity(
            RawOrigin::Signed(caller.clone()).into(),
            pool_id,
            liquidity,
            liquidity,
            0u32.into(),
            0u32.into(),
        )
        .expect("Failed to add liquidity");

        let amount_in: T::Balance = 1_000u128.saturated_into();
        let path = vec![asset_a, asset_b];

        #[extrinsic_call]
        _(
            RawOrigin::Signed(caller.clone()),
            amount_in,
            0u32.into(),
            path,
        );

        // 验证
        let pool = LiquidityPool::<T>::pools(pool_id).expect("Pool should exist");
        assert!(pool.reserve_a > liquidity);
    }

    #[benchmark]
    fn swap_tokens_for_exact_tokens() {
        let caller: T::AccountId = whitelisted_caller();
        let asset_a: T::AssetId = 1u32.into();
        let asset_b: T::AssetId = 2u32.into();

        // 设置
        setup_asset::<T>(asset_a, &caller, 1_000_000_000).expect("Setup asset A failed");
        setup_asset::<T>(asset_b, &caller, 1_000_000_000).expect("Setup asset B failed");

        // 创建池并添加流动性
        LiquidityPool::<T>::create_pool(RawOrigin::Signed(caller.clone()).into(), asset_a, asset_b)
            .expect("Failed to create pool");

        let pool_id: T::PoolId = 0u32.into();
        let liquidity: T::Balance = 100_000u128.saturated_into();

        LiquidityPool::<T>::add_liquidity(
            RawOrigin::Signed(caller.clone()).into(),
            pool_id,
            liquidity,
            liquidity,
            0u32.into(),
            0u32.into(),
        )
        .expect("Failed to add liquidity");

        let amount_out: T::Balance = 500u128.saturated_into();
        let amount_in_max: T::Balance = 1_000u128.saturated_into();
        let path = vec![asset_a, asset_b];

        #[extrinsic_call]
        _(
            RawOrigin::Signed(caller.clone()),
            amount_out,
            amount_in_max,
            path,
        );

        // 验证
        let pool = LiquidityPool::<T>::pools(pool_id).expect("Pool should exist");
        assert!(pool.reserve_b < liquidity);
    }

    impl_benchmark_test_suite!(
        LiquidityPool,
        crate::mock::new_test_ext(),
        crate::mock::Test
    );
}
