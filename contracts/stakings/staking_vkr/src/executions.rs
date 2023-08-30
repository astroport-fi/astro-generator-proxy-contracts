use cosmwasm_std::{attr, wasm_execute, DepsMut, Env, MessageInfo, Response, StdError, Uint128};

use crate::states::{Config, StakerInfo, State};

use crate::errors::ContractError;
use crate::utils::register_schedules;
use crate::ContractResult;
use cw20::Cw20ExecuteMsg;

pub fn bond(
    deps: DepsMut,
    env: Env,
    config: &mut Config,
    sender: &str,
    schedules: Vec<(u64, u64, Uint128)>,
    amount: Uint128,
) -> ContractResult<Response> {
    let sender_addr = deps.api.addr_validate(sender)?;

    if !config.is_authorized(&sender_addr) {
        return Err(ContractError::Std(StdError::generic_err(
            "Can only called by wallet",
        )));
    }

    register_schedules(env.block.height, config, schedules, amount)?;

    let mut state: State = State::load(deps.storage)?;
    let mut staker_info: StakerInfo = StakerInfo::load_or_default(deps.storage, &sender_addr)?;

    // Compute global reward & staker reward
    state.compute_reward(config, env.block.height);
    staker_info.compute_staker_reward(&state)?;

    // Increase bond_amount
    state.total_bond_amount += amount;
    staker_info.bond_amount += amount;
    staker_info.save(deps.storage)?;
    state.save(deps.storage)?;
    config.save(deps.storage)?;

    Ok(Response::new().add_attributes(vec![
        attr("action", "bond"),
        attr("amount", amount.to_string()),
    ]))
}

pub fn unbond(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: Uint128,
) -> ContractResult<Response> {
    let config: Config = Config::load(deps.storage)?;
    let mut state: State = State::load(deps.storage)?;
    let mut staker_info: StakerInfo = StakerInfo::load_or_default(deps.storage, &info.sender)?;

    if staker_info.bond_amount < amount {
        return Err(ContractError::Std(StdError::generic_err(
            "Cannot unbond more than bond amount",
        )));
    }

    // Compute global reward & staker reward
    state.compute_reward(&config, env.block.height);
    staker_info.compute_staker_reward(&state)?;

    // Decrease bond_amount
    state.total_bond_amount = (state.total_bond_amount.checked_sub(amount))?;
    state.save(deps.storage)?;
    // Store or remove updated rewards info
    // depends on the left pending reward and bond amount
    staker_info.bond_amount = (staker_info.bond_amount.checked_sub(amount))?;
    if staker_info.pending_reward.is_zero() && staker_info.bond_amount.is_zero() {
        //no bond, no reward.
        staker_info.delete(deps.storage);
    } else {
        staker_info.save(deps.storage)?;
    }

    Ok(Response::new()
        .add_message(wasm_execute(
            &config.lp_token,
            &Cw20ExecuteMsg::Transfer {
                recipient: info.sender.to_string(),
                amount,
            },
            vec![],
        )?)
        .add_attributes(vec![
            attr("action", "unbond"),
            attr("amount", amount.to_string()),
        ]))
}

// withdraw rewards to executor
pub fn withdraw(deps: DepsMut, env: Env, info: MessageInfo) -> ContractResult<Response> {
    let config: Config = Config::load(deps.storage)?;
    let mut state: State = State::load(deps.storage)?;
    let mut staker_info = StakerInfo::load_or_default(deps.storage, &info.sender)?;

    // Compute global reward & staker reward
    state.compute_reward(&config, env.block.height);
    staker_info.compute_staker_reward(&state)?;
    state.save(deps.storage)?;

    let amount = staker_info.pending_reward;
    staker_info.pending_reward = Uint128::zero();

    // Store or remove updated rewards info
    // depends on the left pending reward and bond amount
    if staker_info.bond_amount.is_zero() {
        staker_info.delete(deps.storage);
    } else {
        staker_info.save(deps.storage)?;
    }

    Ok(Response::new()
        .add_message(wasm_execute(
            &config.token,
            &Cw20ExecuteMsg::Transfer {
                recipient: info.sender.to_string(),
                amount,
            },
            vec![],
        )?)
        .add_attributes(vec![
            attr("action", "withdraw"),
            attr("amount", amount.to_string()),
        ]))
}

pub fn update_config(
    deps: DepsMut,
    info: MessageInfo,
    token: Option<String>,
    pair: Option<String>,
    lp_token: Option<String>,
    admin: Option<String>,
    whitelisted_contracts: Option<Vec<String>>,
) -> ContractResult<Response> {
    let mut attributes = vec![attr("action", "update_config")];

    let mut config: Config = Config::load(deps.storage)?;
    if config.admin != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    if let Some(token) = token {
        config.token = deps.api.addr_validate(token.as_str())?;
        attributes.push(attr("is_updated_token", "true"));
    }

    if let Some(pair) = pair {
        config.pair = deps.api.addr_validate(pair.as_str())?;
        attributes.push(attr("is_updated_pair", "true"));
    }

    if let Some(lp_token) = lp_token {
        config.lp_token = deps.api.addr_validate(lp_token.as_str())?;
        attributes.push(attr("is_updated_lp_token", "true"));
    }

    if let Some(admin) = admin {
        Config::save_admin_nominee(deps.storage, &deps.api.addr_validate(admin.as_str())?)?;
        attributes.push(attr("is_updated_admin_nominee", "true"));
    }

    if let Some(whitelisted_contracts) = whitelisted_contracts {
        config.whitelisted_contracts = whitelisted_contracts
            .iter()
            .map(|item| deps.api.addr_validate(item.as_str()).unwrap())
            .collect();
        attributes.push(attr("is_updated_whitelisted_contracts", "true"));
    }

    config.save(deps.storage)?;

    Ok(Response::new().add_attributes(attributes))
}

pub fn migrate_reward(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    recipient: String,
    amount: Uint128,
) -> ContractResult<Response> {
    let config = Config::load(deps.storage)?;
    if config.admin != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    let message = wasm_execute(
        &config.token,
        &Cw20ExecuteMsg::Transfer {
            recipient: deps.api.addr_validate(recipient.as_str())?.to_string(),
            amount,
        },
        vec![],
    )?;

    Ok(Response::new().add_message(message).add_attributes(vec![
        attr("action", "migrate_reward"),
        attr("recipient", recipient),
        attr("amount", amount.to_string()),
    ]))
}

pub fn approve_admin_nominee(deps: DepsMut, info: MessageInfo) -> ContractResult<Response> {
    if let Some(admin_nominee) = Config::may_load_admin_nominee(deps.storage)? {
        if admin_nominee != info.sender {
            return Err(ContractError::Std(StdError::generic_err(
                "It is not admin nominee",
            )));
        }
    } else {
        return Err(ContractError::Unauthorized {});
    }

    let mut config = Config::load(deps.storage)?;
    config.admin = info.sender;
    config.save(deps.storage)?;

    Ok(Response::new().add_attributes(vec![
        attr("action", "approve_admin_nominee"),
        attr("is_updated_admin", "true"),
    ]))
}
