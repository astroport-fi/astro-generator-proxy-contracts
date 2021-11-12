use cosmwasm_std::{Decimal, Uint128};
use cw20::Cw20ReceiveMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    /// Account who can update config
    pub owner: Option<String>,
    /// Contract used to query addresses related to red-bank (WHALE Token)
    pub whale_token: String,
    ///  WHALE-UST LP token address - accepted by the contract via Cw20ReceiveMsg function
    pub staking_token: Option<String>,
    /// Timestamp from which WHALE Rewards will start getting accrued against the staked LP tokens
    pub init_timestamp: u64,
    /// Timestamp till which WHALE Rewards will be accrued. No staking rewards are accrued beyond this timestamp
    pub till_timestamp: u64,
    /// $WHALE Rewards distributed during the 1st cycle.
    pub cycle_rewards: Option<Uint128>,
    /// Cycle duration in timestamps
    pub cycle_duration: u64,
    /// Percent increase in Rewards per cycle
    pub reward_increase: Option<Decimal>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UpdateConfigMsg {
    /// Account who can update config
    pub owner: Option<String>,
    /// Contract used to query addresses related to red-bank (WHALE Token)
    pub address_provider: Option<String>,
    ///  WHALE-UST LP token address - accepted by the contract via Cw20ReceiveMsg function
    pub staking_token: Option<String>,
    /// Timestamp from which WHALE Rewards will start getting accrued against the staked LP tokens
    pub init_timestamp: Option<u64>,
    /// Timestamp till which WHALE Rewards will be accrued. No staking rewards are accrued beyond this timestamp
    pub till_timestamp: Option<u64>,
    /// $WHALE Rewards distributed during the 1st cycle.
    pub cycle_rewards: Option<Uint128>,
    /// Percent increase in Rewards per cycle
    pub reward_increase: Option<Decimal>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Open a new user position or add to an existing position
    /// @dev Increase the total LP shares Bonded by equal no. of shares as sent by the user
    Receive(Cw20ReceiveMsg),
    /// Update data stored in config / state (cycle params)
    /// @param new_config The new config info to be stored    
    UpdateConfig { new_config: UpdateConfigMsg },
    /// Decrease the total LP shares Bonded by the user
    /// Accrued rewards are claimed along-with this function
    /// @param amount The no. of LP shares to be subtracted from the total Bonded and sent back to the user
    Unbond {
        amount: Uint128,
        withdraw_pending_reward: Option<bool>,
    },
    /// Claim pending rewards
    Claim {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Cw20HookMsg {
    /// Open a new user position or add to an existing position (Cw20ReceiveMsg)
    Bond {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Returns the contract configuration
    Config {},
    /// Returns the global state of the contract
    /// @param timestamp Optional value which can be passed to calculate global_reward_index at a certain timestamp
    State { timestamp: Option<u64> },
    /// Returns the state of a user's staked position (StakerInfo)
    /// @param timestamp Optional value which can be passed to calculate reward_index, pending_reward at a certain timestamp
    StakerInfo {
        staker: String,
        timestamp: Option<u64>,
    },
    /// Helper function, returns the current timestamp
    Timestamp {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    /// Account who can update config
    pub owner: String,
    /// Contract used to query addresses related to red-bank
    pub address_provider: String,
    ///  $WHALE token address
    pub whale_token: String,
    ///  WHALE-UST LP token address
    pub staking_token: String,
    /// Timestamp from which WHALE Rewards will start getting accrued against the staked LP tokens
    pub init_timestamp: u64,
    /// Timestamp till which WHALE Rewards will be accrued. No staking rewards are accrued beyond this timestamp      
    pub till_timestamp: u64,
    /// Cycle duration in timestamps         
    pub cycle_duration: u64,
    /// Percent increase in Rewards per cycle
    pub reward_increase: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StateResponse {
    /// Timestamp at which the current reward cycle begin
    pub current_cycle: u64,
    /// WHALE rewards to be distributed in the current cycle
    pub current_cycle_rewards: Uint128,
    /// Timestamp at which the global_reward_index was last updated
    pub last_distributed: u64,
    /// Total number of WHALE-UST LP tokens deposited in the contract
    pub total_bond_amount: Uint128,
    ///  total WHALE rewards / total_bond_amount ratio. Used to calculate WHALE rewards accured over time elapsed
    pub global_reward_index: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StakerInfoResponse {
    /// User address
    pub staker: String,
    /// WHALE-UST LP tokens deposited by the user
    pub bond_amount: Uint128,
    /// WHALE rewards / bond_amount ratio.  Used to calculate WHALE rewards accured over time elapsed
    pub reward_index: Decimal,
    /// Pending WHALE rewards which are yet to be claimed
    pub pending_reward: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TimeResponse {
    /// Current timestamp
    pub timestamp: u64,
}
