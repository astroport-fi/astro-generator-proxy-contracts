use cosmwasm_std::testing::mock_info;
use cosmwasm_std::{Addr, Env, MessageInfo, Response};

use crate::executions::update_config;
use crate::states::Config;
use crate::ContractResult;

use crate::tests::instantiate::default;
use crate::tests::mock_querier::{custom_deps, CustomDeps};
use crate::tests::test_utils::{expect_unauthorized_err, lp_env, LP_WHITELISTED1};

pub fn exec(
    deps: &mut CustomDeps,
    info: MessageInfo,
    token: Option<String>,
    pair: Option<String>,
    lp_token: Option<String>,
    admin: Option<String>,
    whitelisted_contracts: Option<Vec<String>>,
) -> ContractResult<Response> {
    update_config(
        deps.as_mut(),
        info,
        token,
        pair,
        lp_token,
        admin,
        whitelisted_contracts,
    )
}

pub fn will_success(
    deps: &mut CustomDeps,
    token: Option<String>,
    pair: Option<String>,
    lp_token: Option<String>,
    admin: Option<String>,
    whitelisted_contracts: Option<Vec<String>>,
) -> (Env, MessageInfo, Response) {
    let env = lp_env();
    let info = mock_info(LP_WHITELISTED1, &[]);

    let response = exec(
        deps,
        info.clone(),
        token,
        pair,
        lp_token,
        admin,
        whitelisted_contracts,
    )
    .unwrap();

    (env, info, response)
}

#[test]
fn succeed() {
    let mut deps = custom_deps();

    let (_env, info, _response) = default(&mut deps, vec![], None);

    let whitelisted_contracts = vec![
        "terra1r4qtnusnk63wkg2y6sytwr37aymz0sfy3p2yc9".to_string(),
        "terra14mtctaszgzm4gcedlfslds802fmklnp4up72da".to_string(),
    ];

    will_success(
        &mut deps,
        Some("terra1r0rm0evrlkfvpt0csrcpmnpmrega54czajfd86".to_string()),
        Some("terra1fmcjjt6yc9wqup2r06urnrd928jhrde6gcld6n".to_string()),
        Some("terra199vw7724lzkwz6lf2hsx04lrxfkz09tg8dlp6r".to_string()),
        Some("terra1e8ryd9ezefuucd4mje33zdms9m2s90m57878v9".to_string()),
        Some(whitelisted_contracts.clone()),
    );

    let config = Config::load(&deps.storage).unwrap();
    assert_eq!(
        config.token,
        Addr::unchecked("terra1r0rm0evrlkfvpt0csrcpmnpmrega54czajfd86".to_string())
    );
    assert_eq!(
        config.pair,
        Addr::unchecked("terra1fmcjjt6yc9wqup2r06urnrd928jhrde6gcld6n".to_string())
    );
    assert_eq!(
        config.lp_token,
        Addr::unchecked("terra199vw7724lzkwz6lf2hsx04lrxfkz09tg8dlp6r".to_string())
    );
    assert_eq!(config.admin, info.sender);
    assert_eq!(config.whitelisted_contracts, whitelisted_contracts);

    let admin_nominee = Config::may_load_admin_nominee(&deps.storage).unwrap();
    assert_eq!(
        admin_nominee,
        Some(Addr::unchecked(
            "terra1e8ryd9ezefuucd4mje33zdms9m2s90m57878v9".to_string()
        ))
    );
}

#[test]
fn failed_invalid_permission() {
    let mut deps = custom_deps();

    let (_, mut info, _response) = default(&mut deps, vec![], None);

    info.sender = Addr::unchecked("terra1e8ryd9ezefuucd4mje33zdms9m2s90m57878v9");

    let result = exec(&mut deps, info, None, None, None, None, None);

    expect_unauthorized_err(&result);
}
