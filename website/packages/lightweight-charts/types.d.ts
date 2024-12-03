import { Signal } from "../solid-signals/types";
import { Accessor } from "../solid-signals/2024-11-02/types/signals";
import { Owner } from "../solid-signals/2024-11-02/types/core/owner";
import {
  DeepPartial,
  BaselineStyleOptions,
  CandlestickStyleOptions,
  LineStyleOptions,
  SeriesOptionsCommon,
  Time,
  CandlestickData,
  ISeriesApi,
  BaselineData,
} from "./v4.2.0/types";
import { Color } from "../../scripts/types/self";

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
  data?: Accessor<CandlestickData<Time>[]>;
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

type SplitSeriesBlueprint = {
  datasetPath: AnyDatasetPath;
  main?: boolean;
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
  disabled: Accessor<boolean>;
}
interface SingleSeries extends BaseSeries {
  iseries: ISeriesApi<any>;
}
interface SplitSeries extends BaseSeries {
  chunks: Array<Accessor<ISeriesApi<SeriesType> | undefined>>;
  dataset: ResourceDataset<TimeScale, number>;
}

interface CreateSingleSeriesParameters {
  blueprint: SingleSeriesBlueprint;
  id: string;
}

interface CreateSplitSeriesParameters<S extends TimeScale> {
  dataset: ResourceDataset<S>;
  blueprint: SplitSeriesBlueprint;
  id: string;
  index: number;
  setMinMaxMarkersWhenIdle: VoidFunction;
  disabled?: Accessor<boolean>;
}

type ChartPane = IChartApi & {
  whitespace: ISeriesApi<"Line">;
  hidden: () => boolean;
  setHidden: (b: boolean) => void;
  setInitialVisibleTimeRange: VoidFunction;
  createSingleSeries: (a: CreateSingleSeriesParameters) => SingleSeries;
  createSplitSeries: <S extends TimeScale>(
    a: CreateSplitSeriesParameters<S>,
  ) => SplitSeries;
  splitSeries: SplitSeries[];
};

interface CreatePaneParameters {
  unit: Unit;
  paneIndex?: number;
  whitespace?: true;
  options?: DeepPartial<ChartOptions>;
  config?: SingleSeriesBlueprint[];
}

interface Marker {
  weight: number;
  time: Time;
  value: number;
  seriesChunk: ISeriesApi<any>;
}

interface HoveredLegend {
  label: HTMLLabelElement;
  series: SingleSeries | SplitSeries;
}
