import {
  Accessor,
  Setter,
} from "../packages/solid-signals/2024-04-17/types/signals";
import {
  DeepPartial,
  BaselineStyleOptions,
  CandlestickStyleOptions,
  LineStyleOptions,
  SeriesOptionsCommon,
  Range,
  Time,
  SingleValueData,
  CandlestickData,
  SeriesType,
  IChartApi,
  ISeriesApi,
} from "../packages/lightweight-charts/v4.2.0/types";
import { DatePath, HeightPath } from "./paths";
import { Owner } from "../packages/solid-signals/2024-04-17/types/owner";

type Scale = "date" | "height";

type SettingsTheme = "system" | "dark" | "light";

type FoldersFilter = "all" | "favorites" | "new";

type Signal<T> = Accessor<T> & { set: Setter<T> };

type TimeRange = Range<Time | number>;

type DatasetPath<S extends Scale> = S extends "date" ? DatePath : HeightPath;

type AnyDatasetPath = import("./paths").DatePath | import("./paths").HeightPath;

type Color = () => string;

interface BaselineSpecificSeriesBlueprint {
  type: "Baseline";
  color?: Color;
  options?: DeepPartial<BaselineStyleOptions & SeriesOptionsCommon>;
}

interface CandlestickSpecificSeriesBlueprint {
  type: "Candlestick";
  color?: undefined;
  options?: DeepPartial<CandlestickStyleOptions & SeriesOptionsCommon>;
}

interface LineSpecificSeriesBlueprint {
  type?: "Line";
  color: Color;
  options?: DeepPartial<LineStyleOptions & SeriesOptionsCommon>;
}

type AnySpecificSeriesBlueprint =
  | BaselineSpecificSeriesBlueprint
  | CandlestickSpecificSeriesBlueprint
  | LineSpecificSeriesBlueprint;

type SpecificSeriesBlueprintWithChart<A extends AnySpecificSeriesBlueprint> = {
  chart: IChartApi;
  owner: Owner | null;
} & Omit<A, "type">;

type SeriesBlueprint = {
  datasetPath: AnyDatasetPath;
  title: string;
  defaultActive?: boolean;
} & AnySpecificSeriesBlueprint;

type Unit =
  | "US Dollars"
  | "Bitcoin"
  | "Percentage"
  | "Height"
  | "Count"
  | "Megabytes"
  | "Transactions"
  | "Weight"
  | "Ratio"
  | "Virtual Bytes"
  | "Seconds"
  | "Coinblocks"
  | "ExaHash / Second"
  | "Dollars / (PetaHash / Second)"
  | "";

interface PresetBlueprint {
  top?: SeriesBlueprint[];
  bottom?: SeriesBlueprint[];
}

interface PartialPreset extends PresetBlueprint {
  scale: Scale;
  icon: string;
  name: string;
  unit: Unit;
  title: string;
  description: string;
}

interface PartialPresetFolder {
  name: string;
  tree: PartialPresetTree;
}

type PartialPresetTree = (PartialPreset | PartialPresetFolder)[];

interface Preset extends PartialPreset {
  id: string;
  path: FilePath;
  serializedPath: string;
  isFavorite: Signal<boolean>;
  visited: Signal<boolean>;
}

type PresetTree = (Preset | PresetFolder)[];

interface PresetFolder extends PartialPresetFolder {
  id: string;
  tree: PresetTree;
}

type FilePath = {
  id: string;
  name: string;
}[];

type SerializedPresetsHistory = [string, number][];

interface OHLC {
  open: number;
  high: number;
  low: number;
  close: number;
}

interface ResourceDataset<
  S extends Scale,
  Type extends OHLC | number = number
> {
  scale: S;
  url: string;
  fetch: (id: number) => void;
  fetchedJSONs: FetchedResult<S, Type>[];
  // drop: VoidFunction;
}

type ValuedCandlestickData = CandlestickData & Valued;

interface FetchedResult<
  S extends Scale,
  Type extends number | OHLC,
  Value extends DatasetValue<
    SingleValueData | ValuedCandlestickData
  > = DatasetValue<
    Type extends number ? SingleValueData : ValuedCandlestickData
  >
> {
  at: Date | null;
  json: Signal<FetchedJSON<S, Type> | null>;
  vec: Accessor<Value[] | null>;
  loading: boolean;
}

interface Valued {
  value: number;
}

type DatasetValue<T> = T & Valued;

interface FetchedJSON<S extends Scale, Type extends number | OHLC> {
  source: FetchedSource;
  chunk: FetchedChunk;
  dataset: FetchedDataset<S, Type>;
}

type FetchedSource = string;

interface FetchedChunk {
  id: number;
  previous: string | null;
  next: string | null;
}

type FetchedDataset<
  S extends Scale,
  Type extends number | OHLC
> = S extends "date" ? FetchedDateDataset<Type> : FetchedHeightDataset<Type>;

interface Versioned {
  version: number;
}

interface FetchedDateDataset<Type> extends Versioned {
  map: Record<string, Type>;
}

interface FetchedHeightDataset<Type> extends Versioned {
  map: Type[];
}

type PriceSeriesType = "Candlestick" | "Line";

interface Series {
  id: string;
  title: string;
  chunks: Array<Accessor<ISeriesApi<SeriesType> | undefined>>;
  color: Color | Color[];
  disabled: Accessor<boolean>;
  active: Signal<boolean>;
  visible: Accessor<boolean>;
  dataset: ResourceDataset<Scale, number>;
}

interface Marker {
  weight: number;
  time: Time;
  value: number;
  seriesChunk: ISeriesApi<any>;
}

interface Weighted {
  weight: number;
}

type DatasetCandlestickData = DatasetValue<CandlestickData> & { year: number };
