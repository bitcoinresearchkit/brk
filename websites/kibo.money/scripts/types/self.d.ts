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
  SingleValueData as _SingleValueData,
  CandlestickData as _CandlestickData,
  SeriesType,
  ISeriesApi,
  BaselineData,
} from "../../packages/lightweight-charts/v5.0.5/types";
import { AnyPossibleCohortId, Groups } from "../options";

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

interface PartialChartOption extends PartialOption {
  title?: string;
  unit?: string;
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
  unit: string;
}

type Option = UrlOption | ChartOption | SimulationOption;

type OptionsTree = (Option | OptionsGroup)[];

interface OptionsGroup extends PartialOptionsGroup {
  id: string;
  tree: OptionsTree;
}

type OHLCTuple = [number, number, number, number];

interface Valued {
  value: number;
}
interface Indexed {
  index: number;
}
type ChartData<T> = T & Valued & Indexed;
type SingleValueData = ChartData<_SingleValueData>;
type CandlestickData = ChartData<_CandlestickData>;

type FetchedSource = string;

interface FetchedChunk {
  id: number;
  previous: string | null;
  next: string | null;
}

interface Weighted {
  weight: number;
}

type DatasetCandlestickData = ChartData<CandlestickData>;

// type NotFunction<T> = T extends Function ? never : T;

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
