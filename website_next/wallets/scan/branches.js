import {
  scanBranch,
  GAP_LIMIT,
} from "./branch.js";
import {
  getOutputDescriptorBranchIds,
  isOutputDescriptor,
} from "../derive/index.js";

const keyBranches = /** @type {const} */ ([
  { id: "receive", label: "Receive", path: [0] },
  { id: "change", label: "Change", path: [1] },
  { id: "direct", label: "Direct", path: [] },
]);

const descriptorBranches = /** @type {const} */ ([
  { id: "receive", label: "Receive", path: [] },
  { id: "change", label: "Change", path: [] },
]);

/**
 * @typedef {(typeof keyBranches[number] | typeof descriptorBranches[number])} WalletBranch
 * @typedef {WalletBranch["id"]} WalletBranchId
 */

/**
 * @typedef {import("../derive/address.js").AddressScript} AddressScript
 * @typedef {import("../derive/index.js").AddressType} AddressType
 * @typedef {import("./branch.js").WalletAddress} WalletAddress
 */

/**
 * @typedef {Object} AddressClient
 * @property {(address: string, options?: { cache?: boolean }) => Promise<unknown>} getAddress
 * @property {(addrType: AddressType, prefix: string, options?: { cache?: boolean }) => Promise<unknown>} getAddressHashPrefixMatches
 */

/**
 * @typedef {WalletAddress & {
 *   branchId: WalletBranchId,
 *   branchLabel: string,
 * }} ScannedAddress
 */

/**
 * @typedef {Object} ScanProgress
 * @property {WalletBranchId} branchId
 * @property {string} branchLabel
 * @property {number} scannedCount
 * @property {number} unusedInRow
 */

/**
 * @typedef {Object} ScanBranchesOptions
 * @property {AddressScript} script
 * @property {(progress: ScanProgress) => void} [onProgress]
 */

/**
 * @typedef {Object} ScanBranchesResult
 * @property {ScannedAddress[]} addresses
 * @property {ScannedAddress | undefined} receiveAddress
 * @property {number} gapLimit
 * @property {boolean} maxed
 */

/**
 * @param {WalletAddress} address
 */
function isUsedAddress(address) {
  return address.received > 0 || address.sent > 0 || address.txCount > 0;
}

/**
 * @param {ScannedAddress} a
 * @param {ScannedAddress} b
 */
function compareWalletAddresses(a, b) {
  if (a.typeIndex === undefined) return 1;
  if (b.typeIndex === undefined) return -1;

  return b.typeIndex - a.typeIndex || a.index - b.index;
}

/**
 * @param {WalletAddress} address
 * @param {WalletBranch} branch
 * @returns {ScannedAddress}
 */
function addBranch(address, branch) {
  return {
    ...address,
    branchId: branch.id,
    branchLabel: branch.label,
  };
}

/**
 * @param {string} source
 */
function getWalletBranches(source) {
  if (!isOutputDescriptor(source)) return keyBranches;

  const branchIds = new Set(getOutputDescriptorBranchIds(source));
  const branches = descriptorBranches.filter((branch) => {
    return branchIds.has(branch.id);
  });

  return branches.length ? branches : [descriptorBranches[0]];
}

/**
 * @param {AddressClient} client
 * @param {string} source
 * @param {ScanBranchesOptions} options
 * @returns {Promise<ScanBranchesResult>}
 */
export async function scanBranches(client, source, options) {
  const addresses = /** @type {ScannedAddress[]} */ ([]);
  const branches = getWalletBranches(source);
  const receiveBranch =
    branches.find((branch) => branch.id === "receive") ?? branches[0];
  /** @type {ScannedAddress | undefined} */
  let receiveAddress;
  let maxed = false;

  for (const branch of branches) {
    const scan = await scanBranch(client, source, {
      script: options.script,
      path: branch.path,
      branchId: branch.id,
      onProgress(progress) {
        options.onProgress?.({
          branchId: branch.id,
          branchLabel: branch.label,
          scannedCount: progress.scannedCount,
          unusedInRow: progress.unusedInRow,
        });
      },
    });

    for (const address of scan.addresses) {
      const branchedAddress = addBranch(address, branch);

      if (!isUsedAddress(address)) {
        if (!receiveAddress && branch.id === receiveBranch.id) {
          receiveAddress = branchedAddress;
        }
        continue;
      }

      addresses.push(branchedAddress);
    }

    maxed = maxed || scan.maxed;
  }

  return {
    addresses: addresses.sort(compareWalletAddresses),
    receiveAddress,
    gapLimit: GAP_LIMIT,
    maxed,
  };
}
