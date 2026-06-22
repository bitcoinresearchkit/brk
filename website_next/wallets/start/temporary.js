/**
 * @param {() => void} onStart
 */
export function createTemporaryVault(onStart) {
  const temporary = document.createElement("section");
  const title = document.createElement("h2");
  const text = document.createElement("p");
  const button = document.createElement("button");

  temporary.dataset.mode = "temporary";
  title.append("Temporary vault");
  text.append("Wallets are never saved to this browser.");
  button.type = "button";
  button.append("Start temporary");
  button.addEventListener("click", onStart);
  temporary.append(title, text, button);

  return temporary;
}
