use astroport::router::QueryMsg::SimulateSwapOperations;
use astroport::router::{SimulateSwapOperationsResponse, SwapOperation};
use std::collections::HashMap;
use std::marker::PhantomData;

use crate::tests::test_utils::VALKYRIE_PROXY;
use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage, MOCK_CONTRACT_ADDR};
use cosmwasm_std::{
    from_binary, from_slice, to_binary, Api, Binary, CanonicalAddr, ContractResult, Empty,
    OwnedDeps, Querier, QuerierResult, QueryRequest, SystemError, SystemResult, Uint128, WasmQuery,
};
use cw20::{Cw20QueryMsg, TokenInfoResponse};

pub type CustomDeps = OwnedDeps<MockStorage, MockApi, WasmMockQuerier>;

/// mock_dependencies is a drop-in replacement for cosmwasm_std::testing::mock_dependencies
/// this uses our CustomQuerier.
pub fn custom_deps() -> CustomDeps {
    let custom_querier = WasmMockQuerier::new(MockQuerier::new(&[(MOCK_CONTRACT_ADDR, &[])]));

    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: custom_querier,
        custom_query_type: PhantomData,
    }
}

pub struct WasmMockQuerier {
    base: MockQuerier<Empty>,
    token_querier: TokenQuerier,
    astroport_router_querier: AstroportRouterQuerier,
}

#[derive(Clone, Default)]
pub struct AstroportRouterQuerier {
    prices: HashMap<(String, String), f64>,
}

#[derive(Clone, Default)]
pub struct TokenQuerier {
    // this lets us iterate over all pairs that match the first string
    balances: HashMap<String, HashMap<String, Uint128>>,
}

impl Querier for WasmMockQuerier {
    fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
        // MockQuerier doesn't support Custom, so we ignore it completely here
        let request: QueryRequest<Empty> = match from_slice(bin_request) {
            Ok(v) => v,
            Err(_) => {
                return SystemResult::Err(SystemError::InvalidRequest {
                    error: format!("Parsing query request"),
                    request: bin_request.into(),
                });
            }
        };
        self.handle_query(&request)
    }
}

impl WasmMockQuerier {
    pub fn handle_query(&self, request: &QueryRequest<Empty>) -> QuerierResult {
        match &request {
            QueryRequest::Wasm(WasmQuery::Raw { contract_addr, key }) => {
                self.handle_wasm_raw(contract_addr, key)
            }
            QueryRequest::Wasm(WasmQuery::Smart { contract_addr, msg }) => {
                self.handle_wasm_smart(contract_addr, msg)
            }
            _ => self.base.handle_query(request),
        }
    }

    fn handle_wasm_raw(&self, contract_addr: &String, key: &Binary) -> QuerierResult {
        let key: &[u8] = key.as_slice();

        let mut result = self.query_token_info(contract_addr, key);

        if result.is_none() {
            result = self.query_balance(contract_addr, key);
        }

        if result.is_none() {
            return QuerierResult::Err(SystemError::UnsupportedRequest {
                kind: "handle_wasm_raw".to_string(),
            });
        }

        result.unwrap()
    }

    fn handle_wasm_smart(&self, contract_addr: &String, msg: &Binary) -> QuerierResult {
        let mut result = self.handle_wasm_smart_astroport_router(contract_addr, msg);

        if result.is_none() {
            result = self.handle_cw20(contract_addr, msg);
        }

        if result.is_none() {
            return QuerierResult::Err(SystemError::UnsupportedRequest {
                kind: "handle_wasm_smart".to_string(),
            });
        }

        result.unwrap()
    }

    fn handle_cw20(&self, contract_addr: &String, msg: &Binary) -> Option<QuerierResult> {
        match from_binary(msg) {
            Ok(Cw20QueryMsg::Balance { address }) => {
                let default = Uint128::zero();
                let balance = self.token_querier.balances[contract_addr]
                    .get(address.as_str())
                    .unwrap_or(&default)
                    .clone();

                Some(SystemResult::Ok(ContractResult::from(to_binary(
                    &cw20::BalanceResponse { balance },
                ))))
            }
            Ok(_) => Some(QuerierResult::Err(SystemError::UnsupportedRequest {
                kind: "handle_wasm_smart:cw20".to_string(),
            })),
            Err(_) => None,
        }
    }

    fn handle_wasm_smart_astroport_router(
        &self,
        contract_addr: &String,
        msg: &Binary,
    ) -> Option<QuerierResult> {
        if contract_addr != VALKYRIE_PROXY {
            return None;
        }

        match from_binary(msg) {
            Ok(SimulateSwapOperations {
                offer_amount,
                operations,
            }) => {
                let mut amount = offer_amount.u128();
                for operation in operations.iter() {
                    let price = self
                        .astroport_router_querier
                        .prices
                        .get(&swap_operation_to_string(operation))
                        .unwrap();

                    amount = (amount as f64 * *price) as u128;
                }

                Some(SystemResult::Ok(ContractResult::from(to_binary(
                    &SimulateSwapOperationsResponse {
                        amount: Uint128::new(amount),
                    },
                ))))
            }
            Ok(_) => Some(QuerierResult::Err(SystemError::UnsupportedRequest {
                kind: "handle_wasm_smart:valkyrie_proxy".to_string(),
            })),
            Err(_) => None,
        }
    }

    fn query_token_info(&self, contract_addr: &String, key: &[u8]) -> Option<QuerierResult> {
        if key.to_vec() != to_length_prefixed(b"token_info").to_vec() {
            return None;
        }

        let balances = self.token_querier.balances.get(contract_addr);

        if balances.is_none() {
            return Some(SystemResult::Err(SystemError::InvalidRequest {
                request: key.into(),
                error: format!("No balance info exists for the contract {}", contract_addr,),
            }));
        }

        let balances = balances.unwrap();

        let mut total_supply = Uint128::zero();

        for balance in balances {
            total_supply += *balance.1;
        }

        Some(SystemResult::Ok(ContractResult::Ok(
            to_binary(&TokenInfoResponse {
                name: format!("{}Token", contract_addr),
                symbol: format!("TOK"),
                decimals: 6,
                total_supply,
            })
            .unwrap(),
        )))
    }

    fn query_balance(&self, contract_addr: &String, key: &[u8]) -> Option<QuerierResult> {
        let prefix_balance = to_length_prefixed(b"balance").to_vec();
        if key[..prefix_balance.len()].to_vec() == prefix_balance {}

        let balances = self.token_querier.balances.get(contract_addr);

        if balances.is_none() {
            return Some(SystemResult::Err(SystemError::InvalidRequest {
                request: key.into(),
                error: format!("No balance info exists for the contract {}", contract_addr,),
            }));
        }

        let balances = balances.unwrap();

        let key_address: &[u8] = &key[prefix_balance.len()..];
        let address_raw: CanonicalAddr = CanonicalAddr::from(key_address);
        let api = MockApi::default();
        let address = match api.addr_humanize(&address_raw) {
            Ok(v) => v.to_string(),
            Err(_) => {
                return Some(SystemResult::Err(SystemError::InvalidRequest {
                    error: format!("Parsing query request"),
                    request: key.into(),
                }));
            }
        };
        let balance = match balances.get(&address) {
            Some(v) => v,
            None => {
                return Some(SystemResult::Err(SystemError::InvalidRequest {
                    error: "Balance not found".to_string(),
                    request: key.into(),
                }));
            }
        };

        Some(SystemResult::Ok(ContractResult::Ok(
            to_binary(&balance).unwrap(),
        )))
    }
}

const ZERO: Uint128 = Uint128::zero();

impl WasmMockQuerier {
    pub fn new(base: MockQuerier<Empty>) -> Self {
        WasmMockQuerier {
            base,
            token_querier: TokenQuerier::default(),
            astroport_router_querier: AstroportRouterQuerier::default(),
        }
    }

    pub fn plus_token_balances(&mut self, balances: &[(&str, &[(&str, &Uint128)])]) {
        for (token_contract, balances) in balances.iter() {
            let token_contract = token_contract.to_string();

            if !self.token_querier.balances.contains_key(&token_contract) {
                self.token_querier
                    .balances
                    .insert(token_contract.clone(), HashMap::new());
            }
            let token_balances = self
                .token_querier
                .balances
                .get_mut(&token_contract)
                .unwrap();

            for (account, balance) in balances.iter() {
                let account = account.to_string();
                let account_balance = token_balances.get(&account).unwrap_or(&ZERO);
                let new_balance = *account_balance + *balance;
                token_balances.insert(account, new_balance);
            }
        }
    }
}

// Copy from cosmwasm-storage v0.14.1
fn to_length_prefixed(namespace: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(namespace.len() + 2);
    out.extend_from_slice(&encode_length(namespace));
    out.extend_from_slice(namespace);
    out
}

// Copy from cosmwasm-storage v0.14.1
fn encode_length(namespace: &[u8]) -> [u8; 2] {
    if namespace.len() > 0xFFFF {
        panic!("only supports namespaces up to length 0xFFFF")
    }
    let length_bytes = (namespace.len() as u32).to_be_bytes();
    [length_bytes[2], length_bytes[3]]
}

fn swap_operation_to_string(operation: &SwapOperation) -> (String, String) {
    match operation {
        SwapOperation::NativeSwap {
            offer_denom,
            ask_denom,
        } => (offer_denom.to_string(), ask_denom.to_string()),
        SwapOperation::AstroSwap {
            offer_asset_info,
            ask_asset_info,
        } => (offer_asset_info.to_string(), ask_asset_info.to_string()),
    }
}
