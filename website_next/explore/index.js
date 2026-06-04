export function createExplorePage() {
  const main = document.createElement("main");
  main.className = "explore";
  const title = document.createElement("h1");
  title.append("Explore");
  main.append(title);
  return main;
}
