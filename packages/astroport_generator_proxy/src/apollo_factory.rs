use cosmwasm_bignumber::{Decimal256, Uint256};
use cosmwasm_std::{Addr, Binary, Decimal, Uint128};
use cw20::Cw20ReceiveMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(deny_unknown_fields)]
pub enum ExecuteMsg {
    AddStrategy {
        strategy: String,
        is_apollo: bool,
        receives_rewards: bool,
    },
    RemoveStrategy {
        strategy_id: u64,
    },
    UpdateStrategy {
        strategy_id: u64,
        address: Option<String>,
        execution_paused: Option<bool>,
        deposits_paused: Option<bool>,
        withdrawals_paused: Option<bool>,
    },
    ExecuteStrategy {
        strategy_id: u64,
    },
    WithdrawFromStrategy {
        strategy_id: u64,
        amount: Uint128,
    },
    Receive(Cw20ReceiveMsg),
    EmergencyWithdraw {
        strategy_id: u64,
    },
    UpdateConfig {
        owner: Option<String>,
        warchest: Option<String>,
        distribution_schedule: Option<Vec<(u64, u64, Uint128)>>,
        genesis_time: Option<u64>,
        oracle: Option<String>,
        apollo_token: Option<String>,
        apollo_reward_percentage: Option<Decimal>,
    },
    ZapIntoStrategy {
        strategy_id: u64,
    },
    ZapOutOfStrategy {
        strategy_id: u64,
        amount: Uint128,
    },
    RegisterFee {
        amount: Uint128,
    },
    PassMessage {
        contract_addr: String,
        msg: Binary,
    },
    UpdateRewardWeights {},
    ClaimRewards {
        strategy_id: Option<u64>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(deny_unknown_fields)]
pub enum QueryMsg {
    GetStrategies {
        limit: Option<u32>,
        start_from: Option<u64>,
    },
    GetStrategy {
        id: u64,
    },
    GetUserStrategies {
        user: String,
        limit: Option<u32>,
        start_from: Option<u64>,
    },
    GetConfig {},
    GetStrategyTvl {
        id: u64,
    },
    GetTotalTvl {},
    GetTotalCollectedFees {},
    GetExtensionTotalCollectedFees {},
    GetTotalRewardWeight {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct FactoryStrategyInfoResponse {
    pub id: u64,
    pub address: Addr,
    pub deprecated: bool,
    pub global_index: Decimal,
    pub execution_paused: bool,
    pub withdrawals_paused: bool,
    pub deposits_paused: bool,
    pub total_bond_amount: Uint128,
    pub base_token: Addr,
    pub tvl: Uint128,
    pub performance_fee: Decimal,
    pub reward_index: Decimal256,
    pub extension_reward_index: Decimal256,
    pub lm_reward_index: Decimal256,
    pub last_distributed: u64,
    pub total_shares: Uint128,
    pub reward_weight: Uint256,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct GetStrategiesResponse {
    pub strategies: Vec<FactoryStrategyInfoResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct FactoryUserInfoResponse {
    pub id: u64,
    pub base_token_balance: Uint128,
    pub pending_reward: Uint128,
    pub extension_pending_reward: Uint128,
    pub reward_index: Decimal256,
    pub extension_reward_index: Decimal256,
    pub shares: Uint128,
    pub lm_pending_reward: Uint128,
    pub lm_reward_index: Decimal256,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct GetUserStrategiesResponse {
    pub strategies: Vec<FactoryUserInfoResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct GetConfigResponse {
    pub owner: Addr,
    pub warchest: Addr,
    pub distribution_schedule: Vec<(u64, u64, Uint128)>,
    pub genesis_time: u64,
    pub oracle: Addr,
    pub apollo_token: Addr,
    pub apollo_reward_percentage: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct GetTvlResponse {
    pub tvl: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct GetTotalCollectedFeesResponse {
    pub total_collected_fees: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct GetExtensionTotalCollectedFeesResponse {
    pub extension_total_collected_fees: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct GetTotalRewardWeightResponse {
    pub total_reward_weight: Uint256,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(deny_unknown_fields)]
pub enum Cw20HookMsg {
    Deposit { strategy_id: u64 },
}
