import { BrkClient } from "./modules/brk-client/index.js";

// const brk = new BrkClient("https://next.bitview.space");
const brk = new BrkClient("/");

console.log(`VERSION = ${brk.VERSION}`);

export { brk };
