import {
  Accessor,
  Setter,
} from "../../packages/solid-signals/2024-11-02/types/signals";
import {
  DeepPartial,
  BaselineStyleOptions,
  CandlestickStyleOptions,
  LineStyleOptions,
  SeriesOptionsCommon,
  IRange,
  Time,
  SingleValueData,
  CandlestickData,
  SeriesType,
  ISeriesApi,
  BaselineData,
} from "../../packages/lightweight-charts/v5.0.4/types";
import { AnyPossibleCohortId, Groups } from "../options";
import { Index as _Index, VecIdToIndexes } from "./vecid-to-indexes";
import { Signal } from "../../packages/solid-signals/types";

// type TimeScale = "date" | "height";

type TimeRange = IRange<Time | number>;

// type DatasetPath<Scale extends TimeScale> = Scale extends "date"
//   ? DatePath
//   : HeightPath;

// type AnyDatasetPath = import("./paths").DatePath | import("./paths").HeightPath;

// type AnyPath = AnyDatasetPath | LastPath;

type Color = () => string;
type ColorName = keyof Colors;

// TODO: Compute from VecId when displaying the Unit
// And write a checker when localhost, similar to the dup one
type Unit =
  | ""
  | "Bitcoin"
  | "Coinblocks"
  | "Count"
  | "Date"
  | "Dollars / (PetaHash / Second)"
  | "ExaHash / Second"
  | "Height"
  | "Gigabytes"
  | "Megabytes"
  | "Percentage"
  | "Ratio"
  | "Satoshis"
  | "Seconds"
  | "Transactions"
  | "US Dollars"
  | "Virtual Bytes"
  | "Weight";

interface PartialOption {
  name: string;
}

type DatasetId = keyof VecIdToIndexes;

interface PartialChartOption extends PartialOption {
  title?: string;
  top?: SplitSeriesBlueprint[];
  bottom?: SplitSeriesBlueprint[];
}

interface PartialSimulationOption extends PartialOption {
  kind: "simulation";
  title: string;
  name: string;
}

interface PartialUrlOption extends PartialOption {
  qrcode?: true;
  url: () => string;
}

interface PartialOptionsGroup {
  name: string;
  tree: PartialOptionsTree;
}

type AnyPartialOption =
  | PartialChartOption
  | PartialSimulationOption
  | PartialUrlOption;

type PartialOptionsTree = (AnyPartialOption | PartialOptionsGroup)[];

interface ProcessedOptionAddons {
  id: string;
  path: string[];
  title: string;
}

type SimulationOption = PartialSimulationOption & ProcessedOptionAddons;

interface UrlOption extends PartialUrlOption, ProcessedOptionAddons {
  kind: "url";
}

interface ChartOption
  extends Omit<PartialChartOption, "title">,
    ProcessedOptionAddons {
  kind: "chart";
}

type Option = UrlOption | ChartOption | SimulationOption;

type OptionsTree = (Option | OptionsGroup)[];

interface OptionsGroup extends PartialOptionsGroup {
  id: string;
  tree: OptionsTree;
}

interface OHLC {
  open: number;
  high: number;
  low: number;
  close: number;
}

interface VecResource<Type extends OHLC | number = number> {
  url: string;
  fetch: (from: number, to: number) => Promise<void>;
  ranges: FetchedVecRange<Type>[];
}

type ValuedCandlestickData = CandlestickData & Valued;

interface FetchedVecRange<
  Value extends number | OHLC,
  Data extends SingleValueData | ValuedCandlestickData = Value extends number
    ? SingleValueData
    : ValuedCandlestickData,
> {
  at: Date | null;
  fetched: Signal<Value[] | null>;
  transformed: Accessor<Data[] | null>;
  loading: boolean;
}

interface Valued {
  value: number;
}

type DatasetValue<T> = T & Valued;

type FetchedSource = string;

interface FetchedChunk {
  id: number;
  previous: string | null;
  next: string | null;
}

interface Weighted {
  weight: number;
}

type DatasetCandlestickData = DatasetValue<CandlestickData> & { year: number };

type NotFunction<T> = T extends Function ? never : T;

type DefaultCohortOption = CohortOption<AnyPossibleCohortId>;

interface CohortOption<Id extends AnyPossibleCohortId> {
  name: string;
  title: string;
  datasetId: Id;
  color: Color;
  filenameAddon?: string;
}

type DefaultCohortOptions = CohortOptions<AnyPossibleCohortId>;

interface CohortOptions<Id extends AnyPossibleCohortId> {
  name: string;
  title: string;
  list: CohortOption<Id>[];
}

interface RatioOption {
  color: Color;
  // valueDatasetPath: AnyDatasetPath;
  // ratioDatasetPath: AnyDatasetPath;
  title: string;
}

interface RatioOptions {
  title: string;
  list: RatioOption[];
}

// TODO: Remove
// Fetch last of each individually when in viewport
// type LastValues = Record<LastPath, number> | null;

type Timestamp = -1;
type Index = _Index | Timestamp;
