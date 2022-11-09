interface GeneralInfo {
    multisig: string
}

interface ProxyVKR {
    admin: string,
    initMsg: {
        generator_contract_addr: string,
        pair_addr: string,
        lp_token_addr: string,
        reward_contract_addr: string,
        reward_token_addr: string,
    },
    label: string
}

interface Config {
    generalInfo: GeneralInfo,
    proxyVKR: ProxyVKR,
}