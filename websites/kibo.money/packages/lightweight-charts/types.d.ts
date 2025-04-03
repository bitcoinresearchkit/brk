import { Signal } from "../solid-signals/types";
import { Accessor } from "../solid-signals/v0.2.4-treeshaked/types/signals";
import {
  DeepPartial,
  BaselineStyleOptions,
  CandlestickStyleOptions,
  LineStyleOptions,
  SeriesOptionsCommon,
  Time,
  ISeriesApi,
  BaselineData,
} from "./v5.0.5-treeshaked/types";
import { VecId } from "../../scripts/vecid-to-indexes";

interface BaseSeriesBlueprint {
  title: string;
  defaultActive?: boolean;
}
interface BaselineSeriesBlueprint extends BaseSeriesBlueprint {
  type: "Baseline";
  color?: Color;
  options?: DeepPartial<BaselineStyleOptions & SeriesOptionsCommon>;
  data?: Accessor<BaselineData<Time>[]>;
}
interface CandlestickSeriesBlueprint extends BaseSeriesBlueprint {
  type: "Candlestick";
  color?: Color;
  options?: DeepPartial<CandlestickStyleOptions & SeriesOptionsCommon>;
  data?: Accessor<CandlestickData[]>;
}
interface LineSeriesBlueprint extends BaseSeriesBlueprint {
  type?: "Line";
  color: Color;
  options?: DeepPartial<LineStyleOptions & SeriesOptionsCommon>;
  data?: Accessor<LineData<Time>[]>;
}
type AnySpecificSeriesBlueprint =
  | BaselineSeriesBlueprint
  | CandlestickSeriesBlueprint
  | LineSeriesBlueprint;

type SeriesType = NonNullable<AnySpecificSeriesBlueprint["type"]>;
type PriceSeriesType = "Candlestick" | "Line";

type RemoveSeriesBlueprintFluff<Blueprint extends AnySpecificSeriesBlueprint> =
  Omit<Blueprint, "type" | "title">;

type SplitSeriesBlueprint<> = {
  key: VecId;
} & AnySpecificSeriesBlueprint;

type SingleSeriesBlueprint = AnySpecificSeriesBlueprint;

interface CreateBaseSeriesParameters extends BaseSeriesBlueprint {
  id: string;
  disabled?: Accessor<boolean>;
  color?: Color;
}
interface BaseSeries {
  id: string;
  title: string;
  color: Color | Color[];
  active: Signal<boolean>;
  visible: Accessor<boolean>;
}
interface SingleSeries extends BaseSeries {
  iseries: ISeriesApi<SeriesType>;
  dataset: Accessor<(SingleValueData | CandlestickData)[] | null>;
}
interface SplitSeries extends BaseSeries {
  chunks: Array<Accessor<ISeriesApi<SeriesType> | undefined>>;
  // dataset: ResourceDataset<number>;
}
type AnySeries = SingleSeries | SplitSeries;

interface CreateSingleSeriesParameters {
  blueprint: SingleSeriesBlueprint;
  id: string;
}

interface CreateSplitSeriesParameters {
  // dataset: ResourceDataset;
  blueprint: SplitSeriesBlueprint;
  id: string;
  index: number;
  disabled?: Accessor<boolean>;
}

type ChartPane = IChartApi & {
  whitespace: ISeriesApi<"Line">;
  hidden: () => boolean;
  setHidden: (b: boolean) => void;
  setInitialVisibleTimeRange: VoidFunction;
  createSingleSeries: (a: CreateSingleSeriesParameters) => SingleSeries;
  createSplitSeries: (a: CreateSplitSeriesParameters) => SplitSeries[];
  anySeries: AnySeries[];
  singleSeries: SingleSeries[];
  splitSeries: SplitSeries[];
  remove: VoidFunction;
};

interface CreatePaneParameters {
  options?: DeepPartial<ChartOptions>;
  config?: SingleSeriesBlueprint[];
}

interface Marker {
  weight: number;
  time: Time;
  value: number;
  seriesChunk: ISeriesApi<SeriesType>;
}

interface HoveredLegend {
  label: HTMLLabelElement;
  series: AnySeries;
}
