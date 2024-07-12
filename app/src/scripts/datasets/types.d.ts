type Datasets = ReturnType<typeof import("./index").createDatasets>;

type ResourceScale = (typeof import("./index").scales)[index];

type DatasetValue<T> = T & Valued;

interface ResourceDataset<
  Scale extends ResourceScale,
  Type extends OHLC | number = number,
  FetchedDataset extends
    | FetchedDateDataset<Type>
    | FetchedHeightDataset<Type> = Scale extends "date"
    ? FetchedDateDataset<Type>
    : FetchedHeightDataset<Type>,
  Value extends SingleValueData | CandlestickData = Type extends number
    ? SingleValueData
    : CandlestickData,
> {
  scale: Scale;
  url: string;
  fetch: (id: number) => void;
  fetchedJSONs: FetchedResult<Scale, Type>[];
  drop: VoidFunction;
}

interface FetchedResult<
  Scale extends ResourceScale,
  Type extends number | OHLC,
  Dataset extends
    | FetchedDateDataset<Type>
    | FetchedHeightDataset<Type> = Scale extends "date"
    ? FetchedDateDataset<Type>
    : FetchedHeightDataset<Type>,
  Value extends DatasetValue<SingleValueData | CandlestickData> = DatasetValue<
    Type extends number ? SingleValueData : CandlestickData
  >,
> {
  at: Date | null;
  json: RWS<FetchedJSON<Scale, Type, Dataset> | null>;
  vec: Accessor<Value[] | null>;
  loading: boolean;
}

interface FetchedJSON<
  Scale extends ResourceScale,
  Type extends number | OHLC,
  Dataset extends
    | FetchedDateDataset<Type>
    | FetchedHeightDataset<Type> = Scale extends "date"
    ? FetchedDateDataset<Type>
    : FetchedHeightDataset<Type>,
> {
  source: FetchedSource;
  chunk: FetchedChunk;
  dataset: FetchedDataset<Scale, Type, Dataset>;
}

type FetchedSource = string;

interface FetchedChunk {
  id: number;
  previous: string | null;
  next: string | null;
}

interface FetchedDataset<
  Scale extends ResourceScale,
  Type extends number | OHLC,
  Dataset extends
    | FetchedDateDataset<Type>
    | FetchedHeightDataset<Type> = Scale extends "date"
    ? FetchedDateDataset<Type>
    : FetchedHeightDataset<Type>,
> {
  version: number;
  map: Dataset;
}

type FetchedDateDataset<T> = Record<string, T>;
type FetchedHeightDataset<T> = T[];

interface OHLC {
  open: number;
  high: number;
  low: number;
  close: number;
}

type GroupedKeysToURLPath =
  typeof import("/src/../../datasets/grouped_keys_to_url_path.json");

type DateDatasetPath = import("/src/../../datasets/paths").DatePath;

type HeightDatasetPath = import("/src/../../datasets/paths").HeightPath;

type LastDataPath = import("/src/../../datasets/paths").LastPath;

type DatasetPath<Scale extends ResourceScale> = Scale extends "date"
  ? DateDatasetPath
  : HeightDatasetPath;

type AnyDatasetPath = DateDatasetPath | HeightDatasetPath;
