use crate::{mock::*, Error, Event};
use frame::testing_prelude::*;

// ==================== 池创建测试 ====================

#[test]
fn create_pool_works() {
    new_test_ext().execute_with(|| {
        // 准备资产
        assert_ok!(create_asset(ASSET_ID_A, ALICE, INITIAL_BALANCE));
        assert_ok!(create_asset(ASSET_ID_B, ALICE, INITIAL_BALANCE));

        // 创建池
        assert_ok!(create_pool(
            RuntimeOrigin::signed(ALICE),
            ASSET_ID_A,
            ASSET_ID_B
        ));

        // 验证
        assert_eq!(LiquidityPool::next_pool_id(), 1);
        assert!(LiquidityPool::pools(0).is_some());
        assert!(LiquidityPool::trading_pairs((ASSET_ID_A, ASSET_ID_B)).is_some());

        System::assert_last_event(
            Event::PoolCreated {
                pool_id: 0,
                token_a: ASSET_ID_A,
                token_b: ASSET_ID_B,
                creator: ALICE,
            }
            .into(),
        );
    });
}

#[test]
fn create_pool_normalizes_token_order() {
    new_test_ext().execute_with(|| {
        assert_ok!(create_asset(ASSET_ID_A, ALICE, INITIAL_BALANCE));
        assert_ok!(create_asset(ASSET_ID_B, ALICE, INITIAL_BALANCE));

        // 用反向顺序创建
        assert_ok!(create_pool(
            RuntimeOrigin::signed(ALICE),
            ASSET_ID_B,
            ASSET_ID_A
        ));

        // 应该被规范化为 A < B
        let pool = LiquidityPool::pools(0).unwrap();
        assert_eq!(pool.token_a, ASSET_ID_A);
        assert_eq!(pool.token_b, ASSET_ID_B);
    });
}

#[test]
fn create_pool_fails_with_identical_assets() {
    new_test_ext().execute_with(|| {
        assert_ok!(create_asset(ASSET_ID_A, ALICE, INITIAL_BALANCE));

        assert_noop!(
            create_pool(RuntimeOrigin::signed(ALICE), ASSET_ID_A, ASSET_ID_A),
            Error::<Test>::IdenticalAssets
        );
    });
}

#[test]
fn create_pool_fails_when_already_exists() {
    new_test_ext().execute_with(|| {
        assert_ok!(create_asset(ASSET_ID_A, ALICE, INITIAL_BALANCE));
        assert_ok!(create_asset(ASSET_ID_B, ALICE, INITIAL_BALANCE));

        assert_ok!(create_pool(
            RuntimeOrigin::signed(ALICE),
            ASSET_ID_A,
            ASSET_ID_B
        ));

        // 尝试再次创建
        assert_noop!(
            create_pool(RuntimeOrigin::signed(ALICE), ASSET_ID_A, ASSET_ID_B),
            Error::<Test>::PoolAlreadyExists
        );
    });
}

// ==================== 添加流动性测试 ====================

#[test]
fn add_initial_liquidity_works() {
    new_test_ext().execute_with(|| {
        // 设置池
        assert_ok!(create_asset(ASSET_ID_A, ALICE, INITIAL_BALANCE));
        assert_ok!(create_asset(ASSET_ID_B, ALICE, INITIAL_BALANCE));
        let pool_id = create_pool(RuntimeOrigin::signed(ALICE), ASSET_ID_A, ASSET_ID_B).unwrap();

        let amount_a = 100_000;
        let amount_b = 100_000;

        // 添加流动性
        assert_ok!(add_liquidity(
            RuntimeOrigin::signed(ALICE),
            pool_id,
            amount_a,
            amount_b
        ));

        // 验证池储备
        assert_pool_reserves(pool_id, amount_a, amount_b);

        // 验证 LP Token
        let pool = LiquidityPool::pools(pool_id).unwrap();
        assert!(pool.total_lp_supply > 0);

        System::assert_last_event(
            Event::LiquidityAdded {
                who: ALICE,
                pool_id,
                amount_a,
                amount_b,
                lp_tokens: pool.total_lp_supply,
            }
            .into(),
        );
    });
}

#[test]
fn add_liquidity_maintains_ratio() {
    new_test_ext().execute_with(|| {
        // 设置池并添加初始流动性 (这会创建资产并给 ALICE mint)
        let pool_id = setup_pool(ASSET_ID_A, ASSET_ID_B, 100_000, 200_000).unwrap();

        // BOB 添加流动性 - 直接 mint 给 BOB，不要重复创建资产
        assert_ok!(Assets::mint(
            RuntimeOrigin::signed(ALICE), // ALICE 是资产的 owner
            ASSET_ID_A,
            BOB,
            INITIAL_BALANCE
        ));
        assert_ok!(Assets::mint(
            RuntimeOrigin::signed(ALICE),
            ASSET_ID_B,
            BOB,
            INITIAL_BALANCE
        ));

        // BOB 添加流动性时应该维持比例 1: 2
        assert_ok!(add_liquidity(
            RuntimeOrigin::signed(BOB),
            pool_id,
            50_000,
            100_000
        ));

        // 验证储备增加
        assert_pool_reserves(pool_id, 150_000, 300_000);
    });
}

#[test]
fn add_liquidity_fails_with_zero_amount() {
    new_test_ext().execute_with(|| {
        let pool_id = setup_pool(ASSET_ID_A, ASSET_ID_B, 100_000, 100_000).unwrap();

        assert_noop!(
            add_liquidity(RuntimeOrigin::signed(ALICE), pool_id, 0, 100_000),
            Error::<Test>::ZeroAmount
        );

        assert_noop!(
            add_liquidity(RuntimeOrigin::signed(ALICE), pool_id, 100_000, 0),
            Error::<Test>::ZeroAmount
        );
    });
}

#[test]
fn add_liquidity_fails_with_insufficient_minimum() {
    new_test_ext().execute_with(|| {
        assert_ok!(create_asset(ASSET_ID_A, ALICE, INITIAL_BALANCE));
        assert_ok!(create_asset(ASSET_ID_B, ALICE, INITIAL_BALANCE));
        let pool_id = create_pool(RuntimeOrigin::signed(ALICE), ASSET_ID_A, ASSET_ID_B).unwrap();

        // 添加太少的流动性
        assert_noop!(
            add_liquidity(RuntimeOrigin::signed(ALICE), pool_id, 10, 10),
            Error::<Test>::InsufficientMinimumLiquidity
        );
    });
}

// ==================== 移除流动性测试 ====================

#[test]
fn remove_liquidity_works() {
    new_test_ext().execute_with(|| {
        let pool_id = setup_pool(ASSET_ID_A, ASSET_ID_B, 100_000, 100_000).unwrap();

        let pool = LiquidityPool::pools(pool_id).unwrap();
        let lp_tokens = pool.total_lp_supply / 2; // 移除一半

        let balance_a_before = get_asset_balance(ASSET_ID_A, ALICE);
        let balance_b_before = get_asset_balance(ASSET_ID_B, ALICE);

        // 移除流动性
        assert_ok!(remove_liquidity(
            RuntimeOrigin::signed(ALICE),
            pool_id,
            lp_tokens
        ));

        // 验证余额增加
        let balance_a_after = get_asset_balance(ASSET_ID_A, ALICE);
        let balance_b_after = get_asset_balance(ASSET_ID_B, ALICE);
        assert!(balance_a_after > balance_a_before);
        assert!(balance_b_after > balance_b_before);

        System::assert_has_event(
            Event::LiquidityRemoved {
                who: ALICE,
                pool_id,
                lp_tokens,
                amount_a: balance_a_after - balance_a_before,
                amount_b: balance_b_after - balance_b_before,
            }
            .into(),
        );
    });
}

#[test]
fn remove_liquidity_fails_with_insufficient_lp_tokens() {
    new_test_ext().execute_with(|| {
        let pool_id = setup_pool(ASSET_ID_A, ASSET_ID_B, 100_000, 100_000).unwrap();

        // BOB 没有 LP Token
        assert_noop!(
            remove_liquidity(RuntimeOrigin::signed(BOB), pool_id, 1000),
            Error::<Test>::InsufficientLpTokens
        );
    });
}

// ==================== 交易测试 ====================

#[test]
fn swap_exact_tokens_for_tokens_works() {
    new_test_ext().execute_with(|| {
        let pool_id = setup_pool(ASSET_ID_A, ASSET_ID_B, 100_000, 100_000).unwrap();

        // 给 BOB 资产
        assert_ok!(Assets::mint(
            RuntimeOrigin::signed(ALICE),
            ASSET_ID_A,
            BOB,
            10_000
        ));

        let amount_in = 1_000;
        let path = vec![ASSET_ID_A, ASSET_ID_B];

        let balance_b_before = get_asset_balance(ASSET_ID_B, BOB);

        // 执行交易
        assert_ok!(swap_exact_tokens(
            RuntimeOrigin::signed(BOB),
            amount_in,
            0,
            path.clone()
        ));

        // 验证余额变化
        let balance_b_after = get_asset_balance(ASSET_ID_B, BOB);
        assert!(balance_b_after > balance_b_before);

        System::assert_has_event(
            Event::Swap {
                who: BOB,
                path,
                amount_in,
                amount_out: balance_b_after - balance_b_before,
            }
            .into(),
        );
    });
}

#[test]
fn swap_fails_with_insufficient_output() {
    new_test_ext().execute_with(|| {
        setup_pool(ASSET_ID_A, ASSET_ID_B, 100_000, 100_000).unwrap();

        assert_ok!(Assets::mint(
            RuntimeOrigin::signed(ALICE),
            ASSET_ID_A,
            BOB,
            10_000
        ));

        let path = vec![ASSET_ID_A, ASSET_ID_B];

        // 设置不可能达到的最小输出
        assert_noop!(
            swap_exact_tokens(RuntimeOrigin::signed(BOB), 1_000, 1_000_000, path),
            Error::<Test>::SlippageExceeded
        );
    });
}

#[test]
fn swap_fails_with_invalid_path() {
    new_test_ext().execute_with(|| {
        setup_pool(ASSET_ID_A, ASSET_ID_B, 100_000, 100_000).unwrap();

        // 路径太短
        assert_noop!(
            swap_exact_tokens(RuntimeOrigin::signed(ALICE), 1_000, 0, vec![ASSET_ID_A]),
            Error::<Test>::InvalidPath
        );

        // 不存在的池
        assert_noop!(
            swap_exact_tokens(
                RuntimeOrigin::signed(ALICE),
                1_000,
                0,
                vec![ASSET_ID_A, ASSET_ID_C]
            ),
            Error::<Test>::PoolNotFound
        );
    });
}

#[test]
fn swap_multi_hop_works() {
    new_test_ext().execute_with(|| {
        // 创建池 A-B
        let pool_id_1 = setup_pool(ASSET_ID_A, ASSET_ID_B, 100_000, 100_000).unwrap();

        // 创建资产 C 并创建池 B-C
        assert_ok!(create_asset(ASSET_ID_C, ALICE, INITIAL_BALANCE));
        let pool_id_2 = create_pool(RuntimeOrigin::signed(ALICE), ASSET_ID_B, ASSET_ID_C).unwrap();
        assert_ok!(add_liquidity(
            RuntimeOrigin::signed(ALICE),
            pool_id_2,
            100_000,
            100_000
        ));

        // 给 BOB 资产 A
        assert_ok!(Assets::mint(
            RuntimeOrigin::signed(ALICE),
            ASSET_ID_A,
            BOB,
            10_000
        ));

        let path = vec![ASSET_ID_A, ASSET_ID_B, ASSET_ID_C];
        let balance_c_before = get_asset_balance(ASSET_ID_C, BOB);

        // 执行多跳交易
        assert_ok!(swap_exact_tokens(
            RuntimeOrigin::signed(BOB),
            1_000,
            0,
            path
        ));

        // 验证最终获得了 C
        let balance_c_after = get_asset_balance(ASSET_ID_C, BOB);
        assert!(balance_c_after > balance_c_before);
    });
}

// ==================== 数学函数测试 ====================

#[test]
fn get_amounts_out_works() {
    new_test_ext().execute_with(|| {
        setup_pool(ASSET_ID_A, ASSET_ID_B, 100_000, 100_000).unwrap();

        let path = vec![ASSET_ID_A, ASSET_ID_B];
        let amount_in = 1_000;

        let amounts = LiquidityPool::get_amounts_out(amount_in, &path).unwrap();

        assert_eq!(amounts.len(), 2);
        assert_eq!(amounts[0], amount_in);
        assert!(amounts[1] > 0);
        assert!(amounts[1] < amount_in); // 由于手续费，输出应该少于输入
    });
}

#[test]
fn constant_product_formula_holds() {
    new_test_ext().execute_with(|| {
        let initial_a = 100_000;
        let initial_b = 100_000;
        let pool_id = setup_pool(ASSET_ID_A, ASSET_ID_B, initial_a, initial_b).unwrap();

        let k_before = initial_a * initial_b;

        // 执行交易
        assert_ok!(Assets::mint(
            RuntimeOrigin::signed(ALICE),
            ASSET_ID_A,
            BOB,
            10_000
        ));
        assert_ok!(swap_exact_tokens(
            RuntimeOrigin::signed(BOB),
            1_000,
            0,
            vec![ASSET_ID_A, ASSET_ID_B]
        ));

        let pool = LiquidityPool::pools(pool_id).unwrap();
        let k_after = pool.reserve_a * pool.reserve_b;

        // K 应该增加（由于手续费）
        assert!(k_after >= k_before);
    });
}
