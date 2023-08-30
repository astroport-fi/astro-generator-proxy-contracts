use crate::errors::ContractError;
#[cfg(not(feature = "library"))]
use crate::executions::{bond, migrate_reward, unbond, update_config, withdraw};
use crate::queries::{query_config, query_staker_info, query_state};
use crate::states::{Config, State};
use crate::ContractResult;
use ap_valkyrie::staking_vkr::{Cw20HookMsg, ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    from_binary, to_binary, Binary, Decimal, Deps, DepsMut, Env, MessageInfo, Response, Uint128,
};
use cw20::Cw20ReceiveMsg;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> ContractResult<Response> {
    Config {
        admin: info.sender,
        token: deps.api.addr_validate(&msg.token)?,
        usdc_token: deps.api.addr_validate(&msg.token)?,
        pair: deps.api.addr_validate(&msg.pair)?,
        lp_token: deps.api.addr_validate(&msg.lp_token)?,
        whitelisted_contracts: msg
            .whitelisted_contracts
            .iter()
            .map(|item| deps.api.addr_validate(item).unwrap())
            .collect(),
        distribution_schedule: vec![],
    }
    .save(deps.storage)?;

    State {
        last_distributed: env.block.height,
        total_bond_amount: Uint128::zero(),
        global_reward_index: Decimal::zero(),
    }
    .save(deps.storage)?;

    Ok(Response::new().add_attribute("action", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> ContractResult<Response> {
    match msg {
        ExecuteMsg::Receive(msg) => receive_cw20(deps, env, info, msg),
        ExecuteMsg::Unbond { amount } => unbond(deps, env, info, amount),
        ExecuteMsg::Withdraw {} => withdraw(deps, env, info),
        ExecuteMsg::UpdateConfig {
            token,
            pair,
            lp_token,
            admin,
            whitelisted_contracts,
        } => update_config(
            deps,
            info,
            token,
            pair,
            lp_token,
            admin,
            whitelisted_contracts,
        ),
        ExecuteMsg::MigrateReward { recipient, amount } => {
            migrate_reward(deps, env, info, recipient, amount)
        }
        ExecuteMsg::ApproveAdminNominee {} => crate::executions::approve_admin_nominee(deps, info),
    }
}

pub fn receive_cw20(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    cw20_msg: Cw20ReceiveMsg,
) -> ContractResult<Response> {
    let mut config: Config = Config::load(deps.storage)?;

    // only staking token contract can execute this message
    if config.lp_token != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    match from_binary(&cw20_msg.msg)? {
        Cw20HookMsg::Bond { schedules } => bond(
            deps,
            env,
            &mut config,
            &cw20_msg.sender,
            schedules,
            cw20_msg.amount,
        ),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> ContractResult<Binary> {
    let result = match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::State { block_height } => to_binary(&query_state(deps, block_height)?),
        QueryMsg::StakerInfo { staker } => to_binary(&query_staker_info(deps, env, &staker)?),
    }?;

    Ok(result)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> ContractResult<Response> {
    Ok(Response::default())
}
