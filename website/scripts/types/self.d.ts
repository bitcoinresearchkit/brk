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
import { DatePath, HeightPath, LastPath } from "./paths";
import { Owner } from "../packages/solid-signals/2024-04-17/types/owner";

type GrowToSize<T, N extends number, A extends T[]> = A["length"] extends N
  ? A
  : GrowToSize<T, N, [...A, T]>;

type FixedArray<T, N extends number> = GrowToSize<T, N, []>;

type Signal<T> = Accessor<T> & { set: Setter<T> };

type SettingsTheme = "system" | "dark" | "light";
type FoldersFilter = "all" | "favorites" | "new";

type TimeScale = "date" | "height";

type TimeRange = Range<Time | number>;

type DatasetPath<Scale extends TimeScale> = Scale extends "date"
  ? DatePath
  : HeightPath;

type AnyDatasetPath = import("./paths").DatePath | import("./paths").HeightPath;

type AnyPath = AnyDatasetPath | LastPath;

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
  main?: boolean;
  formatNumber?: false;
} & AnySpecificSeriesBlueprint;

type Unit =
  | ""
  | "Bitcoin"
  | "Coinblocks"
  | "Count"
  | "Date"
  | "Dollars / (PetaHash / Second)"
  | "ExaHash / Second"
  | "Height"
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
  icon: string;
  name: string;
}

interface PartialHomeOption extends PartialOption {
  kind: "home";
  title: "Home";
  name: "Home";
}

interface PartialDashboardOption extends PartialOption {
  title: string;
  description: string;
  defaultOpen?: false;
  groups: {
    name: string;
    unit?: Unit;
    values: {
      name: string;
      path: LastPath;
      unit?: Unit;
      formatNumber?: false;
    }[];
  }[];
}

interface PartialChartOption extends PartialOption {
  scale: TimeScale;
  title: string;
  shortTitle?: string;
  unit: Unit;
  description: string;
  top?: SeriesBlueprint[];
  bottom?: SeriesBlueprint[];
  dashboard?: {
    ignoreName?: boolean;
    skip?: boolean;
  };
}

interface PartialPdfOption extends PartialOption {
  file: string;
}

interface PartialOptionsGroup {
  name: string;
  tree: PartialOptionsTree;
  dashboard?: {
    skip?: true;
    flatten?: true;
    hopOver?: true;
    separate?: true;
    defaultOpen?: true;
  };
}

type AnyPartialOption =
  | PartialHomeOption
  | PartialPdfOption
  | PartialDashboardOption
  | PartialChartOption;

type PartialOptionsTree = (AnyPartialOption | PartialOptionsGroup)[];

interface ProcessedOptionAddons {
  id: string;
  path: OptionPath;
  serializedPath: string;
  isFavorite: Signal<boolean>;
  visited: Signal<boolean>;
}

type OptionPath = {
  id: string;
  name: string;
}[];

type HomeOption = PartialHomeOption & ProcessedOptionAddons;

interface PdfOption extends PartialPdfOption, ProcessedOptionAddons {
  kind: "pdf";
  title: string;
}

interface DashboardOption
  extends PartialDashboardOption,
    ProcessedOptionAddons {
  kind: "dashboard";
}

interface ChartOption extends PartialChartOption, ProcessedOptionAddons {
  kind: "chart";
}

type Option = HomeOption | PdfOption | DashboardOption | ChartOption;

type OptionsTree = (Option | OptionsGroup)[];

interface OptionsGroup extends PartialOptionsGroup {
  id: string;
  tree: OptionsTree;
}

type SerializedHistory = [string, number][];

interface OHLC {
  open: number;
  high: number;
  low: number;
  close: number;
}

interface ResourceDataset<
  Scale extends TimeScale,
  Type extends OHLC | number = number,
> {
  scale: Scale;
  url: string;
  fetch: (id: number) => void;
  fetchedJSONs: FetchedResult<Scale, Type>[];
  // drop: VoidFunction;
}

type ValuedCandlestickData = CandlestickData & Valued;

interface FetchedResult<
  Scale extends TimeScale,
  Type extends number | OHLC,
  Value extends DatasetValue<
    SingleValueData | ValuedCandlestickData
  > = DatasetValue<
    Type extends number ? SingleValueData : ValuedCandlestickData
  >,
> {
  at: Date | null;
  json: Signal<FetchedJSON<Scale, Type> | null>;
  vec: Accessor<Value[] | null>;
  loading: boolean;
}

interface Valued {
  value: number;
}

type DatasetValue<T> = T & Valued;

interface FetchedJSON<Scale extends TimeScale, Type extends number | OHLC> {
  source: FetchedSource;
  chunk: FetchedChunk;
  dataset: FetchedDataset<Scale, Type>;
}

type FetchedSource = string;

interface FetchedChunk {
  id: number;
  previous: string | null;
  next: string | null;
}

type FetchedDataset<
  Scale extends TimeScale,
  Type extends number | OHLC,
> = Scale extends "date"
  ? FetchedDateDataset<Type>
  : FetchedHeightDataset<Type>;

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
  dataset: ResourceDataset<TimeScale, number>;
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

declare global {
  interface Window {
    MyNamespace: any;
  }
}
