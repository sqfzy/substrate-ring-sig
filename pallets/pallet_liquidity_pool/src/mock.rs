use crate as pallet_liquidity_pool;
use crate::*;
use frame::runtime::prelude::*;
use frame::traits::{AsEnsureOriginWithArg, BlakeTwo256, ConstU128, ConstU32, IdentityLookup};

#[cfg(test)]
pub use tests::*;

// 测试常量
pub const ALICE: u64 = 1;
pub const BOB: u64 = 2;
pub const CHARLIE: u64 = 3;

pub const ASSET_ID_A: u32 = 1;
pub const ASSET_ID_B: u32 = 2;
pub const ASSET_ID_C: u32 = 3;

pub const INITIAL_BALANCE: u128 = 1_000_000_000;
pub const INITIAL_LIQUIDITY: u128 = 100_000;

#[cfg(test)]
pub mod tests {
    use super::*;
    use frame::testing_prelude::*;

    pub type Block = frame_system::mocking::MockBlock<Test>;
    pub type Balance = u128;
    pub type AssetId = u32;

    // 构建测试运行时
    #[frame_construct_runtime]
    mod runtime {
        #[runtime::runtime]
        #[runtime::derive(
            RuntimeCall,
            RuntimeEvent,
            RuntimeError,
            RuntimeOrigin,
            RuntimeFreezeReason,
            RuntimeHoldReason,
            RuntimeSlashReason,
            RuntimeLockId,
            RuntimeTask
        )]
        pub struct Test;

        #[runtime::pallet_index(0)]
        pub type System = frame_system;

        #[runtime::pallet_index(1)]
        pub type Balances = pallet_balances;

        #[runtime::pallet_index(2)]
        pub type Assets = pallet_assets;

        #[runtime::pallet_index(3)]
        pub type LiquidityPool = pallet_liquidity_pool;
    }

    // ========== 参数配置 ==========
    parameter_types! {
        pub const LiquidityPoolPalletId: PalletId = PalletId(*b"liqd/pol");
        pub const MinimumLiquidity: Balance = 1_000;
        pub const DefaultFeeRate: u32 = 30; // 0.3%
        pub const MaxPathLength: u32 = 4;
    }

    // ========== System 配置 ==========
    #[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
    impl frame_system::Config for Test {
        type Block = Block;
        type AccountData = pallet_balances::AccountData<Balance>;
    }

    // ========== Balances 配置 ==========
    impl pallet_balances::Config for Test {
        type RuntimeEvent = RuntimeEvent;
        type RuntimeFreezeReason = RuntimeFreezeReason;
        type WeightInfo = ();
        type Balance = Balance;
        type DustRemoval = ();
        type ExistentialDeposit = ConstU128<1>;
        type AccountStore = System;
        type ReserveIdentifier = [u8; 8];
        type RuntimeHoldReason = RuntimeHoldReason;
        type FreezeIdentifier = ();
        type MaxLocks = ConstU32<50>;
        type MaxReserves = ConstU32<50>;
        type MaxFreezes = ConstU32<50>;
        type DoneSlashHandler = ();
    }

    // ========== Assets 配置 ==========
    impl pallet_assets::Config for Test {
        type RuntimeEvent = RuntimeEvent;
        type Balance = Balance;
        type AssetId = AssetId;
        type AssetIdParameter = u32;
        type Currency = Balances;
        type CreateOrigin = AsEnsureOriginWithArg<frame_system::EnsureSigned<u64>>;
        type ForceOrigin = frame_system::EnsureRoot<u64>;
        type AssetDeposit = ConstU128<100>;
        type AssetAccountDeposit = ConstU128<10>;
        type MetadataDepositBase = ConstU128<10>;
        type MetadataDepositPerByte = ConstU128<1>;
        type ApprovalDeposit = ConstU128<1>;
        type StringLimit = ConstU32<50>;
        type Freezer = ();
        type Extra = ();
        type WeightInfo = ();
        type RemoveItemsLimit = ConstU32<1000>;
        type CallbackHandle = ();
        type Holder = ();
    }

    // ========== LiquidityPool 配置 ==========
    impl pallet_liquidity_pool::Config for Test {
        type RuntimeEvent = RuntimeEvent;
        type AssetId = AssetId;
        type Balance = Balance;
        type PoolId = u32;
        type Assets = Assets;
        type PalletId = LiquidityPoolPalletId;
        type MinimumLiquidity = MinimumLiquidity;
        type DefaultFeeRate = DefaultFeeRate;
        type MaxPathLength = MaxPathLength;
    }

    // ========== 测试环境初始化 ==========
    pub fn new_test_ext() -> TestExternalities {
        let mut storage = frame_system::GenesisConfig::<Test>::default()
            .build_storage()
            .unwrap();

        pallet_balances::GenesisConfig::<Test> {
            balances: vec![
                (ALICE, INITIAL_BALANCE),
                (BOB, INITIAL_BALANCE),
                (CHARLIE, INITIAL_BALANCE),
            ],
            dev_accounts: None,
        }
        .assimilate_storage(&mut storage)
        .unwrap();

        let mut ext = TestExternalities::new(storage);
        ext.execute_with(|| {
            System::set_block_number(1);
        });
        ext
    }

    // ========== Helper 函数 ==========

    /// 创建资产并铸造给账户
    pub fn create_asset(asset_id: AssetId, owner: u64, amount: Balance) -> DispatchResult {
        Assets::force_create(RuntimeOrigin::root(), asset_id, owner, true, 1)?;
        Assets::mint(RuntimeOrigin::signed(owner), asset_id, owner, amount)?;
        Ok(())
    }

    /// 为多个账户创建资产
    pub fn create_asset_for_accounts(
        asset_id: AssetId,
        owner: u64,
        accounts: Vec<(u64, Balance)>,
    ) -> DispatchResult {
        Assets::force_create(RuntimeOrigin::root(), asset_id, owner, true, 1)?;
        for (account, amount) in accounts {
            Assets::mint(RuntimeOrigin::signed(owner), asset_id, account, amount)?;
        }
        Ok(())
    }

    /// 创建流动性池
    pub fn create_pool(
        origin: RuntimeOrigin,
        token_a: AssetId,
        token_b: AssetId,
    ) -> Result<u32, DispatchError> {
        let pool_id = LiquidityPool::next_pool_id();
        LiquidityPool::create_pool(origin, token_a, token_b)?;
        Ok(pool_id)
    }

    /// 添加流动性
    pub fn add_liquidity(
        origin: RuntimeOrigin,
        pool_id: u32,
        amount_a: Balance,
        amount_b: Balance,
    ) -> DispatchResult {
        LiquidityPool::add_liquidity(origin, pool_id, amount_a, amount_b, 0, 0)
    }

    /// 移除流动性
    pub fn remove_liquidity(
        origin: RuntimeOrigin,
        pool_id: u32,
        lp_tokens: Balance,
    ) -> DispatchResult {
        LiquidityPool::remove_liquidity(origin, pool_id, lp_tokens, 0, 0)
    }

    /// 执行交易
    pub fn swap_exact_tokens(
        origin: RuntimeOrigin,
        amount_in: Balance,
        amount_out_min: Balance,
        path: Vec<AssetId>,
    ) -> DispatchResult {
        LiquidityPool::swap_exact_tokens_for_tokens(origin, amount_in, amount_out_min, path)
    }

    /// 设置完整的池（创建资产 + 创建池 + 添加流动性）
    pub fn setup_pool(
        token_a: AssetId,
        token_b: AssetId,
        amount_a: Balance,
        amount_b: Balance,
    ) -> Result<u32, DispatchError> {
        // 创建资产
        create_asset(token_a, ALICE, INITIAL_BALANCE)?;
        create_asset(token_b, ALICE, INITIAL_BALANCE)?;

        // 创建池
        let pool_id = create_pool(RuntimeOrigin::signed(ALICE), token_a, token_b)?;

        // 添加流动性
        add_liquidity(RuntimeOrigin::signed(ALICE), pool_id, amount_a, amount_b)?;

        Ok(pool_id)
    }

    /// 断言池状态
    pub fn assert_pool_reserves(
        pool_id: u32,
        expected_reserve_a: Balance,
        expected_reserve_b: Balance,
    ) {
        let pool = LiquidityPool::pools(pool_id).expect("Pool should exist");
        assert_eq!(pool.reserve_a, expected_reserve_a, "Reserve A mismatch");
        assert_eq!(pool.reserve_b, expected_reserve_b, "Reserve B mismatch");
    }

    /// 断言 LP Token 余额
    pub fn assert_lp_balance(pool_id: u32, account: u64, expected_balance: Balance) {
        let position = LiquidityPool::lp_positions(pool_id, account);
        match position {
            Some(pos) => assert_eq!(pos.lp_amount, expected_balance, "LP balance mismatch"),
            None => assert_eq!(expected_balance, 0, "Expected no LP position"),
        }
    }

    /// 断言资产余额
    pub fn assert_asset_balance(asset_id: AssetId, account: u64, expected_balance: Balance) {
        let balance = Assets::balance(asset_id, account);
        assert_eq!(
            balance, expected_balance,
            "Asset balance mismatch for asset {}",
            asset_id
        );
    }

    /// 获取账户资产余额
    pub fn get_asset_balance(asset_id: AssetId, account: u64) -> Balance {
        Assets::balance(asset_id, account)
    }
}
