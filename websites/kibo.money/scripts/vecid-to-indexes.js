const Addressindex = 0;
/** @typedef {typeof Addressindex} Addressindex */
const Dateindex = 1;
/** @typedef {typeof Dateindex} Dateindex */
const Height = 2;
/** @typedef {typeof Height} Height */
const P2PK33index = 3;
/** @typedef {typeof P2PK33index} P2PK33index */
const P2PK65index = 4;
/** @typedef {typeof P2PK65index} P2PK65index */
const P2PKHindex = 5;
/** @typedef {typeof P2PKHindex} P2PKHindex */
const P2SHindex = 6;
/** @typedef {typeof P2SHindex} P2SHindex */
const P2TRindex = 7;
/** @typedef {typeof P2TRindex} P2TRindex */
const P2WPKHindex = 8;
/** @typedef {typeof P2WPKHindex} P2WPKHindex */
const P2WSHindex = 9;
/** @typedef {typeof P2WSHindex} P2WSHindex */
const Txindex = 10;
/** @typedef {typeof Txindex} Txindex */
const Txinindex = 11;
/** @typedef {typeof Txinindex} Txinindex */
const Txoutindex = 12;
/** @typedef {typeof Txoutindex} Txoutindex */
const Weekindex = 13;
/** @typedef {typeof Weekindex} Weekindex */
const Monthindex = 14;
/** @typedef {typeof Monthindex} Monthindex */
const Yearindex = 15;
/** @typedef {typeof Yearindex} Yearindex */
const Decadeindex = 16;
/** @typedef {typeof Decadeindex} Decadeindex */
const Difficultyepoch = 17;
/** @typedef {typeof Difficultyepoch} Difficultyepoch */
const Halvingepoch = 18;
/** @typedef {typeof Halvingepoch} Halvingepoch */

/** @typedef {Addressindex | Dateindex | Height | P2PK33index | P2PK65index | P2PKHindex | P2SHindex | P2TRindex | P2WPKHindex | P2WSHindex | Txindex | Txinindex | Txoutindex | Weekindex | Monthindex | Yearindex | Decadeindex | Difficultyepoch | Halvingepoch} Index */

export const VecIdToIndexes = {
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
  timestamp: [Height],
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
/** @typedef {typeof VecIdToIndexes} VecIdToIndexes */
/** @typedef {keyof VecIdToIndexes} VecId */
