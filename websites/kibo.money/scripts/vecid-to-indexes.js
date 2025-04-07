//
// File auto-generated, any modification will be overwritten
//

/** @typedef {0} Height */
/** @typedef {1} Dateindex */
/** @typedef {2} Weekindex */
/** @typedef {3} Difficultyepoch */
/** @typedef {4} Monthindex */
/** @typedef {5} Quarterindex */
/** @typedef {6} Yearindex */
/** @typedef {7} Decadeindex */
/** @typedef {8} Halvingepoch */
/** @typedef {9} Addressindex */
/** @typedef {10} P2PK33index */
/** @typedef {11} P2PK65index */
/** @typedef {12} P2PKHindex */
/** @typedef {13} P2SHindex */
/** @typedef {14} P2TRindex */
/** @typedef {15} P2WPKHindex */
/** @typedef {16} P2WSHindex */
/** @typedef {17} Txindex */
/** @typedef {18} Txinindex */
/** @typedef {19} Txoutindex */
/** @typedef {20} Emptyindex */
/** @typedef {21} Multisigindex */
/** @typedef {22} Opreturnindex */
/** @typedef {23} Pushonlyindex */
/** @typedef {24} Unknownindex */

/** @typedef {Height | Dateindex | Weekindex | Difficultyepoch | Monthindex | Quarterindex | Yearindex | Decadeindex | Halvingepoch | Addressindex | P2PK33index | P2PK65index | P2PKHindex | P2SHindex | P2TRindex | P2WPKHindex | P2WSHindex | Txindex | Txinindex | Txoutindex | Emptyindex | Multisigindex | Opreturnindex | Pushonlyindex | Unknownindex} Index */

export function createVecIdToIndexes() {
  const Height = /** @satisfies {Height} */ (0);
  const Dateindex = /** @satisfies {Dateindex} */ (1);
  const Weekindex = /** @satisfies {Weekindex} */ (2);
  const Difficultyepoch = /** @satisfies {Difficultyepoch} */ (3);
  const Monthindex = /** @satisfies {Monthindex} */ (4);
  const Quarterindex = /** @satisfies {Quarterindex} */ (5);
  const Yearindex = /** @satisfies {Yearindex} */ (6);
  const Decadeindex = /** @satisfies {Decadeindex} */ (7);
  const Halvingepoch = /** @satisfies {Halvingepoch} */ (8);
  const Addressindex = /** @satisfies {Addressindex} */ (9);
  const P2PK33index = /** @satisfies {P2PK33index} */ (10);
  const P2PK65index = /** @satisfies {P2PK65index} */ (11);
  const P2PKHindex = /** @satisfies {P2PKHindex} */ (12);
  const P2SHindex = /** @satisfies {P2SHindex} */ (13);
  const P2TRindex = /** @satisfies {P2TRindex} */ (14);
  const P2WPKHindex = /** @satisfies {P2WPKHindex} */ (15);
  const P2WSHindex = /** @satisfies {P2WSHindex} */ (16);
  const Txindex = /** @satisfies {Txindex} */ (17);
  const Txinindex = /** @satisfies {Txinindex} */ (18);
  const Txoutindex = /** @satisfies {Txoutindex} */ (19);
  const Emptyindex = /** @satisfies {Emptyindex} */ (20);
  const Multisigindex = /** @satisfies {Multisigindex} */ (21);
  const Opreturnindex = /** @satisfies {Opreturnindex} */ (22);
  const Pushonlyindex = /** @satisfies {Pushonlyindex} */ (23);
  const Unknownindex = /** @satisfies {Unknownindex} */ (24);

  return {
    addressindex: [Txoutindex],
    addresstype: [Addressindex],
    addresstypeindex: [Addressindex],
    "base-size": [Txindex],
    "block-count": [Height],
    "block-count-sum": [Dateindex, Weekindex, Monthindex, Quarterindex, Yearindex, Decadeindex, Difficultyepoch],
    "block-interval": [Height],
    "block-interval-10p": [Dateindex],
    "block-interval-25p": [Dateindex],
    "block-interval-75p": [Dateindex],
    "block-interval-90p": [Dateindex],
    "block-interval-average": [Dateindex, Weekindex, Monthindex, Quarterindex, Yearindex, Decadeindex, Difficultyepoch],
    "block-interval-max": [Dateindex, Weekindex, Monthindex, Quarterindex, Yearindex, Decadeindex, Difficultyepoch],
    "block-interval-median": [Dateindex],
    "block-interval-min": [Dateindex, Weekindex, Monthindex, Quarterindex, Yearindex, Decadeindex, Difficultyepoch],
    "block-size": [Height],
    "block-size-sum": [Dateindex, Weekindex, Monthindex, Quarterindex, Yearindex, Decadeindex, Difficultyepoch],
    "block-vbytes": [Height],
    "block-vbytes-sum": [Dateindex, Weekindex, Monthindex, Quarterindex, Yearindex, Decadeindex, Difficultyepoch],
    "block-weight": [Height],
    "block-weight-sum": [Dateindex, Weekindex, Monthindex, Quarterindex, Yearindex, Decadeindex, Difficultyepoch],
    blockhash: [Height],
    close: [Dateindex, Height, Weekindex, Monthindex, Quarterindex, Yearindex, Decadeindex, Difficultyepoch],
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
    "first-monthindex": [Quarterindex, Yearindex],
    "first-multisigindex": [Height],
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
    height: [Addressindex, Height, P2PK33index, P2PK65index, P2PKHindex, P2SHindex, P2TRindex, P2WPKHindex, P2WSHindex, Txindex, Txinindex, Txoutindex, Emptyindex, Multisigindex, Opreturnindex, Pushonlyindex, Unknownindex],
    high: [Dateindex, Height, Weekindex, Monthindex, Quarterindex, Yearindex, Decadeindex, Difficultyepoch],
    "high-in-cents": [Dateindex, Height],
    "input-count": [Txindex],
    "input-count-sum": [Dateindex, Height, Weekindex, Monthindex, Quarterindex, Yearindex, Decadeindex, Difficultyepoch],
    "is-coinbase": [Txindex],
    "is-explicitly-rbf": [Txindex],
    "last-dateindex": [Weekindex, Monthindex],
    "last-height": [Dateindex, Difficultyepoch, Halvingepoch],
    "last-monthindex": [Quarterindex, Yearindex],
    "last-txindex": [Height],
    "last-txinindex": [Txindex],
    "last-txoutindex": [Txindex],
    "last-yearindex": [Decadeindex],
    locktime: [Txindex],
    low: [Dateindex, Height, Weekindex, Monthindex, Quarterindex, Yearindex, Decadeindex, Difficultyepoch],
    "low-in-cents": [Dateindex, Height],
    monthindex: [Dateindex, Monthindex],
    ohlc: [Dateindex, Height, Weekindex, Monthindex, Quarterindex, Yearindex, Decadeindex, Difficultyepoch],
    "ohlc-in-cents": [Dateindex, Height],
    open: [Dateindex, Height, Weekindex, Monthindex, Quarterindex, Yearindex, Decadeindex, Difficultyepoch],
    "open-in-cents": [Dateindex, Height],
    "output-count": [Txindex],
    "output-count-sum": [Dateindex, Height, Weekindex, Monthindex, Quarterindex, Yearindex, Decadeindex, Difficultyepoch],
    p2pk33addressbytes: [P2PK33index],
    p2pk65addressbytes: [P2PK65index],
    p2pkhaddressbytes: [P2PKHindex],
    p2shaddressbytes: [P2SHindex],
    p2traddressbytes: [P2TRindex],
    p2wpkhaddressbytes: [P2WPKHindex],
    p2wshaddressbytes: [P2WSHindex],
    quarterindex: [Monthindex, Quarterindex],
    "real-date": [Height],
    "sats-per-dollar": [Dateindex, Height, Weekindex, Monthindex, Quarterindex, Yearindex, Decadeindex, Difficultyepoch],
    timestamp: [Dateindex, Height, Weekindex, Monthindex, Quarterindex, Yearindex, Decadeindex, Difficultyepoch, Halvingepoch],
    "total-block-count": [Dateindex, Height, Weekindex, Monthindex, Quarterindex, Yearindex, Decadeindex, Difficultyepoch],
    "total-block-size": [Dateindex, Height, Weekindex, Monthindex, Quarterindex, Yearindex, Decadeindex, Difficultyepoch],
    "total-block-vbytes": [Dateindex, Height, Weekindex, Monthindex, Quarterindex, Yearindex, Decadeindex, Difficultyepoch],
    "total-block-weight": [Dateindex, Height, Weekindex, Monthindex, Quarterindex, Yearindex, Decadeindex, Difficultyepoch],
    "total-input-count": [Dateindex, Height, Weekindex, Monthindex, Quarterindex, Yearindex, Decadeindex, Difficultyepoch],
    "total-output-count": [Dateindex, Height, Weekindex, Monthindex, Quarterindex, Yearindex, Decadeindex, Difficultyepoch],
    "total-size": [Txindex],
    txid: [Txindex],
    txoutindex: [Txinindex],
    txversion: [Txindex],
    value: [Txoutindex],
    weekindex: [Dateindex, Weekindex],
    yearindex: [Monthindex, Yearindex],
  }
}
/** @typedef {ReturnType<typeof createVecIdToIndexes>} VecIdToIndexes */
/** @typedef {keyof VecIdToIndexes} VecId */
