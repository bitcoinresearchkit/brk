import { BrkClient } from "../index.js";

let client = new BrkClient("http://localhost:3110");

let blocks = await client.getBlocks();
