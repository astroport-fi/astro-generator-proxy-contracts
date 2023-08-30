use crate::executions::unbond;
use crate::states::{StakerInfo, State};
use crate::tests::instantiate::default;
use crate::tests::mock_querier::{custom_deps, CustomDeps};
use crate::tests::test_utils::expect_generic_err;
use crate::ContractResult;
use cosmwasm_std::{Env, MessageInfo, Response, Uint128};
use std::ops::Add;

pub fn exec_unbond(
    deps: &mut CustomDeps,
    env: Env,
    info: MessageInfo,
    amount: Uint128,
) -> ContractResult<Response> {
    unbond(deps.as_mut(), env, info, amount)
}

pub fn will_success(deps: &mut CustomDeps, amount: Uint128) -> (Env, MessageInfo, Response) {
    let (mut env, info, _response) = default(deps, vec![(0, 100, amount)], Some(amount));

    env.block.height = 10;

    let response = exec_unbond(deps, env.clone(), info.clone(), amount).unwrap();
    (env, info, response)
}

#[test]
fn succeed() {
    let mut deps = custom_deps();
    let total_bonded = Uint128::new(200u128);
    let (_env, info, _response) = will_success(&mut deps, total_bonded);

    let state1 = State::load(deps.as_ref().storage).unwrap();
    let info1 = StakerInfo::load_or_default(deps.as_ref().storage, &info.sender).unwrap();

    assert_eq!(state1.total_bond_amount, Uint128::zero());
    assert_eq!(state1.last_distributed, 10);

    assert_eq!(info1.pending_reward, Uint128::new(20));
    assert_eq!(info1.bond_amount, Uint128::zero());
}

#[test]
fn failed_invalid_amount() {
    let mut deps = custom_deps();
    let total_bonded = Uint128::new(200u128);
    let (env, info, _response) =
        default(&mut deps, vec![(0, 100, total_bonded)], Some(total_bonded));
    let result = exec_unbond(&mut deps, env, info, total_bonded.add(total_bonded));

    expect_generic_err(&result, "Cannot unbond more than bond amount");
}
