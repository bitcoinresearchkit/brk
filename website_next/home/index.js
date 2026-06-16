const links = [
  { href: "/explore", label: "Explore" },
  { href: "/learn", label: "Learn" },
  { href: "/build", label: "Build" },
  { href: "/wallets", label: "Wallets" },
];

export function createHomePage() {
  const main = document.createElement("main");
  main.className = "home";

  const title = document.createElement("h1");
  const nav = document.createElement("nav");

  nav.setAttribute("aria-label", "Sections");
  title.append("bitview");

  for (const { href, label } of links) {
    const link = document.createElement("a");
    link.href = href;
    link.append(label);
    nav.append(link);
  }

  main.append(title, nav);
  return main;
}
