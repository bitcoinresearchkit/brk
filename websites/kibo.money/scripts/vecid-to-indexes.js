//
// File auto-generated, any modifications will be overwritten
//

/** @typedef {0} DateIndex */
/** @typedef {1} DecadeIndex */
/** @typedef {2} DifficultyEpoch */
/** @typedef {3} EmptyOutputIndex */
/** @typedef {4} HalvingEpoch */
/** @typedef {5} Height */
/** @typedef {6} InputIndex */
/** @typedef {7} MonthIndex */
/** @typedef {8} OpReturnIndex */
/** @typedef {9} OutputIndex */
/** @typedef {10} P2AIndex */
/** @typedef {11} P2MSIndex */
/** @typedef {12} P2PK33Index */
/** @typedef {13} P2PK65Index */
/** @typedef {14} P2PKHIndex */
/** @typedef {15} P2SHIndex */
/** @typedef {16} P2TRIndex */
/** @typedef {17} P2WPKHIndex */
/** @typedef {18} P2WSHIndex */
/** @typedef {19} QuarterIndex */
/** @typedef {20} TxIndex */
/** @typedef {21} UnknownOutputIndex */
/** @typedef {22} WeekIndex */
/** @typedef {23} YearIndex */

/** @typedef {DateIndex | DecadeIndex | DifficultyEpoch | EmptyOutputIndex | HalvingEpoch | Height | InputIndex | MonthIndex | OpReturnIndex | OutputIndex | P2AIndex | P2MSIndex | P2PK33Index | P2PK65Index | P2PKHIndex | P2SHIndex | P2TRIndex | P2WPKHIndex | P2WSHIndex | QuarterIndex | TxIndex | UnknownOutputIndex | WeekIndex | YearIndex} Index */

export function createVecIdToIndexes() {
  const DateIndex = /** @satisfies {DateIndex} */ (0);
  const DecadeIndex = /** @satisfies {DecadeIndex} */ (1);
  const DifficultyEpoch = /** @satisfies {DifficultyEpoch} */ (2);
  const EmptyOutputIndex = /** @satisfies {EmptyOutputIndex} */ (3);
  const HalvingEpoch = /** @satisfies {HalvingEpoch} */ (4);
  const Height = /** @satisfies {Height} */ (5);
  const InputIndex = /** @satisfies {InputIndex} */ (6);
  const MonthIndex = /** @satisfies {MonthIndex} */ (7);
  const OpReturnIndex = /** @satisfies {OpReturnIndex} */ (8);
  const OutputIndex = /** @satisfies {OutputIndex} */ (9);
  const P2AIndex = /** @satisfies {P2AIndex} */ (10);
  const P2MSIndex = /** @satisfies {P2MSIndex} */ (11);
  const P2PK33Index = /** @satisfies {P2PK33Index} */ (12);
  const P2PK65Index = /** @satisfies {P2PK65Index} */ (13);
  const P2PKHIndex = /** @satisfies {P2PKHIndex} */ (14);
  const P2SHIndex = /** @satisfies {P2SHIndex} */ (15);
  const P2TRIndex = /** @satisfies {P2TRIndex} */ (16);
  const P2WPKHIndex = /** @satisfies {P2WPKHIndex} */ (17);
  const P2WSHIndex = /** @satisfies {P2WSHIndex} */ (18);
  const QuarterIndex = /** @satisfies {QuarterIndex} */ (19);
  const TxIndex = /** @satisfies {TxIndex} */ (20);
  const UnknownOutputIndex = /** @satisfies {UnknownOutputIndex} */ (21);
  const WeekIndex = /** @satisfies {WeekIndex} */ (22);
  const YearIndex = /** @satisfies {YearIndex} */ (23);

  return /** @type {const} */ ({
    "base-size": [TxIndex],
    blockhash: [Height],
    date: [DateIndex, Height],
    "date-fixed": [Height],
    dateindex: [DateIndex, Height],
    "dateindex-count": [MonthIndex, WeekIndex],
    decadeindex: [DecadeIndex, YearIndex],
    difficulty: [Height],
    difficultyepoch: [DifficultyEpoch, Height],
    emptyoutputindex: [EmptyOutputIndex],
    "first-dateindex": [MonthIndex, WeekIndex],
    "first-emptyoutputindex": [Height],
    "first-height": [DateIndex, DifficultyEpoch, HalvingEpoch],
    "first-inputindex": [Height, TxIndex],
    "first-monthindex": [QuarterIndex, YearIndex],
    "first-opreturnindex": [Height],
    "first-outputindex": [Height, TxIndex],
    "first-p2aindex": [Height],
    "first-p2msindex": [Height],
    "first-p2pk33index": [Height],
    "first-p2pk65index": [Height],
    "first-p2pkhindex": [Height],
    "first-p2shindex": [Height],
    "first-p2trindex": [Height],
    "first-p2wpkhindex": [Height],
    "first-p2wshindex": [Height],
    "first-txindex": [Height],
    "first-unknownoutputindex": [Height],
    "first-yearindex": [DecadeIndex],
    halvingepoch: [HalvingEpoch, Height],
    height: [Height],
    "height-count": [DateIndex, DifficultyEpoch],
    inputindex: [InputIndex],
    "is-explicitly-rbf": [TxIndex],
    "last-dateindex": [MonthIndex, WeekIndex],
    "last-height": [DateIndex, DifficultyEpoch, HalvingEpoch],
    "last-inputindex": [TxIndex],
    "last-monthindex": [QuarterIndex, YearIndex],
    "last-outputindex": [TxIndex],
    "last-txindex": [Height],
    "last-yearindex": [DecadeIndex],
    monthindex: [DateIndex, MonthIndex],
    "monthindex-count": [QuarterIndex, YearIndex],
    opreturnindex: [OpReturnIndex],
    outputindex: [InputIndex, OutputIndex],
    outputtype: [OutputIndex],
    outputtypeindex: [OutputIndex],
    p2abytes: [P2AIndex],
    p2aindex: [P2AIndex],
    p2msindex: [P2MSIndex],
    p2pk33bytes: [P2PK33Index],
    p2pk33index: [P2PK33Index],
    p2pk65bytes: [P2PK65Index],
    p2pk65index: [P2PK65Index],
    p2pkhbytes: [P2PKHIndex],
    p2pkhindex: [P2PKHIndex],
    p2shbytes: [P2SHIndex],
    p2shindex: [P2SHIndex],
    p2trbytes: [P2TRIndex],
    p2trindex: [P2TRIndex],
    p2wpkhbytes: [P2WPKHIndex],
    p2wpkhindex: [P2WPKHIndex],
    p2wshbytes: [P2WSHIndex],
    p2wshindex: [P2WSHIndex],
    quarterindex: [MonthIndex, QuarterIndex],
    rawlocktime: [TxIndex],
    timestamp: [Height],
    "timestamp-fixed": [Height],
    "total-size": [Height, TxIndex],
    txid: [TxIndex],
    txindex: [EmptyOutputIndex, OpReturnIndex, P2MSIndex, TxIndex, UnknownOutputIndex],
    "txindex-count": [Height],
    txversion: [TxIndex],
    unknownoutputindex: [UnknownOutputIndex],
    value: [OutputIndex],
    weekindex: [DateIndex, WeekIndex],
    weight: [Height],
    yearindex: [MonthIndex, YearIndex],
    "yearindex-count": [DecadeIndex],
  });
}
/** @typedef {ReturnType<typeof createVecIdToIndexes>} VecIdToIndexes */
/** @typedef {keyof VecIdToIndexes} VecId */
