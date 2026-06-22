import { createElement } from "../../dom.js";

/**
 * @typedef {Object} WalletTab
 * @property {string} id
 * @property {string} label
 * @property {HTMLElement} panel
 * @property {() => void} mount
 */

/**
 * @param {WalletTab} tab
 */
function createTabButton(tab) {
  const button = document.createElement("button");

  button.type = "button";
  button.role = "tab";
  button.id = `wallet-tab-${tab.id}`;
  button.setAttribute("aria-controls", `wallet-panel-${tab.id}`);
  button.append(tab.label);

  return button;
}

/**
 * @param {WalletTab} tab
 */
function setupTabPanel(tab) {
  tab.panel.id = `wallet-panel-${tab.id}`;
  tab.panel.role = "tabpanel";
  tab.panel.setAttribute("aria-labelledby", `wallet-tab-${tab.id}`);
}

/**
 * @param {readonly WalletTab[]} tabs
 */
export function createWalletTabs(tabs) {
  const element = createElement("section", "tabs");
  const nav = document.createElement("nav");
  const panels = document.createElement("div");
  const buttons = tabs.map(createTabButton);

  nav.setAttribute("aria-label", "Wallet sections");
  nav.role = "tablist";
  panels.append(...tabs.map((tab) => {
    setupTabPanel(tab);

    return tab.panel;
  }));
  nav.append(...buttons);
  element.append(nav, panels);

  /**
   * @param {number} selectedIndex
   */
  function select(selectedIndex) {
    for (let index = 0; index < tabs.length; index += 1) {
      const selected = index === selectedIndex;

      buttons[index].setAttribute("aria-selected", String(selected));
      buttons[index].tabIndex = selected ? 0 : -1;
      tabs[index].panel.hidden = !selected;
    }

    tabs[selectedIndex].mount();
  }

  for (let index = 0; index < buttons.length; index += 1) {
    buttons[index].addEventListener("click", () => {
      select(index);
    });
  }

  select(0);

  return element;
}
