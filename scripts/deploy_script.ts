import "dotenv/config";
import { Coin, LCDClient, LocalTerra, Wallet } from "@terra-money/terra.js";

import {
  executeContract,
  newClient,
  executeContractJsonForMultiSig,
  readArtifact,
  writeArtifact,
  queryContract,
  uploadContract,
  toEncodedBinary,
  deployContract,
  instantiateContract,
  Client,
  migrate,
} from "./helpers/helpers.js";
import { join } from "path";

async function main() {
  // terra, wallet
  const { terra, wallet } = newClient();
  console.log(
    `chainID: ${terra.config.chainID} wallet: ${wallet.key.accAddress}`
  );

  // network : stores contract addresses
  const network = readArtifact(terra.config.chainID);
  const ARTIFACTS_PATH = "../artifacts";

  let msg = {
    generator_contract_addr: network.generator_address,
    pair_addr: network.vkr_luna_pair,
    lp_token_addr: network.vkr_luna_lp_token,
    reward_contract_addr: network.vkr_luna_lp_staking,
    reward_token_addr: network.vkr_token,
  };
  console.log(msg);

  // return;
  network.generator_proxy_to_VKR_LUNA_LP_contract = await deployContract(
    terra,
    wallet,
    join(ARTIFACTS_PATH, "generator_proxy_to_vkr.wasm"),
    msg,
    "generator_proxy_to_vkr"
  );
  console.log(network.generator_proxy_to_VKR_LUNA_LP_contract);
  writeArtifact(network, terra.config.chainID);
}

main().catch(console.log);
