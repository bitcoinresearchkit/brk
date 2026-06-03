export function createBuildPage() {
  const main = document.createElement("main");
  main.className = "build";
  const title = document.createElement("h1");
  title.append("Build");
  main.append(title);
  return main;
}
