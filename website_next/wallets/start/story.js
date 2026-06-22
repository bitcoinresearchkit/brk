/**
 * @param {string} text
 */
function createDetail(text) {
  const item = document.createElement("li");

  item.append(text);

  return item;
}

export function createStartStory() {
  const story = document.createElement("article");
  const title = document.createElement("h1");
  const titleBreak = document.createElement("br");
  const titleAccent = document.createElement("span");
  const lead = document.createElement("p");
  const details = document.createElement("ul");
  const warningRule = document.createElement("hr");
  const warning = document.createElement("small");

  titleAccent.append("wallets");
  title.append("Watch-only", titleBreak, titleAccent);
  lead.append("View a Bitcoin wallet privately, without spending access.");
  details.append(
    createDetail("Open xpubs and watch-only descriptors."),
    createDetail("Addresses are derived on your device."),
    createDetail("Public lookups use anonymity sets."),
    createDetail("Local servers fetch directly and are best."),
    createDetail("Save encrypted wallets, or use a temporary session."),
  );
  warning.append(
    "Use a VPN for extra network privacy.",
    document.createElement("br"),
    "On-chain address links will reduce anonymity.",
  );
  story.append(title, lead, details, warningRule, warning);

  return story;
}
