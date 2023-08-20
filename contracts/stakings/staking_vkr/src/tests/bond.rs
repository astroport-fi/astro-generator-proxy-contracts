use crate::ContractResult;
use ap_valkyrie::staking_vkr::{Cw20HookMsg, ExecuteMsg};
use cosmwasm_std::testing::mock_info;
use cosmwasm_std::{to_binary, Addr, Env, Response, Uint128};
use cw20::Cw20ReceiveMsg;

use crate::entrypoints::execute;
use crate::states::{StakerInfo, State};
use crate::tests::instantiate::default;
use crate::tests::mock_querier::{custom_deps, CustomDeps};
use crate::tests::test_utils::{LP_LIQUIDITY_TOKEN, LP_WHITELISTED1, LP_WHITELISTED2};

pub fn exec_bond(
    deps: &mut CustomDeps,
    env: &Env,
    sender: &Addr,
    schedules: Vec<(u64, u64, Uint128)>,
    amount: Uint128,
) -> ContractResult<Response> {
    let info = mock_info(LP_LIQUIDITY_TOKEN, &[]);
    let msg = ExecuteMsg::Receive(Cw20ReceiveMsg {
        sender: sender.to_string(),
        amount,
        msg: to_binary(&Cw20HookMsg::Bond { schedules }).unwrap(),
    });

    execute(deps.as_mut(), env.clone(), info.clone(), msg)
}

fn will_success(deps: &mut CustomDeps, env: Env, sender: &Addr) {
    let amount = Uint128::new(100u128);
    exec_bond(
        deps,
        &env,
        sender,
        vec![(0, 100, Uint128::new(100))],
        amount,
    )
    .unwrap();
}

#[test]
fn succeed() {
    let sender1 = Addr::unchecked(LP_WHITELISTED1);
    let sender2 = Addr::unchecked(LP_WHITELISTED2);

    let mut deps = custom_deps();
    let (env, _info, _response) = default(&mut deps, vec![], None);
    will_success(&mut deps, env.clone(), &sender1);
    will_success(&mut deps, env.clone(), &sender2);

    let state1 = State::load(deps.as_ref().storage).unwrap();
    let info1 = StakerInfo::load_or_default(deps.as_ref().storage, &sender1).unwrap();
    let info2 = StakerInfo::load_or_default(deps.as_ref().storage, &sender2).unwrap();

    assert_eq!(state1.total_bond_amount, Uint128::new(200u128));
    assert_eq!(state1.last_distributed, 0);

    assert_eq!(info1.pending_reward, Uint128::zero());
    assert_eq!(info1.bond_amount, Uint128::new(100u128));

    assert_eq!(info2.pending_reward, Uint128::zero());
    assert_eq!(info2.bond_amount, Uint128::new(100u128));
}
