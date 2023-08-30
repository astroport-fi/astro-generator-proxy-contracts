use ap_valkyrie::staking_vkr::{Cw20HookMsg, ExecuteMsg, InstantiateMsg};
use cosmwasm_std::{to_binary, Addr, Decimal, Env, MessageInfo, Response, Uint128};
use cw20::Cw20ReceiveMsg;

use crate::entrypoints::{execute, instantiate};
use crate::states::{Config, State};
use crate::tests::mock_querier::{custom_deps, CustomDeps};
use crate::tests::test_utils::{
    default_sender, lp_env, DEFAULT_SENDER, LP_DISTRIBUTION_SCHEDULE1, LP_DISTRIBUTION_SCHEDULE2,
    LP_LIQUIDITY_TOKEN, LP_PAIR_TOKEN, LP_REWARD_TOKEN, LP_WHITELISTED1, LP_WHITELISTED2,
};
use crate::ContractResult;

pub fn exec(
    deps: &mut CustomDeps,
    env: Env,
    info: MessageInfo,
    token: String,
    pair: String,
    lp_token: String,
    whitelisted_contracts: Vec<String>,
) -> ContractResult<Response> {
    let msg = InstantiateMsg {
        token,
        pair,
        lp_token,
        whitelisted_contracts,
    };

    instantiate(deps.as_mut(), env, info, msg)
}

pub fn default(
    deps: &mut CustomDeps,
    schedules: Vec<(u64, u64, Uint128)>,
    total_bonded: Option<Uint128>,
) -> (Env, MessageInfo, Response) {
    let env = lp_env();
    let info = default_sender();

    let response = exec(
        deps,
        env.clone(),
        info.clone(),
        LP_REWARD_TOKEN.to_string(),
        LP_PAIR_TOKEN.to_string(),
        LP_LIQUIDITY_TOKEN.to_string(),
        vec![LP_WHITELISTED1.to_string(), LP_WHITELISTED2.to_string()],
    )
    .unwrap();

    if let Some(total_bonded) = total_bonded {
        let msg = ExecuteMsg::Receive(Cw20ReceiveMsg {
            sender: info.sender.to_string(),
            amount: total_bonded,
            msg: to_binary(&Cw20HookMsg::Bond { schedules }).unwrap(),
        });

        let mut info = info.clone();
        info.sender = Addr::unchecked(LP_LIQUIDITY_TOKEN);
        execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    }

    deps.querier.plus_token_balances(&[
        (
            LP_REWARD_TOKEN,
            &[(DEFAULT_SENDER, &LP_DISTRIBUTION_SCHEDULE1.2)],
        ),
        (
            LP_REWARD_TOKEN,
            &[(DEFAULT_SENDER, &LP_DISTRIBUTION_SCHEDULE2.2)],
        ),
    ]);

    (env, info, response)
}

#[test]
fn succeed() {
    let mut deps = custom_deps();

    let (env, info, _response) = default(
        &mut deps,
        vec![(0, 10, Uint128::new(100))],
        Some(Uint128::new(100u128)),
    );

    let config = Config::load(&deps.storage).unwrap();
    assert_eq!(config.token, LP_REWARD_TOKEN);
    assert_eq!(config.pair, LP_PAIR_TOKEN);
    assert_eq!(config.lp_token, LP_LIQUIDITY_TOKEN);
    assert_eq!(config.admin, info.sender);
    assert_eq!(
        config.whitelisted_contracts,
        vec![LP_WHITELISTED1.to_string(), LP_WHITELISTED2.to_string()]
    );
    assert_eq!(
        config.distribution_schedule,
        vec![(0, 10, Uint128::new(100))]
    );

    let state = State::load(&deps.storage).unwrap();
    assert_eq!(state.global_reward_index, Decimal::zero());
    assert_eq!(state.last_distributed, env.block.height);
    assert_eq!(state.total_bond_amount, Uint128::new(100));
}
