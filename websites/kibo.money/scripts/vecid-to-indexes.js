/** @typedef {0} Addressindex */
/** @typedef {1} Dateindex */
/** @typedef {2} Height */
/** @typedef {3} P2PK33index */
/** @typedef {4} P2PK65index */
/** @typedef {5} P2PKHindex */
/** @typedef {6} P2SHindex */
/** @typedef {7} P2TRindex */
/** @typedef {8} P2WPKHindex */
/** @typedef {9} P2WSHindex */
/** @typedef {10} Txindex */
/** @typedef {11} Txinindex */
/** @typedef {12} Txoutindex */
/** @typedef {13} Weekindex */
/** @typedef {14} Monthindex */
/** @typedef {15} Yearindex */
/** @typedef {16} Decadeindex */
/** @typedef {17} Difficultyepoch */
/** @typedef {18} Halvingepoch */

/** @typedef {Addressindex | Dateindex | Height | P2PK33index | P2PK65index | P2PKHindex | P2SHindex | P2TRindex | P2WPKHindex | P2WSHindex | Txindex | Txinindex | Txoutindex | Weekindex | Monthindex | Yearindex | Decadeindex | Difficultyepoch | Halvingepoch} Index */

export function createVecIdToIndexes() {
  const Addressindex = /** @satisfies {Addressindex} */ (0);
  const Dateindex = /** @satisfies {Dateindex} */ (1);
  const Height = /** @satisfies {Height} */ (2);
  const P2PK33index = /** @satisfies {P2PK33index} */ (3);
  const P2PK65index = /** @satisfies {P2PK65index} */ (4);
  const P2PKHindex = /** @satisfies {P2PKHindex} */ (5);
  const P2SHindex = /** @satisfies {P2SHindex} */ (6);
  const P2TRindex = /** @satisfies {P2TRindex} */ (7);
  const P2WPKHindex = /** @satisfies {P2WPKHindex} */ (8);
  const P2WSHindex = /** @satisfies {P2WSHindex} */ (9);
  const Txindex = /** @satisfies {Txindex} */ (10);
  const Txinindex = /** @satisfies {Txinindex} */ (11);
  const Txoutindex = /** @satisfies {Txoutindex} */ (12);
  const Weekindex = /** @satisfies {Weekindex} */ (13);
  const Monthindex = /** @satisfies {Monthindex} */ (14);
  const Yearindex = /** @satisfies {Yearindex} */ (15);
  const Decadeindex = /** @satisfies {Decadeindex} */ (16);
  const Difficultyepoch = /** @satisfies {Difficultyepoch} */ (17);
  const Halvingepoch = /** @satisfies {Halvingepoch} */ (18);

  return {
    addressindex: [Txoutindex],
    addresstype: [Addressindex],
    addresstypeindex: [Addressindex],
    "base-size": [Txindex],
    "block-count": [Dateindex],
    "block-interval": [Height],
    "block-interval-10p": [Dateindex],
    "block-interval-25p": [Dateindex],
    "block-interval-75p": [Dateindex],
    "block-interval-90p": [Dateindex],
    "block-interval-average": [Dateindex, Weekindex, Monthindex, Yearindex, Decadeindex, Difficultyepoch],
    "block-interval-max": [Dateindex, Weekindex, Monthindex, Yearindex, Decadeindex, Difficultyepoch],
    "block-interval-median": [Dateindex],
    "block-interval-min": [Dateindex, Weekindex, Monthindex, Yearindex, Decadeindex, Difficultyepoch],
    blockhash: [Height],
    close: [Dateindex, Height, Weekindex, Monthindex, Yearindex, Decadeindex, Difficultyepoch],
    "close-in-cents": [Dateindex, Height],
    date: [Dateindex],
    dateindex: [Dateindex, Height],
    decadeindex: [Yearindex, Decadeindex],
    difficulty: [Height],
    difficultyepoch: [Height, Difficultyepoch],
    "first-addressindex": [Height],
    "first-dateindex": [Weekindex, Monthindex],
    "first-emptyindex": [Height],
    "first-height": [Dateindex, Difficultyepoch, Halvingepoch],
    "first-monthindex": [Yearindex],
    "first-multisigindex": [Height],
    "first-open": [Weekindex, Monthindex, Yearindex, Decadeindex, Difficultyepoch],
    "first-opreturnindex": [Height],
    "first-p2pk33index": [Height],
    "first-p2pk65index": [Height],
    "first-p2pkhindex": [Height],
    "first-p2shindex": [Height],
    "first-p2trindex": [Height],
    "first-p2wpkhindex": [Height],
    "first-p2wshindex": [Height],
    "first-pushonlyindex": [Height],
    "first-txindex": [Height],
    "first-txinindex": [Height, Txindex],
    "first-txoutindex": [Height, Txindex],
    "first-unkownindex": [Height],
    "first-yearindex": [Decadeindex],
    "fixed-date": [Height],
    "fixed-timestamp": [Height],
    halvingepoch: [Height, Halvingepoch],
    height: [Addressindex, Height, Txindex],
    high: [Dateindex, Height],
    "high-in-cents": [Dateindex, Height],
    "high-max": [Weekindex, Monthindex, Yearindex, Decadeindex, Difficultyepoch],
    "inputs-count": [Txindex],
    "is-coinbase": [Txindex],
    "is-explicitly-rbf": [Txindex],
    "last-dateindex": [Weekindex, Monthindex],
    "last-height": [Dateindex, Difficultyepoch, Halvingepoch],
    "last-monthindex": [Yearindex],
    "last-txindex": [Height],
    "last-txinindex": [Txindex],
    "last-txoutindex": [Txindex],
    "last-yearindex": [Decadeindex],
    locktime: [Txindex],
    low: [Dateindex, Height],
    "low-in-cents": [Dateindex, Height],
    "low-min": [Weekindex, Monthindex, Yearindex, Decadeindex, Difficultyepoch],
    monthindex: [Dateindex, Monthindex],
    ohlc: [Dateindex, Height, Weekindex, Monthindex, Yearindex, Decadeindex, Difficultyepoch],
    "ohlc-in-cents": [Dateindex, Height],
    open: [Dateindex, Height],
    "open-in-cents": [Dateindex, Height],
    "outputs-count": [Txindex],
    p2pk33addressbytes: [P2PK33index],
    p2pk65addressbytes: [P2PK65index],
    p2pkhaddressbytes: [P2PKHindex],
    p2shaddressbytes: [P2SHindex],
    p2traddressbytes: [P2TRindex],
    p2wpkhaddressbytes: [P2WPKHindex],
    p2wshaddressbytes: [P2WSHindex],
    "real-date": [Height],
    "sats-per-dollar": [Dateindex, Height],
    size: [Height],
    timestamp: [Dateindex, Height, Weekindex, Monthindex, Yearindex, Decadeindex, Difficultyepoch, Halvingepoch],
    "total-block-count": [Dateindex],
    "total-size": [Txindex],
    txid: [Txindex],
    txoutindex: [Txinindex],
    txversion: [Txindex],
    value: [Txoutindex],
    weekindex: [Dateindex, Weekindex],
    weight: [Height],
    yearindex: [Monthindex, Yearindex],
  }
}
/** @typedef {ReturnType<typeof createVecIdToIndexes>} VecIdToIndexes */
/** @typedef {keyof VecIdToIndexes} VecId */
