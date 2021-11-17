use cosmwasm_std::{
    entry_point, from_binary, to_binary, Addr, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo,
    Response, StdError, StdResult, SubMsg, Uint128, WasmMsg,
};
use cw20::{BalanceResponse, Cw20ExecuteMsg, Cw20QueryMsg, Cw20ReceiveMsg};

use crate::error::ContractError;
use crate::state::{Config, CONFIG};
use astroport_generator_proxy::apollo_factory::{
    Cw20HookMsg as ApolloFacCw20HookMsg, ExecuteMsg as ApolloFacExecuteMsg,
    QueryMsg as ApolloFacQueryMsg,
};
use astroport_generator_proxy::generator_proxy::{
    Cw20HookMsg, ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg,
};
use astroport_generator_proxy::whale_staking::{
    Cw20ReceiveMsg as ApolloFactoryCw20ReceiveMsg, ExecuteMsg as WhaleExecuteMsg,
    QueryMsg as WhaleQueryMsg, StakerInfoResponse,
};
use cw2::set_contract_version;

// version info for migration info
const CONTRACT_NAME: &str = "astroport-generator-proxy-to-whale";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let config = Config {
        generator_contract_addr: deps.api.addr_validate(&msg.generator_contract_addr)?,
        pair_addr: deps.api.addr_validate(&msg.pair_addr)?,
        lp_token_addr: deps.api.addr_validate(&msg.lp_token_addr)?,
        reward_contract_addr: deps.api.addr_validate(&msg.reward_contract_addr)?,
        reward_token_addr: deps.api.addr_validate(&msg.reward_token_addr)?,
    };
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Receive(msg) => receive_cw20(deps, env, info, msg),
        ExecuteMsg::UpdateRewards {} => update_rewards(deps),
        ExecuteMsg::SendRewards { account, amount } => send_rewards(deps, info, account, amount),
        ExecuteMsg::Withdraw { account, amount } => withdraw(deps, info, account, amount),
        ExecuteMsg::EmergencyWithdraw { account, amount } => withdraw(deps, info, account, amount),
    }
}

/// @dev Receives LP tokens sent by Generator contract. Further sends them to the APOLLO LP Staking contract (happens via Apollo Factory)
fn receive_cw20(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    cw20_msg: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    let mut response = Response::new();
    let cfg = CONFIG.load(deps.storage)?;

    if let Ok(Cw20HookMsg::Deposit {}) = from_binary(&cw20_msg.msg) {
        if cw20_msg.sender != cfg.generator_contract_addr || info.sender != cfg.lp_token_addr {
            return Err(ContractError::Unauthorized {});
        }
        response
            .messages
            .push(SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: cfg.lp_token_addr.to_string(),
                funds: vec![],
                msg: to_binary(&Cw20ExecuteMsg::Send {
                    contract: cfg.reward_contract_addr.to_string(),
                    amount: cw20_msg.amount,
                    msg: to_binary(&ApolloFacCw20HookMsg::Deposit {
                        strategy_id: config.strategy_id,
                    })?,
                })?,
            })));
    } else {
        return Err(ContractError::IncorrectCw20HookMessageVariant {});
    }
    Ok(response)
}

/// @dev Claims pending rewards from the APOLLO LP staking contract (happens via Apollo Factory)
fn update_rewards(deps: DepsMut) -> Result<Response, ContractError> {
    let mut response = Response::new();
    let cfg = CONFIG.load(deps.storage)?;

    response
        .messages
        .push(SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: cfg.reward_contract_addr.to_string(),
            funds: vec![],
            msg: to_binary(&ApolloFacExecuteMsg::ClaimRewards {
                strategy_id: config.strategy_id,
            })?,
        })));

    Ok(response)
}

/// @dev Transfers APOLLO rewards
/// @param account : User to which APOLLO tokens are to be transferred
/// @param amount : Number of APOLLO to be transferred
fn send_rewards(
    deps: DepsMut,
    info: MessageInfo,
    account: Addr,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let mut response = Response::new();
    let cfg = CONFIG.load(deps.storage)?;
    if info.sender != cfg.generator_contract_addr {
        return Err(ContractError::Unauthorized {});
    };

    response
        .messages
        .push(SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: cfg.reward_token_addr.to_string(),
            msg: to_binary(&Cw20ExecuteMsg::Transfer {
                recipient: account.to_string(),
                amount,
            })?,
            funds: vec![],
        })));
    Ok(response)
}

/// @dev Withdraws LP Tokens from the staking contract. Rewards are NOT claimed when withdrawing LP tokens
/// @param account : User to which LP tokens are to be transferred
/// @param amount : Number of LP to be unstaked and transferred
fn withdraw(
    deps: DepsMut,
    info: MessageInfo,
    account: Addr,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let mut response = Response::new();
    let cfg = CONFIG.load(deps.storage)?;
    if info.sender != cfg.generator_contract_addr {
        return Err(ContractError::Unauthorized {});
    };

    // withdraw from the end reward contract
    response.messages.push(SubMsg::new(WasmMsg::Execute {
        contract_addr: cfg.reward_contract_addr.to_string(),
        funds: vec![],
        msg: to_binary(&ApolloFacExecuteMsg::WithdrawFromStrategy {
            strategy_id: config.strategy_id,
            amount: amount.into(),
        })?,
    }));

    response.messages.push(SubMsg::new(WasmMsg::Execute {
        contract_addr: cfg.lp_token_addr.to_string(),
        funds: vec![],
        msg: to_binary(&Cw20ExecuteMsg::Transfer {
            recipient: account.to_string(),
            amount,
        })?,
    }));

    Ok(response)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    let cfg = CONFIG.load(deps.storage)?;
    match msg {
        QueryMsg::Deposit {} => {
            let res: GetUserStrategiesResponse = deps.querier.query_wasm_smart(
                cfg.reward_contract_addr,
                &ApolloFacQueryMsg::GetUserStrategies {
                    user: env.contract.address.to_string(),
                    limit: None,
                    start_from: None,
                },
            )?;
            let lp_staking_strat = res.strategies[0];
            let deposit_amount = lp_staking_strat.base_token_balance;
            to_binary(&deposit_amount)
        }
        QueryMsg::Reward {} => {
            let res: Result<BalanceResponse, StdError> = deps.querier.query_wasm_smart(
                cfg.reward_token_addr,
                &Cw20QueryMsg::Balance {
                    address: env.contract.address.into_string(),
                },
            );
            let reward_amount = res?.balance;

            to_binary(&reward_amount)
        }
        QueryMsg::PendingToken {} => {
            let res: GetUserStrategiesResponse = deps.querier.query_wasm_smart(
                cfg.reward_contract_addr,
                &ApolloFacQueryMsg::GetUserStrategies {
                    user: env.contract.address.to_string(),
                    limit: None,
                    start_from: None,
                },
            )?;
            let lp_staking_strat = res.strategies[0];
            let pending_reward = lp_staking_strat.lm_pending_reward;
            to_binary(&Some(pending_reward))
        }
        QueryMsg::RewardInfo {} => {
            let config = CONFIG.load(deps.storage)?;
            to_binary(&config.reward_token_addr)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}
