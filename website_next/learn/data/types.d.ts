import { rollingWindows } from "./rolling-windows.js";

declare global {
  type RollingWindowKey = (typeof rollingWindows)[number][1];
}

export {};
