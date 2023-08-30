use cosmwasm_schema::{cw_serde, QueryResponses};

use cosmwasm_std::{Decimal, Uint128};
use cw20::Cw20ReceiveMsg;

#[cw_serde]
pub struct InstantiateMsg {
    pub token: String,
    pub pair: String,
    pub lp_token: String,
    pub whitelisted_contracts: Vec<String>,
}

#[cw_serde]
pub struct ConfigResponse {
    pub admin: String,
    pub token: String,
    pub pair: String,
    pub lp_token: String,
    pub whitelisted_contracts: Vec<String>,
    pub distribution_schedule: Vec<(u64, u64, Uint128)>,
}

#[cw_serde]
pub struct StateResponse {
    pub last_distributed: u64,
    pub total_bond_amount: Uint128,
    pub global_reward_index: Decimal,
}

#[cw_serde]
pub struct StakerInfoResponse {
    pub staker: String,
    pub reward_index: Decimal,
    pub bond_amount: Uint128,
    pub pending_reward: Uint128,
}

#[cw_serde]
pub enum Cw20HookMsg {
    Bond { schedules: Vec<(u64, u64, Uint128)> },
}

/// We currently take no arguments for migrations
#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Receive(Cw20ReceiveMsg),
    Unbond {
        amount: Uint128,
    },
    /// Withdraw pending rewards
    Withdraw {},
    UpdateConfig {
        token: Option<String>,
        pair: Option<String>,
        lp_token: Option<String>,
        admin: Option<String>,
        whitelisted_contracts: Option<Vec<String>>,
    },
    MigrateReward {
        recipient: String,
        amount: Uint128,
    },
    ApproveAdminNominee {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[returns(StateResponse)]
    State { block_height: Option<u64> },
    #[returns(StakerInfoResponse)]
    StakerInfo { staker: String },
}
