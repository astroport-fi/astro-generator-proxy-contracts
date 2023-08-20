use cosmwasm_std::{
    to_binary, Addr, CosmosMsg, Env, MessageInfo, Response, SubMsg, Uint128, WasmMsg,
};
use cw20::Cw20ExecuteMsg;

use crate::executions::withdraw;
use crate::tests::instantiate::default;
use crate::tests::mock_querier::{custom_deps, CustomDeps};
use crate::tests::test_utils::{LP_WHITELISTED1, VALKYRIE_TOKEN};
use crate::ContractResult;

pub fn exec(deps: &mut CustomDeps, env: Env, info: MessageInfo) -> ContractResult<Response> {
    withdraw(deps.as_mut(), env, info)
}

fn will_success(deps: &mut CustomDeps) -> Response {
    let (mut env, info, _response) = default(
        deps,
        vec![(0, 100, Uint128::new(1000))],
        Some(Uint128::new(1000u128)),
    );
    env.block.height = 20;
    exec(deps, env, info).unwrap()
}

#[test]
fn succeed() {
    let mut deps = custom_deps();

    let res = will_success(&mut deps);

    assert_eq!(
        res.messages,
        vec![SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: Addr::unchecked(VALKYRIE_TOKEN).to_string(),
            msg: to_binary(&Cw20ExecuteMsg::Transfer {
                recipient: Addr::unchecked(LP_WHITELISTED1).to_string(),
                amount: Uint128::new(200u128),
            })
            .unwrap(),
            funds: vec![],
        }))]
    );
}
