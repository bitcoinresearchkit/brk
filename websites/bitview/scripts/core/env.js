const localhost = window.location.hostname === "localhost";
const standalone =
  "standalone" in window.navigator && !!window.navigator.standalone;
const userAgent = navigator.userAgent.toLowerCase();
const isChrome = userAgent.includes("chrome");
const safari = userAgent.includes("safari");
const safariOnly = safari && !isChrome;
const macOS = userAgent.includes("mac os");
const iphone = userAgent.includes("iphone");
const ipad = userAgent.includes("ipad");
const ios = iphone || ipad;

export default {
  standalone,
  userAgent,
  isChrome,
  safari,
  safariOnly,
  macOS,
  iphone,
  ipad,
  ios,
  localhost,
};
