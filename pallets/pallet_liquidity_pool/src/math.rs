use frame::traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Zero};
use scale_info::prelude::cmp::min;

/// 恒定乘积做市商算法 (x * y = k)
pub struct ConstantProductMath;

impl ConstantProductMath {
    /// 计算输出金额 (给定输入)
    /// formula: amount_out = (amount_in * 997 * reserve_out) / (reserve_in * 1000 + amount_in * 997)
    /// 997/1000 = 0.3% 手续费
    pub fn get_amount_out<Balance>(
        amount_in: Balance,
        reserve_in: Balance,
        reserve_out: Balance,
        fee_rate: u32, // 基点 (30 = 0.3%)
    ) -> Option<Balance>
    where
        Balance: CheckedAdd
            + CheckedSub
            + CheckedMul
            + CheckedDiv
            + Copy
            + Zero
            + From<u32>
            + PartialOrd,
    {
        if amount_in.is_zero() || reserve_in.is_zero() || reserve_out.is_zero() {
            return None;
        }

        // 扣除手续费:  amount_in_with_fee = amount_in * (10000 - fee_rate)
        let fee_denominator: Balance = 10000u32.into();
        let fee_numerator: Balance = fee_denominator.checked_sub(&fee_rate.into())?;

        let amount_in_with_fee = amount_in.checked_mul(&fee_numerator)?;
        let numerator = amount_in_with_fee.checked_mul(&reserve_out)?;
        let denominator = reserve_in
            .checked_mul(&fee_denominator)?
            .checked_add(&amount_in_with_fee)?;

        numerator.checked_div(&denominator)
    }

    /// 计算输入金额 (给定输出)
    /// formula: amount_in = (reserve_in * amount_out * 1000) / ((reserve_out - amount_out) * 997) + 1
    pub fn get_amount_in<Balance>(
        amount_out: Balance,
        reserve_in: Balance,
        reserve_out: Balance,
        fee_rate: u32,
    ) -> Option<Balance>
    where
        Balance: CheckedAdd
            + CheckedSub
            + CheckedMul
            + CheckedDiv
            + Copy
            + Zero
            + From<u32>
            + PartialOrd,
    {
        if amount_out.is_zero() || reserve_in.is_zero() || reserve_out.is_zero() {
            return None;
        }

        if amount_out >= reserve_out {
            return None; // 输出超过储备
        }

        let fee_denominator: Balance = 10000u32.into();
        let fee_numerator: Balance = fee_denominator.checked_sub(&fee_rate.into())?;

        let numerator = reserve_in
            .checked_mul(&amount_out)?
            .checked_mul(&fee_denominator)?;
        let denominator = reserve_out
            .checked_sub(&amount_out)?
            .checked_mul(&fee_numerator)?;

        let amount_in = numerator.checked_div(&denominator)?;
        // 向上取整
        amount_in.checked_add(&1u32.into())
    }

    /// 计算添加流动性时的最优比例
    pub fn quote<Balance>(
        amount_a: Balance,
        reserve_a: Balance,
        reserve_b: Balance,
    ) -> Option<Balance>
    where
        Balance: CheckedMul + CheckedDiv + Copy + Zero + PartialOrd,
    {
        if amount_a.is_zero() || reserve_a.is_zero() || reserve_b.is_zero() {
            return None;
        }

        amount_a.checked_mul(&reserve_b)?.checked_div(&reserve_a)
    }

    /// 计算初始 LP Token 数量 (首次添加流动性)
    pub fn calculate_initial_lp<Balance>(amount_a: Balance, amount_b: Balance) -> Option<Balance>
    where
        Balance: CheckedMul
            + CheckedAdd
            + CheckedDiv
            + CheckedSub
            + Copy
            + PartialOrd
            + From<u32>
            + Zero,
    {
        // sqrt(amount_a * amount_b)
        let product = amount_a.checked_mul(&amount_b)?;
        Self::sqrt(product)
    }

    /// 计算后续 LP Token 数量
    pub fn calculate_lp_amount<Balance>(
        amount_a: Balance,
        amount_b: Balance,
        reserve_a: Balance,
        reserve_b: Balance,
        total_lp: Balance,
    ) -> Option<Balance>
    where
        Balance: CheckedMul + CheckedDiv + Copy + Zero + PartialOrd + Ord,
    {
        if reserve_a.is_zero() || reserve_b.is_zero() || total_lp.is_zero() {
            return None;
        }

        let lp_from_a = amount_a.checked_mul(&total_lp)?.checked_div(&reserve_a)?;
        let lp_from_b = amount_b.checked_mul(&total_lp)?.checked_div(&reserve_b)?;

        Some(min(lp_from_a, lp_from_b))
    }

    /// 简单平方根实现 (牛顿法)
    fn sqrt<Balance>(x: Balance) -> Option<Balance>
    where
        Balance: CheckedAdd + CheckedDiv + CheckedSub + Copy + PartialOrd + From<u32> + Zero,
    {
        if x.is_zero() {
            return Some(x);
        }

        let mut z = x;
        let mut y = x.checked_add(&1u32.into())?.checked_div(&2u32.into())?;

        while y < z {
            z = y;
            y = x
                .checked_div(&z)?
                .checked_add(&z)?
                .checked_div(&2u32.into())?;
        }

        Some(z)
    }
}
