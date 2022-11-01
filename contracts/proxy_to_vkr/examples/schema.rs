use astroport::generator_proxy::{ExecuteMsg, InstantiateMsg, QueryMsg};
use astroport_generator_proxies::proxy_to_vkr::MigrateMsg;
use cosmwasm_schema::write_api;

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        query: QueryMsg,
        execute: ExecuteMsg,
        migrate: MigrateMsg,
    }
}
