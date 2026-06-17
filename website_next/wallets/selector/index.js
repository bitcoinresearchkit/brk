/**
 * @typedef {Object} StoredWallet
 * @property {string} id
 * @property {string} name
 */

/**
 * @typedef {Object} WalletSelectorOptions
 * @property {() => string} getSelectedId
 * @property {(walletId: string) => void} onSelect
 */

/**
 * @param {HTMLElement} walletList
 * @param {StoredWallet[]} wallets
 * @param {WalletSelectorOptions} options
 */
function renderButtons(walletList, wallets, options) {
  walletList.replaceChildren();

  for (const wallet of wallets) {
    const button = document.createElement("button");
    const selected = wallet.id === options.getSelectedId();

    button.type = "button";
    button.className = "wallets__wallet-button";
    button.setAttribute("aria-pressed", selected ? "true" : "false");
    button.setAttribute("data-wallet-id", wallet.id);
    button.append(wallet.name);
    button.addEventListener("click", () => {
      options.onSelect(wallet.id);
    });
    walletList.append(button);
  }
}

/**
 * @param {HTMLElement} walletList
 * @param {WalletSelectorOptions} options
 */
export function createSelector(walletList, options) {
  function selectSnappedWallet() {
    const buttons = [...walletList.querySelectorAll(".wallets__wallet-button")];

    if (buttons.length === 0) return;

    const listRect = walletList.getBoundingClientRect();
    const listCenter = listRect.left + listRect.width / 2;
    const closest = buttons.reduce((best, button) => {
      const rect = button.getBoundingClientRect();
      const center = rect.left + rect.width / 2;
      const distance = Math.abs(center - listCenter);

      return distance < best.distance
        ? { button, distance }
        : best;
    }, {
      button: buttons[0],
      distance: Number.POSITIVE_INFINITY,
    });
    const id = closest.button.getAttribute("data-wallet-id");

    if (id && id !== options.getSelectedId()) {
      options.onSelect(id);
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
    },
    /**
     * @param {StoredWallet[]} wallets
     */
    render(wallets) {
      renderButtons(walletList, wallets, options);
    },
  };
}
