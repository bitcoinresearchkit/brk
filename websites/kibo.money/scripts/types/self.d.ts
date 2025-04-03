import { Accessor } from "../../packages/solid-signals/v0.2.4-treeshaked/types/signals";
import {
  DeepPartial,
  BaselineStyleOptions,
  CandlestickStyleOptions,
  LineStyleOptions,
  SeriesOptionsCommon,
  Time,
  SingleValueData as _SingleValueData,
  CandlestickData as _CandlestickData,
  BaselineData,
} from "../../packages/lightweight-charts/v5.0.5-treeshaked/types";
import { AnyPossibleCohortId } from "../options";

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

interface BaseSeriesBlueprint {
  title: string;
  key: VecId;
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
type AnySeriesBlueprint =
  | BaselineSeriesBlueprint
  | CandlestickSeriesBlueprint
  | LineSeriesBlueprint;

interface PartialChartOption extends PartialOption {
  title?: string;
  unit?: Unit;
  top?: AnySeriesBlueprint[];
  bottom?: AnySeriesBlueprint[];
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
  unit: Unit;
}

type Option = UrlOption | ChartOption | SimulationOption;

type OptionsTree = (Option | OptionsGroup)[];

interface OptionsGroup extends PartialOptionsGroup {
  id: string;
  tree: OptionsTree;
}

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
