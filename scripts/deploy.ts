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

    console.log("network: ", network);

      network.generatorProxyToVkrAddress = await deployContract(
        terra,
        wallet,
        join(ARTIFACTS_PATH, "generator_proxy_to_vkr.wasm"),
        {
          generator_contract_addr: network.generatorAddress,
          pair_addr: network.vkrLunaPairAddress,
          lp_token_addr: network.vkrLunaLpTokenAddress,
          reward_contract_addr: network.vkrLunaLpStakingAddress,
          reward_token_addr: network.vkrTokenAddress,
        },
        "Astroport generator proxy to VKR"
    );

    console.log(`Address Generator proxy to VKR contract: ${network.generatorProxyToVkrAddress}`)
    writeArtifact(network, terra.config.chainID)
  }
}

main().catch(console.log);
