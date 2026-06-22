import { scanBranches } from "./branches.js";
import { isOutputDescriptor } from "../derive/index.js";
import { parseOutputDescriptor } from "../derive/descriptor.js";
import { addressScripts } from "../derive/script.js";

/**
 * @typedef {import("../derive/address.js").AddressScript} AddressScript
 * @typedef {import("../derive/index.js").AddressType} AddressType
 * @typedef {Awaited<ReturnType<typeof scanBranches>>["addresses"][number]} WalletAddress
 * @typedef {Awaited<ReturnType<typeof scanBranches>>} ScriptScan
 */

/**
 * @typedef {Object} WalletScan
 * @property {WalletAddress[]} addresses
 * @property {WalletAddress | undefined} receiveAddress
 * @property {number} btcUsdPrice
 */

/**
 * @typedef {Object} WalletScanClient
 * @property {string} domain
 * @property {(address: string, options?: { cache?: boolean }) => Promise<unknown>} getAddress
 * @property {(addrType: AddressType, prefix: string, options?: { cache?: boolean }) => Promise<unknown>} getAddressHashPrefixMatches
 * @property {(options?: { cache?: boolean }) => Promise<unknown>} getLivePrice
 */

/**
 * @typedef {Object} WalletScanProgress
 * @property {string} branchLabel
 * @property {number} scannedCount
 * @property {number} unusedInRow
 */

/**
 * @typedef {Object} ScanScript
 * @property {AddressScript} id
 * @property {string} label
 */

const descriptorScripts = /** @type {const} */ ({
  v0_p2wsh_sortedmulti: "P2WSH",
});

/**
 * @param {string} source
 * @returns {readonly ScanScript[]}
 */
function getSourceScripts(source) {
  if (isOutputDescriptor(source)) {
    const script = parseOutputDescriptor(source).script;

    return [{
      id: script,
      label: descriptorScripts[script],
    }];
  }

  return addressScripts;
}

/**
 * @param {WalletAddress} a
 * @param {WalletAddress} b
 */
function compareWalletAddresses(a, b) {
  return (
    (b.typeIndex ?? -1) - (a.typeIndex ?? -1) ||
    a.script.localeCompare(b.script) ||
    a.branchLabel.localeCompare(b.branchLabel) ||
    a.index - b.index
  );
}

/**
 * @param {ScriptScan} scan
 */
function getLatestSeenIndex(scan) {
  return scan.addresses.reduce((latest, address) => {
    return Math.max(latest, address.typeIndex ?? -1);
  }, -1);
}

/**
 * @param {readonly ScriptScan[]} scans
 */
function selectReceiveAddress(scans) {
  let receiveAddress = scans.find((scan) => {
    return scan.receiveAddress;
  })?.receiveAddress;
  let selectedSeenIndex = -1;

  for (const scan of scans) {
    const seenIndex = getLatestSeenIndex(scan);
    const hasActivity = scan.addresses.length > 0;

    if (
      hasActivity &&
      scan.receiveAddress &&
      seenIndex >= selectedSeenIndex
    ) {
      receiveAddress = scan.receiveAddress;
      selectedSeenIndex = seenIndex;
    }
  }

  return receiveAddress;
}

/**
 * @param {Object} options
 * @param {WalletScanClient} options.client
 * @param {string} options.source
 * @param {(progress: WalletScanProgress) => void} [options.onProgress]
 * @returns {Promise<WalletScan>}
 */
export async function scanWalletAddresses({
  client,
  source,
  onProgress,
}) {
  const scans = /** @type {ScriptScan[]} */ ([]);

  for (const script of getSourceScripts(source)) {
    scans.push(await scanBranches(client, source, {
      script: script.id,
      onProgress(progress) {
        onProgress?.({
          ...progress,
          branchLabel: `${script.label} ${progress.branchLabel}`,
        });
      },
    }));
  }

  const addresses = scans.flatMap((scan) => scan.addresses)
    .sort(compareWalletAddresses);
  const btcUsdPrice = /** @type {number} */ (
    await client.getLivePrice({ cache: false })
  );

  return {
    addresses,
    receiveAddress: selectReceiveAddress(scans),
    btcUsdPrice,
  };
}
