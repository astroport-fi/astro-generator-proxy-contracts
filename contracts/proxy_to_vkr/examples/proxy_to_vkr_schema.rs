use ap_generator_proxy::{ExecuteMsg, InstantiateMsg, QueryMsg};
use ap_generator_proxy_to_vkr::MigrateMsg;
use cosmwasm_schema::write_api;

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        query: QueryMsg,
        execute: ExecuteMsg,
        migrate: MigrateMsg,
    }
}
