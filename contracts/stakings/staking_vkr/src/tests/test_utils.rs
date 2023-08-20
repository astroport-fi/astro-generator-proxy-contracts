use cosmwasm_std::testing::{mock_env, mock_info};
use cosmwasm_std::{Addr, Env, MessageInfo, Response, StdError, Uint128};

use crate::errors::ContractError;
use crate::ContractResult;

pub const LP_REWARD_TOKEN: &str = VALKYRIE_TOKEN;
pub const LP_PAIR_TOKEN: &str = "terra17n5sunn88hpy965mzvt3079fqx3rttnplg779g";
pub const LP_LIQUIDITY_TOKEN: &str = "terra1627ldjvxatt54ydd3ns6xaxtd68a2vtyu7kakj";
pub const LP_WHITELISTED1: &str = "terra190fxpjfkp6cygr2k9unzjurq42dyehqd579h5j";
pub const LP_WHITELISTED2: &str = "terra1c7m6j8ya58a2fkkptn8fgudx8sqjqvc8azq0ex";
pub const LP_DISTRIBUTION_SCHEDULE1: (u64, u64, Uint128) = (0, 100, Uint128::new(1000000u128));
pub const LP_DISTRIBUTION_SCHEDULE2: (u64, u64, Uint128) = (100, 200, Uint128::new(10000000u128));

pub const DEFAULT_SENDER: &str = "terra1sq9ppsvt4k378wwhvm2vyfg7kqrhtve8p0n3a6";
pub const VALKYRIE_TOKEN: &str = "terra1xj49zyqrwpv5k928jwfpfy2ha668nwdgkwlrg3";
pub const VALKYRIE_PROXY: &str = "terra1fnywlw4edny3vw44x04xd67uzkdqluymgreu7g";
pub const LIQUIDITY: &str = "terra1l7xu2rl3c7qmtx3r5sd2tz25glf6jh8ul7aag7";

pub fn default_sender() -> MessageInfo {
    mock_info(LP_WHITELISTED1, &[])
}

pub fn lp_env() -> Env {
    let mut env = mock_env_contract(LIQUIDITY);
    env.block.height = 0;
    env
}

pub fn mock_env_contract(contract: &str) -> Env {
    let mut env = mock_env();

    env.contract.address = Addr::unchecked(contract);

    env
}

pub fn expect_generic_err(result: &ContractResult<Response>, expect_msg: &str) {
    match result {
        Ok(_) => panic!("Must return error"),
        Err(ContractError::Std(StdError::GenericErr { msg, .. })) => assert_eq!(msg, expect_msg),
        Err(e) => panic!("Unexpected error: {:?}", e),
    }
}

pub fn expect_unauthorized_err(result: &ContractResult<Response>) {
    match result {
        Ok(_) => panic!("Must return error"),
        Err(ContractError::Unauthorized {}) => {
            // do nothing
        }
        Err(e) => panic!("Unexpected error: {:?}", e),
    }
}
