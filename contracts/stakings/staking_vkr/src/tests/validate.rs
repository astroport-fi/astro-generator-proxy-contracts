use crate::entrypoints::query;
use crate::errors::ContractError;
use crate::tests::bond::exec_bond;
use crate::tests::instantiate::default;
use crate::tests::mock_querier::{custom_deps, CustomDeps};
use crate::tests::unbond::exec_unbond;
use ap_valkyrie::staking_vkr::{QueryMsg, StakerInfoResponse};
use cosmwasm_std::{from_binary, Env, Uint128};

fn query_staker_info(deps: &CustomDeps, env: Env, sender: String) -> StakerInfoResponse {
    from_binary::<StakerInfoResponse>(
        &query(
            deps.as_ref(),
            env.clone(),
            QueryMsg::StakerInfo { staker: sender },
        )
        .unwrap(),
    )
    .unwrap()
}

#[test]
fn calculation() {
    let mut deps = custom_deps();

    let (mut env, info, _response) = default(&mut deps, vec![], None);

    // try to bond 100 tokens without schedules
    let err = exec_bond(&mut deps, &env, &info.sender, vec![], Uint128::new(100u128)).unwrap_err();
    assert_eq!(ContractError::SchedulesAmountError {}, err);

    exec_bond(
        &mut deps,
        &env,
        &info.sender,
        vec![(0, 100, Uint128::new(100))],
        Uint128::new(100u128),
    )
    .unwrap();
    env.block.height += 100;

    // try to register new schedule with wrong block height
    let err = exec_bond(
        &mut deps,
        &env,
        &info.sender,
        vec![(0, 100, Uint128::new(100))],
        Uint128::new(100u128),
    )
    .unwrap_err();
    assert_eq!(ContractError::ScheduleError {}, err);

    exec_bond(
        &mut deps,
        &env,
        &info.sender,
        vec![(100, 200, Uint128::new(100))],
        Uint128::new(100u128),
    )
    .unwrap();

    let res = query_staker_info(&deps, env.clone(), info.sender.to_string());
    assert_eq!(res.pending_reward, Uint128::new(100));
    assert_eq!(res.bond_amount, Uint128::new(200u128));

    env.block.height += 10;
    exec_unbond(&mut deps, env.clone(), info.clone(), Uint128::new(100u128)).unwrap();

    let res = query_staker_info(&deps, env.clone(), info.sender.to_string());
    assert_eq!(res.pending_reward, Uint128::new(110));
    assert_eq!(res.bond_amount, Uint128::new(100u128));

    env.block.height += 10;

    let res = query_staker_info(&deps, env.clone(), info.sender.to_string());
    assert_eq!(res.pending_reward, Uint128::new(120));
    assert_eq!(res.bond_amount, Uint128::new(100u128));
}
