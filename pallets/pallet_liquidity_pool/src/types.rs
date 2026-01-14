use codec::{Decode, Encode, MaxEncodedLen};
use frame::prelude::*;
use scale_info::prelude::vec::Vec;
use scale_info::TypeInfo;

/// 流动性池信息
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct PoolInfo<AssetId, Balance> {
    /// 代币 A
    pub token_a: AssetId,
    /// 代币 B
    pub token_b: AssetId,
    /// 代币 A 储备量
    pub reserve_a: Balance,
    /// 代币 B 储备量
    pub reserve_b: Balance,
    /// LP Token 总供应量
    pub total_lp_supply: Balance,
    /// 手续费率 (基点, 30 = 0.3%)
    pub fee_rate: u32,
    /// 池是否激活
    pub is_active: bool,
}

/// LP Token 持有信息
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct LpPosition<Balance> {
    /// LP Token 数量
    pub lp_amount: Balance,
    /// 最后更新区块
    pub last_update: u32,
}

/// 交易路径
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct TradePath<AssetId> {
    /// 资产路径
    pub path: Vec<AssetId>,
}
