export const localhost = window.location.hostname === "localhost";
export const standalone =
  "standalone" in window.navigator && !!window.navigator.standalone;
export const userAgent = navigator.userAgent.toLowerCase();
export const isChrome = userAgent.includes("chrome");
export const safari = userAgent.includes("safari");
export const safariOnly = safari && !isChrome;
export const macOS = userAgent.includes("mac os");
export const iphone = userAgent.includes("iphone");
export const ipad = userAgent.includes("ipad");
export const ios = iphone || ipad;
export const canShare = "canShare" in navigator;
