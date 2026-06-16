import {
  scanXpubWallet,
  XPUB_GAP_LIMIT,
} from "./xpub-scan.js";
import {
  getOutputDescriptorBranchIds,
  isOutputDescriptor,
} from "../xpub/index.js";

export const xpubWalletBranches = /** @type {const} */ ([
  { id: "receive", label: "Receive", path: [0] },
  { id: "change", label: "Change", path: [1] },
  { id: "direct", label: "Direct", path: [] },
]);

const descriptorWalletBranches = /** @type {const} */ ([
  { id: "receive", label: "Receive", path: [] },
  { id: "change", label: "Change", path: [] },
]);

const UNKNOWN_TYPE_INDEX = Number.MAX_SAFE_INTEGER;

/**
 * @typedef {(typeof xpubWalletBranches[number] | typeof descriptorWalletBranches[number])} WalletBranch
 * @typedef {WalletBranch["id"]} WalletBranchId
 */

/**
 * @typedef {import("../xpub/address.js").AddressScript} AddressScript
 * @typedef {import("../xpub/index.js").AddressType} AddressType
 */

/**
 * @typedef {Object} AddressClient
 * @property {(address: string, options?: { cache?: boolean }) => Promise<unknown>} getAddress
 * @property {(addrType: AddressType, prefix: string, options?: { cache?: boolean }) => Promise<unknown>} getAddressHashPrefixMatches
 */

/**
 * @typedef {Object} WalletAddress
 * @property {number} index
 * @property {string} address
 * @property {string} script
 * @property {string} network
 * @property {AddressType} addrType
 * @property {number} balance
 * @property {number} received
 * @property {number} sent
 * @property {number} txCount
 * @property {number | undefined} typeIndex
 * @property {string[]} historyAddresses
 * @property {number} historyBucketSize
 */

/**
 * @typedef {WalletAddress & {
 *   branchId: WalletBranchId,
 *   branchLabel: string,
 * }} XpubWalletAddress
 */

/**
 * @typedef {Object} ScanProgress
 * @property {WalletBranchId} branchId
 * @property {string} branchLabel
 * @property {number} scannedCount
 * @property {number} unusedInRow
 */

/**
 * @typedef {Object} ScanXpubWalletOptions
 * @property {number} start
 * @property {AddressScript} script
 * @property {(progress: ScanProgress) => void} [onProgress]
 */

/**
 * @typedef {Object} ScanXpubWalletResult
 * @property {XpubWalletAddress[]} addresses
 * @property {XpubWalletAddress | undefined} receiveAddress
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
 * @param {XpubWalletAddress} address
 */
function getSortIndex(address) {
  return address.typeIndex ?? UNKNOWN_TYPE_INDEX;
}

/**
 * @param {XpubWalletAddress} a
 * @param {XpubWalletAddress} b
 */
function compareWalletAddresses(a, b) {
  return getSortIndex(a) - getSortIndex(b) || a.index - b.index;
}

/**
 * @param {WalletAddress} address
 * @param {WalletBranch} branch
 * @returns {XpubWalletAddress}
 */
function addBranch(address, branch) {
  return {
    ...address,
    branchId: branch.id,
    branchLabel: branch.label,
  };
}

/**
 * @param {string} xpub
 */
function getWalletBranches(xpub) {
  if (!isOutputDescriptor(xpub)) return xpubWalletBranches;

  const branchIds = new Set(getOutputDescriptorBranchIds(xpub));
  const branches = descriptorWalletBranches.filter((branch) => {
    return branchIds.has(branch.id);
  });

  return branches.length ? branches : [descriptorWalletBranches[0]];
}

/**
 * @param {AddressClient} client
 * @param {string} xpub
 * @param {ScanXpubWalletOptions} options
 * @returns {Promise<ScanXpubWalletResult>}
 */
export async function scanXpubBranches(client, xpub, options) {
  const addresses = /** @type {XpubWalletAddress[]} */ ([]);
  const branches = getWalletBranches(xpub);
  const receiveBranch =
    branches.find((branch) => branch.id === "receive") ?? branches[0];
  /** @type {XpubWalletAddress | undefined} */
  let receiveAddress;
  let maxed = false;

  for (const branch of branches) {
    const scan = await scanXpubWallet(client, xpub, {
      start: options.start,
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
    gapLimit: XPUB_GAP_LIMIT,
    maxed,
  };
}
