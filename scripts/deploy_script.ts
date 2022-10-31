import "dotenv/config";
import { LCDClient, Wallet } from "@terra-money/terra.js";
import {
  newClient,
  readArtifact,
  writeArtifact,
  deployContract,
} from "./helpers/helpers.js";
import { join } from "path";

const ARTIFACTS_PATH = "../artifacts";

async function main() {
    const { terra, wallet } = newClient();
    console.log(`chainID: ${terra.config.chainID} wallet: ${wallet.key.accAddress}`);

    await uploadAndInitGenProxyToVkr(terra, wallet)
}

async function uploadAndInitGenProxyToVkr(terra: LCDClient, wallet: Wallet){
  const network = readArtifact(terra.config.chainID);

  if (!network.generatorProxyToVkrAddress) {
    console.log('Deploy the Generator proxy to vkr...');

    let resp = await deployContract(
        terra,
        wallet,
        join(ARTIFACTS_PATH, "generator_proxy_to_vkr.wasm"),
        {
          generator_contract_addr: network.generator_address,
          pair_addr: network.vkr_luna_pair,
          lp_token_addr: network.vkr_luna_lp_token,
          reward_contract_addr: network.vkr_luna_lp_staking,
          reward_token_addr: network.vkr_token,
        },
        "Astroport generator proxy to VKR"
    );

    // @ts-ignore
    network.generatorProxyToVkrAddress = resp.shift().shift()

    console.log(`Address Generator proxy to VKR contract: ${network.generatorProxyToVkrAddress}`)
    writeArtifact(network, terra.config.chainID)
  }
}

main().catch(console.log);
