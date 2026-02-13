export const localhost = window.location.hostname === "localhost";
const userAgent = navigator.userAgent.toLowerCase();
const iphone = userAgent.includes("iphone");
const ipad = userAgent.includes("ipad");
export const ios = iphone || ipad;
export const canShare = "canShare" in navigator;
