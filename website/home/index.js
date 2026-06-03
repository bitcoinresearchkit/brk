export function createHomePage() {
  const main = document.createElement("main");
  main.className = "home";
  const title = document.createElement("h1");
  title.append("Home");
  main.append(title);
  return main;
}
