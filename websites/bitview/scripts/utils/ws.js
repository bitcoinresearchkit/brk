import signals from "../signals.js";

/**
 * @template T
 * @param {(callback: (value: T) => void) => WebSocket} creator
 */
function createWebsocket(creator) {
  let ws = /** @type {WebSocket | null} */ (null);

  const live = signals.createSignal(false);
  const latest = signals.createSignal(/** @type {T | null} */ (null));

  function reinitWebSocket() {
    if (!ws || ws.readyState === ws.CLOSED) {
      console.log("ws: reinit");
      resource.open();
    }
  }

  function reinitWebSocketIfDocumentNotHidden() {
    !window.document.hidden && reinitWebSocket();
  }

  const resource = {
    live,
    latest,
    open() {
      ws = creator((value) => latest.set(() => value));

      ws.addEventListener("open", () => {
        console.log("ws: open");
        live.set(true);
      });

      ws.addEventListener("close", () => {
        console.log("ws: close");
        live.set(false);
      });

      window.document.addEventListener(
        "visibilitychange",
        reinitWebSocketIfDocumentNotHidden,
      );

      window.document.addEventListener("online", reinitWebSocket);
    },
    close() {
      ws?.close();
      window.document.removeEventListener(
        "visibilitychange",
        reinitWebSocketIfDocumentNotHidden,
      );
      window.document.removeEventListener("online", reinitWebSocket);
      live.set(false);
      ws = null;
    },
  };

  return resource;
}

/**
 * @param {(candle: CandlestickData) => void} callback
 */
function krakenCandleWebSocketCreator(callback) {
  const ws = new WebSocket("wss://ws.kraken.com/v2");

  ws.addEventListener("open", () => {
    ws.send(
      JSON.stringify({
        method: "subscribe",
        params: {
          channel: "ohlc",
          symbol: ["BTC/USD"],
          interval: 1440,
        },
      }),
    );
  });

  ws.addEventListener("message", (message) => {
    const result = JSON.parse(message.data);

    if (result.channel !== "ohlc") return;

    const { interval_begin, open, high, low, close } = result.data.at(-1);

    /** @type {CandlestickData} */
    const candle = {
      // index: -1,
      time: new Date(interval_begin).valueOf() / 1000,
      open: Number(open),
      high: Number(high),
      low: Number(low),
      close: Number(close),
    };

    candle && callback({ ...candle });
  });

  return ws;
}

/** @type {ReturnType<typeof createWebsocket<CandlestickData>>} */
const kraken1dCandle = createWebsocket((callback) =>
  krakenCandleWebSocketCreator(callback),
);

kraken1dCandle.open();

export const webSockets = {
  kraken1dCandle,
};
/** @typedef {typeof webSockets} WebSockets */
