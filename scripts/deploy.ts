import "dotenv/config";
import { LCDClient, Wallet } from "@terra-money/terra.js";
import {
  newClient,
  readArtifact,
  writeArtifact,
  deployContract,
} from "./helpers.js";
import { join } from "path";
import { chainConfigs } from "./types.d/chain_configs.js";

const ARTIFACTS_PATH = "../artifacts";

async function main() {
  const { terra, wallet } = newClient();
  console.log(`chainID: ${terra.config.chainID} wallet: ${wallet.key.accAddress}`);

  if (!chainConfigs.generalInfo.multisig) {
    throw new Error("Set the proper owner multisig for the contracts")
  }

  await uploadAndInitGenProxyToVkr(terra, wallet)
}

async function uploadAndInitGenProxyToVkr(terra: LCDClient, wallet: Wallet) {
  const network = readArtifact(terra.config.chainID);

  if (!network.generatorProxyToVkrAddress) {
    console.log('Deploy the Generator proxy to vkr...');

    chainConfigs.proxyVKR.admin ||= chainConfigs.generalInfo.multisig

    network.generatorProxyToVkrAddress = await deployContract(
      terra,
      wallet,
      chainConfigs.proxyVKR.admin,
      join(ARTIFACTS_PATH, "generator_proxy_to_vkr.wasm"),
      chainConfigs.proxyVKR.initMsg,
      chainConfigs.proxyVKR.label,
    );

    console.log(`Address Generator proxy to VKR contract: ${network.generatorProxyToVkrAddress}`)
    writeArtifact(network, terra.config.chainID)
  }
}

main().catch(console.log);
