import { createHoldButton } from "../hold/index.js";

/**
 * @typedef {Object} StoredWallet
 * @property {string} id
 * @property {string} name
 */

/**
 * @typedef {Object} WalletSelectorOptions
 * @property {() => string} getSelectedId
 * @property {(walletId: string) => void} onSelect
 * @property {() => void} onAdd
 * @property {() => void} onDelete
 *
 * @typedef {Object} WalletSelectorButton
 * @property {HTMLButtonElement} button
 * @property {string} id
 */

/**
 * @param {HTMLElement} walletList
 * @param {StoredWallet[]} wallets
 * @param {WalletSelectorOptions} options
 * @returns {WalletSelectorButton[]}
 */
function renderButtons(walletList, wallets, options) {
  const buttons = /** @type {WalletSelectorButton[]} */ ([]);
  const add = document.createElement("button");
  const remove = createHoldButton({
    label: "DELETE",
    title: "Hold to delete",
    className: "delete",
    onHold: options.onDelete,
  });

  walletList.replaceChildren();

  for (const wallet of wallets) {
    const button = document.createElement("button");
    const selected = wallet.id === options.getSelectedId();

    button.type = "button";
    button.setAttribute("aria-pressed", selected ? "true" : "false");
    button.append(wallet.name);
    button.addEventListener("click", () => {
      options.onSelect(wallet.id);
    });
    buttons.push({ button, id: wallet.id });
    walletList.append(button);
  }

  add.type = "button";
  add.append("ADD");
  add.addEventListener("click", options.onAdd);
  walletList.append(add);
  if (wallets.length > 0) {
    walletList.append(remove);
  }

  return buttons;
}

/**
 * @param {HTMLElement} walletList
 * @param {WalletSelectorOptions} options
 */
export function createSelector(walletList, options) {
  /** @type {WalletSelectorButton[]} */
  let buttons = [];

  function selectSnappedWallet() {
    if (buttons.length === 0) return;

    const listRect = walletList.getBoundingClientRect();
    const listCenter = listRect.left + listRect.width / 2;
    const closest = buttons.reduce((best, item) => {
      const rect = item.button.getBoundingClientRect();
      const center = rect.left + rect.width / 2;
      const distance = Math.abs(center - listCenter);

      return distance < best.distance
        ? { item, distance }
        : best;
    }, {
      item: buttons[0],
      distance: Number.POSITIVE_INFINITY,
    });

    if (closest.item.id !== options.getSelectedId()) {
      options.onSelect(closest.item.id);
    }
  }

  walletList.addEventListener("scrollend", () => {
    selectSnappedWallet();
  });

  walletList.addEventListener("wheel", (event) => {
    const delta = Math.abs(event.deltaX) > Math.abs(event.deltaY)
      ? event.deltaX
      : event.deltaY;

    if (delta === 0) return;

    const maxScrollLeft = walletList.scrollWidth - walletList.clientWidth;
    const nextScrollLeft = Math.max(
      0,
      Math.min(maxScrollLeft, walletList.scrollLeft + delta),
    );

    if (nextScrollLeft === walletList.scrollLeft) return;

    event.preventDefault();
    walletList.scrollLeft = nextScrollLeft;
  }, { passive: false });

  return {
    clear() {
      walletList.replaceChildren();
      buttons = [];
    },
    /**
     * @param {StoredWallet[]} wallets
     */
    render(wallets) {
      buttons = renderButtons(walletList, wallets, options);
    },
  };
}
