//! Python base client and pattern factory generation.

use std::fmt::Write;

use crate::{
    ClientConstants, ClientMetadata, CohortConstants, IndexSetPattern, PythonSyntax,
    StructuralPattern, format_json, generate_parameterized_field, index_to_field_name,
};

/// Generate class-level constants for the BrkClient class.
pub fn generate_class_constants(output: &mut String) {
    let constants = ClientConstants::collect();

    // VERSION
    writeln!(output, "    VERSION = \"{}\"\n", constants.version).unwrap();

    // INDEXES, POOL_ID_TO_POOL_NAME
    write_class_const(output, "INDEXES", &format_json(&constants.indexes));
    // Python needs string keys for pool map
    let pool_map: std::collections::BTreeMap<String, &str> = constants
        .pool_map
        .iter()
        .map(|(k, v)| (k.to_string(), *v))
        .collect();
    write_class_const(output, "POOL_ID_TO_POOL_NAME", &format_json(&pool_map));

    // Cohort constants (no camelCase conversion for Python)
    for (name, value) in CohortConstants::all() {
        write_class_const(output, name, &format_json(&value));
    }
}

fn write_class_const(output: &mut String, name: &str, json: &str) {
    let indented = json
        .lines()
        .enumerate()
        .map(|(i, line)| {
            if i == 0 {
                format!("    {} = {}", name, line)
            } else {
                format!("    {}", line)
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    writeln!(output, "{}\n", indented).unwrap();
}

/// Generate the base BrkClient class with HTTP functionality
pub fn generate_base_client(output: &mut String) {
    writeln!(
        output,
        r#"class BrkError(Exception):
    """Custom error class for BRK client errors."""

    def __init__(self, message: str, status: Optional[int] = None):
        super().__init__(message)
        self.status = status


class BrkClientBase:
    """Base HTTP client for making requests."""

    def __init__(self, base_url: str, timeout: float = 30.0):
        parsed = urlparse(base_url)
        self._host = parsed.netloc
        self._secure = parsed.scheme == 'https'
        self._timeout = timeout
        self._conn: Optional[Union[HTTPSConnection, HTTPConnection]] = None

    def _connect(self) -> Union[HTTPSConnection, HTTPConnection]:
        """Get or create HTTP connection."""
        if self._conn is None:
            if self._secure:
                self._conn = HTTPSConnection(self._host, timeout=self._timeout)
            else:
                self._conn = HTTPConnection(self._host, timeout=self._timeout)
        return self._conn

    def get(self, path: str) -> bytes:
        """Make a GET request and return raw bytes."""
        try:
            conn = self._connect()
            conn.request("GET", path)
            res = conn.getresponse()
            data = res.read()
            if res.status >= 400:
                raise BrkError(f"HTTP error: {{res.status}}", res.status)
            return data
        except (ConnectionError, OSError, TimeoutError) as e:
            self._conn = None
            raise BrkError(str(e))

    def get_json(self, path: str) -> Any:
        """Make a GET request and return JSON."""
        return json.loads(self.get(path))

    def get_text(self, path: str) -> str:
        """Make a GET request and return text."""
        return self.get(path).decode()

    def close(self) -> None:
        """Close the HTTP client."""
        if self._conn:
            self._conn.close()
            self._conn = None

    def __enter__(self) -> BrkClientBase:
        return self

    def __exit__(self, exc_type: Optional[type], exc_val: Optional[BaseException], exc_tb: Optional[Any]) -> None:
        self.close()


def _m(acc: str, s: str) -> str:
    """Build metric name with suffix."""
    if not s: return acc
    return f"{{acc}}_{{s}}" if acc else s


def _p(prefix: str, acc: str) -> str:
    """Build metric name with prefix."""
    return f"{{prefix}}_{{acc}}" if acc else prefix

"#
    )
    .unwrap();
}

/// Generate the MetricData and MetricEndpointBuilder classes
pub fn generate_endpoint_class(output: &mut String) {
    writeln!(
        output,
        r#"# Date conversion constants
_GENESIS = date(2009, 1, 3)  # day1 0, week1 0
_DAY_ONE = date(2009, 1, 9)  # day1 1 (6 day gap after genesis)
_EPOCH = datetime(2009, 1, 1, tzinfo=timezone.utc)
_DATE_INDEXES = frozenset([
    'minute1', 'minute5', 'minute10', 'minute30',
    'hour1', 'hour4', 'hour12',
    'day1', 'day3', 'week1',
    'month1', 'month3', 'month6',
    'year1', 'year10',
])

def _index_to_date(index: str, i: int) -> Union[date, datetime]:
    """Convert an index value to a date/datetime for date-based indexes."""
    if index == 'minute1':
        return _EPOCH + timedelta(minutes=i)
    elif index == 'minute5':
        return _EPOCH + timedelta(minutes=i * 5)
    elif index == 'minute10':
        return _EPOCH + timedelta(minutes=i * 10)
    elif index == 'minute30':
        return _EPOCH + timedelta(minutes=i * 30)
    elif index == 'hour1':
        return _EPOCH + timedelta(hours=i)
    elif index == 'hour4':
        return _EPOCH + timedelta(hours=i * 4)
    elif index == 'hour12':
        return _EPOCH + timedelta(hours=i * 12)
    elif index == 'day1':
        return _GENESIS if i == 0 else _DAY_ONE + timedelta(days=i - 1)
    elif index == 'day3':
        return _EPOCH.date() - timedelta(days=1) + timedelta(days=i * 3)
    elif index == 'week1':
        return _GENESIS + timedelta(weeks=i)
    elif index == 'month1':
        return date(2009 + i // 12, i % 12 + 1, 1)
    elif index == 'month3':
        m = i * 3
        return date(2009 + m // 12, m % 12 + 1, 1)
    elif index == 'month6':
        m = i * 6
        return date(2009 + m // 12, m % 12 + 1, 1)
    elif index == 'year1':
        return date(2009 + i, 1, 1)
    elif index == 'year10':
        return date(2009 + i * 10, 1, 1)
    else:
        raise ValueError(f"{{index}} is not a date-based index")


def _date_to_index(index: str, d: Union[date, datetime]) -> int:
    """Convert a date/datetime to an index value for date-based indexes.

    Returns the floor index (latest index whose date is <= the given date).
    For sub-day indexes (minute*, hour*), a plain date is treated as midnight UTC.
    """
    if index in ('minute1', 'minute5', 'minute10', 'minute30', 'hour1', 'hour4', 'hour12'):
        if isinstance(d, datetime):
            dt = d if d.tzinfo else d.replace(tzinfo=timezone.utc)
        else:
            dt = datetime(d.year, d.month, d.day, tzinfo=timezone.utc)
        secs = int((dt - _EPOCH).total_seconds())
        div = {{'minute1': 60, 'minute5': 300, 'minute10': 600, 'minute30': 1800,
               'hour1': 3600, 'hour4': 14400, 'hour12': 43200}}
        return secs // div[index]
    dd = d.date() if isinstance(d, datetime) else d
    if index == 'day1':
        if dd < _DAY_ONE:
            return 0
        return 1 + (dd - _DAY_ONE).days
    elif index == 'day3':
        return (dd - date(2008, 12, 31)).days // 3
    elif index == 'week1':
        return (dd - _GENESIS).days // 7
    elif index == 'month1':
        return (dd.year - 2009) * 12 + (dd.month - 1)
    elif index == 'month3':
        return (dd.year - 2009) * 4 + (dd.month - 1) // 3
    elif index == 'month6':
        return (dd.year - 2009) * 2 + (dd.month - 1) // 6
    elif index == 'year1':
        return dd.year - 2009
    elif index == 'year10':
        return (dd.year - 2009) // 10
    else:
        raise ValueError(f"{{index}} is not a date-based index")


@dataclass
class MetricData(Generic[T]):
    """Metric data with range information. Always int-indexed."""
    version: int
    index: Index
    total: int
    start: int
    end: int
    stamp: str
    data: List[T]

    @property
    def is_date_based(self) -> bool:
        """Whether this metric uses a date-based index."""
        return self.index in _DATE_INDEXES

    def indexes(self) -> List[int]:
        """Get raw index numbers."""
        return list(range(self.start, self.end))

    def keys(self) -> List[int]:
        """Get keys as index numbers."""
        return self.indexes()

    def items(self) -> List[Tuple[int, T]]:
        """Get (index, value) pairs."""
        return list(zip(self.indexes(), self.data))

    def to_dict(self) -> Dict[int, T]:
        """Return {{index: value}} dict."""
        return dict(zip(self.indexes(), self.data))

    def __iter__(self) -> Iterator[Tuple[int, T]]:
        """Iterate over (index, value) pairs."""
        return iter(zip(self.indexes(), self.data))

    def __len__(self) -> int:
        return len(self.data)

    def to_polars(self) -> pl.DataFrame:
        """Convert to Polars DataFrame with 'index' and 'value' columns."""
        try:
            import polars as pl  # type: ignore[import-not-found]
        except ImportError:
            raise ImportError("polars is required: pip install polars")
        return pl.DataFrame({{"index": self.indexes(), "value": self.data}})

    def to_pandas(self) -> pd.DataFrame:
        """Convert to Pandas DataFrame with 'index' and 'value' columns."""
        try:
            import pandas as pd  # type: ignore[import-not-found]
        except ImportError:
            raise ImportError("pandas is required: pip install pandas")
        return pd.DataFrame({{"index": self.indexes(), "value": self.data}})


@dataclass
class DateMetricData(MetricData[T]):
    """Metric data with date-based index. Extends MetricData with date methods."""

    def dates(self) -> List[Union[date, datetime]]:
        """Get dates for the index range. Returns datetime for sub-daily indexes, date for daily+."""
        return [_index_to_date(self.index, i) for i in range(self.start, self.end)]

    def date_items(self) -> List[Tuple[Union[date, datetime], T]]:
        """Get (date, value) pairs."""
        return list(zip(self.dates(), self.data))

    def to_date_dict(self) -> Dict[Union[date, datetime], T]:
        """Return {{date: value}} dict."""
        return dict(zip(self.dates(), self.data))

    def to_polars(self, with_dates: bool = True) -> pl.DataFrame:
        """Convert to Polars DataFrame.

        Returns a DataFrame with columns:
        - 'date' and 'value' if with_dates=True (default)
        - 'index' and 'value' otherwise
        """
        try:
            import polars as pl  # type: ignore[import-not-found]
        except ImportError:
            raise ImportError("polars is required: pip install polars")
        if with_dates:
            return pl.DataFrame({{"date": self.dates(), "value": self.data}})
        return pl.DataFrame({{"index": self.indexes(), "value": self.data}})

    def to_pandas(self, with_dates: bool = True) -> pd.DataFrame:
        """Convert to Pandas DataFrame.

        Returns a DataFrame with columns:
        - 'date' and 'value' if with_dates=True (default)
        - 'index' and 'value' otherwise
        """
        try:
            import pandas as pd  # type: ignore[import-not-found]
        except ImportError:
            raise ImportError("pandas is required: pip install pandas")
        if with_dates:
            return pd.DataFrame({{"date": self.dates(), "value": self.data}})
        return pd.DataFrame({{"index": self.indexes(), "value": self.data}})


# Type aliases for non-generic usage
AnyMetricData = MetricData[Any]
AnyDateMetricData = DateMetricData[Any]


class _EndpointConfig:
    """Shared endpoint configuration."""
    client: BrkClientBase
    name: str
    index: Index
    start: Optional[int]
    end: Optional[int]

    def __init__(self, client: BrkClientBase, name: str, index: Index,
                 start: Optional[int] = None, end: Optional[int] = None):
        self.client = client
        self.name = name
        self.index = index
        self.start = start
        self.end = end

    def path(self) -> str:
        return f"/api/metric/{{self.name}}/{{self.index}}"

    def _build_path(self, format: Optional[str] = None) -> str:
        params = []
        if self.start is not None:
            params.append(f"start={{self.start}}")
        if self.end is not None:
            params.append(f"end={{self.end}}")
        if format is not None:
            params.append(f"format={{format}}")
        query = "&".join(params)
        p = self.path()
        return f"{{p}}?{{query}}" if query else p

    def _new(self, start: Optional[int] = None, end: Optional[int] = None) -> _EndpointConfig:
        return _EndpointConfig(self.client, self.name, self.index, start, end)

    def get_metric(self) -> MetricData[Any]:
        return MetricData(**self.client.get_json(self._build_path()))

    def get_date_metric(self) -> DateMetricData[Any]:
        return DateMetricData(**self.client.get_json(self._build_path()))

    def get_csv(self) -> str:
        return self.client.get_text(self._build_path(format='csv'))


class RangeBuilder(Generic[T]):
    """Builder with range specified."""

    def __init__(self, config: _EndpointConfig):
        self._config = config

    def fetch(self) -> MetricData[T]:
        """Fetch the range as parsed JSON."""
        return self._config.get_metric()

    def fetch_csv(self) -> str:
        """Fetch the range as CSV string."""
        return self._config.get_csv()


class SingleItemBuilder(Generic[T]):
    """Builder for single item access."""

    def __init__(self, config: _EndpointConfig):
        self._config = config

    def fetch(self) -> MetricData[T]:
        """Fetch the single item."""
        return self._config.get_metric()

    def fetch_csv(self) -> str:
        """Fetch as CSV."""
        return self._config.get_csv()


class SkippedBuilder(Generic[T]):
    """Builder after calling skip(n). Chain with take() to specify count."""

    def __init__(self, config: _EndpointConfig):
        self._config = config

    def take(self, n: int) -> RangeBuilder[T]:
        """Take n items after the skipped position."""
        start = self._config.start or 0
        return RangeBuilder(self._config._new(start, start + n))

    def fetch(self) -> MetricData[T]:
        """Fetch from skipped position to end."""
        return self._config.get_metric()

    def fetch_csv(self) -> str:
        """Fetch as CSV."""
        return self._config.get_csv()


class DateRangeBuilder(RangeBuilder[T]):
    """Range builder that returns DateMetricData."""
    def fetch(self) -> DateMetricData[T]:
        return self._config.get_date_metric()


class DateSingleItemBuilder(SingleItemBuilder[T]):
    """Single item builder that returns DateMetricData."""
    def fetch(self) -> DateMetricData[T]:
        return self._config.get_date_metric()


class DateSkippedBuilder(SkippedBuilder[T]):
    """Skipped builder that returns DateMetricData."""
    def take(self, n: int) -> DateRangeBuilder[T]:
        start = self._config.start or 0
        return DateRangeBuilder(self._config._new(start, start + n))
    def fetch(self) -> DateMetricData[T]:
        return self._config.get_date_metric()


class MetricEndpointBuilder(Generic[T]):
    """Builder for metric endpoint queries with int-based indexing.

    Examples:
        data = endpoint.fetch()
        data = endpoint[5].fetch()
        data = endpoint[:10].fetch()
        data = endpoint.head(20).fetch()
        data = endpoint.skip(100).take(10).fetch()
    """

    def __init__(self, client: BrkClientBase, name: str, index: Index):
        self._config = _EndpointConfig(client, name, index)

    @overload
    def __getitem__(self, key: int) -> SingleItemBuilder[T]: ...
    @overload
    def __getitem__(self, key: slice) -> RangeBuilder[T]: ...

    def __getitem__(self, key: Union[int, slice]) -> Union[SingleItemBuilder[T], RangeBuilder[T]]:
        """Access single item or slice by integer index."""
        if isinstance(key, int):
            return SingleItemBuilder(self._config._new(key, key + 1))
        return RangeBuilder(self._config._new(key.start, key.stop))

    def head(self, n: int = 10) -> RangeBuilder[T]:
        """Get the first n items."""
        return RangeBuilder(self._config._new(end=n))

    def tail(self, n: int = 10) -> RangeBuilder[T]:
        """Get the last n items."""
        return RangeBuilder(self._config._new(end=0) if n == 0 else self._config._new(start=-n))

    def skip(self, n: int) -> SkippedBuilder[T]:
        """Skip the first n items."""
        return SkippedBuilder(self._config._new(start=n))

    def fetch(self) -> MetricData[T]:
        """Fetch all data."""
        return self._config.get_metric()

    def fetch_csv(self) -> str:
        """Fetch all data as CSV."""
        return self._config.get_csv()

    def path(self) -> str:
        """Get the base endpoint path."""
        return self._config.path()


class DateMetricEndpointBuilder(Generic[T]):
    """Builder for metric endpoint queries with date-based indexing.

    Accepts dates in __getitem__ and returns DateMetricData from fetch().

    Examples:
        data = endpoint.fetch()
        data = endpoint[date(2020, 1, 1)].fetch()
        data = endpoint[date(2020, 1, 1):date(2023, 1, 1)].fetch()
        data = endpoint[:10].fetch()
    """

    def __init__(self, client: BrkClientBase, name: str, index: Index):
        self._config = _EndpointConfig(client, name, index)

    @overload
    def __getitem__(self, key: int) -> DateSingleItemBuilder[T]: ...
    @overload
    def __getitem__(self, key: datetime) -> DateSingleItemBuilder[T]: ...
    @overload
    def __getitem__(self, key: date) -> DateSingleItemBuilder[T]: ...
    @overload
    def __getitem__(self, key: slice) -> DateRangeBuilder[T]: ...

    def __getitem__(self, key: Union[int, slice, date, datetime]) -> Union[DateSingleItemBuilder[T], DateRangeBuilder[T]]:
        """Access single item or slice. Accepts int, date, or datetime."""
        if isinstance(key, (date, datetime)):
            idx = _date_to_index(self._config.index, key)
            return DateSingleItemBuilder(self._config._new(idx, idx + 1))
        if isinstance(key, int):
            return DateSingleItemBuilder(self._config._new(key, key + 1))
        start, stop = key.start, key.stop
        if isinstance(start, (date, datetime)):
            start = _date_to_index(self._config.index, start)
        if isinstance(stop, (date, datetime)):
            stop = _date_to_index(self._config.index, stop)
        return DateRangeBuilder(self._config._new(start, stop))

    def head(self, n: int = 10) -> DateRangeBuilder[T]:
        """Get the first n items."""
        return DateRangeBuilder(self._config._new(end=n))

    def tail(self, n: int = 10) -> DateRangeBuilder[T]:
        """Get the last n items."""
        return DateRangeBuilder(self._config._new(end=0) if n == 0 else self._config._new(start=-n))

    def skip(self, n: int) -> DateSkippedBuilder[T]:
        """Skip the first n items."""
        return DateSkippedBuilder(self._config._new(start=n))

    def fetch(self) -> DateMetricData[T]:
        """Fetch all data."""
        return self._config.get_date_metric()

    def fetch_csv(self) -> str:
        """Fetch all data as CSV."""
        return self._config.get_csv()

    def path(self) -> str:
        """Get the base endpoint path."""
        return self._config.path()


# Type aliases for non-generic usage
AnyMetricEndpointBuilder = MetricEndpointBuilder[Any]
AnyDateMetricEndpointBuilder = DateMetricEndpointBuilder[Any]


class MetricPattern(Protocol[T]):
    """Protocol for metric patterns with different index sets."""

    @property
    def name(self) -> str:
        """Get the metric name."""
        ...

    def indexes(self) -> List[str]:
        """Get the list of available indexes for this metric."""
        ...

    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]:
        """Get an endpoint builder for a specific index, if supported."""
        ...

"#
    )
    .unwrap();
}

/// Generate index accessor classes
pub fn generate_index_accessors(output: &mut String, patterns: &[IndexSetPattern]) {
    if patterns.is_empty() {
        return;
    }

    // Generate static index tuples
    writeln!(output, "# Static index tuples").unwrap();
    for (i, pattern) in patterns.iter().enumerate() {
        write!(output, "_i{} = (", i + 1).unwrap();
        for (j, index) in pattern.indexes.iter().enumerate() {
            if j > 0 {
                write!(output, ", ").unwrap();
            }
            write!(output, "'{}'", index.name()).unwrap();
        }
        // Single-element tuple needs trailing comma
        if pattern.indexes.len() == 1 {
            write!(output, ",").unwrap();
        }
        writeln!(output, ")").unwrap();
    }
    writeln!(output).unwrap();

    // Generate helper functions
    writeln!(
        output,
        r#"def _ep(c: BrkClientBase, n: str, i: Index) -> MetricEndpointBuilder[Any]:
    return MetricEndpointBuilder(c, n, i)

def _dep(c: BrkClientBase, n: str, i: Index) -> DateMetricEndpointBuilder[Any]:
    return DateMetricEndpointBuilder(c, n, i)
"#
    )
    .unwrap();

    writeln!(output, "# Index accessor classes\n").unwrap();

    for (i, pattern) in patterns.iter().enumerate() {
        let by_class_name = format!("_{}By", pattern.name);
        let idx_var = format!("_i{}", i + 1);

        // Generate the By class with compact methods
        writeln!(output, "class {}(Generic[T]):", by_class_name).unwrap();
        writeln!(
            output,
            "    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n"
        )
        .unwrap();
        for index in &pattern.indexes {
            let method_name = index_to_field_name(index);
            let index_name = index.name();
            let (builder_type, helper) = if index.is_date_based() {
                ("DateMetricEndpointBuilder", "_dep")
            } else {
                ("MetricEndpointBuilder", "_ep")
            };
            writeln!(
                output,
                "    def {}(self) -> {}[T]: return {}(self._c, self._n, '{}')",
                method_name, builder_type, helper, index_name
            )
            .unwrap();
        }
        writeln!(output).unwrap();

        // Generate the main accessor class
        writeln!(output, "class {}(Generic[T]):", pattern.name).unwrap();
        writeln!(output, "    by: {}[T]", by_class_name).unwrap();
        writeln!(
            output,
            "    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, {}(c, n)",
            by_class_name
        )
        .unwrap();
        writeln!(output, "    @property").unwrap();
        writeln!(output, "    def name(self) -> str: return self._n").unwrap();
        writeln!(output, "    def indexes(self) -> List[str]: return list({})", idx_var).unwrap();
        writeln!(
            output,
            "    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in {} else None",
            idx_var
        )
        .unwrap();
        writeln!(output).unwrap();
    }
}

/// Generate structural pattern classes
pub fn generate_structural_patterns(
    output: &mut String,
    patterns: &[StructuralPattern],
    metadata: &ClientMetadata,
) {
    if patterns.is_empty() {
        return;
    }

    writeln!(output, "# Reusable structural pattern classes\n").unwrap();

    for pattern in patterns {
        // Generate class
        if pattern.is_generic {
            writeln!(output, "class {}(Generic[T]):", pattern.name).unwrap();
        } else {
            writeln!(output, "class {}:", pattern.name).unwrap();
        }
        writeln!(
            output,
            "    \"\"\"Pattern struct for repeated tree structure.\"\"\""
        )
        .unwrap();
        writeln!(output, "    ").unwrap();
        writeln!(
            output,
            "    def __init__(self, client: BrkClientBase, acc: str):"
        )
        .unwrap();
        writeln!(
            output,
            "        \"\"\"Create pattern node with accumulated metric name.\"\"\""
        )
        .unwrap();

        let syntax = PythonSyntax;
        for field in &pattern.fields {
            generate_parameterized_field(output, &syntax, field, pattern, metadata, "        ");
        }

        writeln!(output).unwrap();
    }
}
