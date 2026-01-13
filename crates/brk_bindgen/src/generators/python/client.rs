//! Python base client and pattern factory generation.

use std::fmt::Write;

use brk_cohort::{
    AGE_RANGE_NAMES, AMOUNT_RANGE_NAMES, EPOCH_NAMES, GE_AMOUNT_NAMES, LT_AMOUNT_NAMES,
    MAX_AGE_NAMES, MIN_AGE_NAMES, SPENDABLE_TYPE_NAMES, TERM_NAMES, YEAR_NAMES,
};
use brk_types::{pools, Index};
use serde::Serialize;

use crate::{
    ClientMetadata, IndexSetPattern, PythonSyntax, StructuralPattern, VERSION,
    generate_parameterized_field, index_to_field_name,
};

/// Generate class-level constants for the BrkClient class.
pub fn generate_class_constants(output: &mut String) {
    fn class_const<T: Serialize>(output: &mut String, name: &str, value: &T) {
        let json = serde_json::to_string_pretty(value).unwrap();
        // Indent all lines for class body
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

    // VERSION
    writeln!(output, "    VERSION = \"v{}\"\n", VERSION).unwrap();

    // INDEXES
    let indexes = Index::all();
    let indexes_list: Vec<&str> = indexes.iter().map(|i| i.serialize_long()).collect();
    class_const(output, "INDEXES", &indexes_list);

    // POOL_ID_TO_POOL_NAME
    let pools = pools();
    let mut sorted_pools: Vec<_> = pools.iter().collect();
    sorted_pools.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    let pool_map: std::collections::BTreeMap<String, &str> = sorted_pools
        .iter()
        .map(|p| (p.slug().to_string(), p.name))
        .collect();
    class_const(output, "POOL_ID_TO_POOL_NAME", &pool_map);

    // Cohort names
    class_const(output, "TERM_NAMES", &TERM_NAMES);
    class_const(output, "EPOCH_NAMES", &EPOCH_NAMES);
    class_const(output, "YEAR_NAMES", &YEAR_NAMES);
    class_const(output, "SPENDABLE_TYPE_NAMES", &SPENDABLE_TYPE_NAMES);
    class_const(output, "AGE_RANGE_NAMES", &AGE_RANGE_NAMES);
    class_const(output, "MAX_AGE_NAMES", &MAX_AGE_NAMES);
    class_const(output, "MIN_AGE_NAMES", &MIN_AGE_NAMES);
    class_const(output, "AMOUNT_RANGE_NAMES", &AMOUNT_RANGE_NAMES);
    class_const(output, "GE_AMOUNT_NAMES", &GE_AMOUNT_NAMES);
    class_const(output, "LT_AMOUNT_NAMES", &LT_AMOUNT_NAMES);
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

    def close(self):
        """Close the HTTP client."""
        if self._conn:
            self._conn.close()
            self._conn = None

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
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
        r#"class MetricData(TypedDict, Generic[T]):
    """Metric data with range information."""
    total: int
    start: int
    end: int
    data: List[T]


# Type alias for non-generic usage
AnyMetricData = MetricData[Any]


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

    def get_json(self) -> Any:
        return self.client.get_json(self._build_path())

    def get_csv(self) -> str:
        return self.client.get_text(self._build_path(format='csv'))


class RangeBuilder(Generic[T]):
    """Builder with range specified."""

    def __init__(self, config: _EndpointConfig):
        self._config = config

    def fetch(self) -> MetricData[T]:
        """Fetch the range as parsed JSON."""
        return self._config.get_json()

    def fetch_csv(self) -> str:
        """Fetch the range as CSV string."""
        return self._config.get_csv()


class SingleItemBuilder(Generic[T]):
    """Builder for single item access."""

    def __init__(self, config: _EndpointConfig):
        self._config = config

    def fetch(self) -> MetricData[T]:
        """Fetch the single item."""
        return self._config.get_json()

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
        return RangeBuilder(_EndpointConfig(
            self._config.client, self._config.name, self._config.index,
            start, start + n
        ))

    def fetch(self) -> MetricData[T]:
        """Fetch from skipped position to end."""
        return self._config.get_json()

    def fetch_csv(self) -> str:
        """Fetch as CSV."""
        return self._config.get_csv()


class MetricEndpointBuilder(Generic[T]):
    """Builder for metric endpoint queries.

    Use method chaining to specify the data range, then call fetch() or fetch_csv() to execute.

    Examples:
        # Fetch all data
        data = endpoint.fetch()

        # Single item access
        data = endpoint[5].fetch()

        # Slice syntax (Python-native)
        data = endpoint[:10].fetch()      # First 10
        data = endpoint[-5:].fetch()      # Last 5
        data = endpoint[100:110].fetch()  # Range

        # Convenience methods (pandas-style)
        data = endpoint.head().fetch()    # First 10 (default)
        data = endpoint.head(20).fetch()  # First 20
        data = endpoint.tail(5).fetch()   # Last 5

        # Iterator-style chaining
        data = endpoint.skip(100).take(10).fetch()
    """

    def __init__(self, client: BrkClientBase, name: str, index: Index):
        self._config = _EndpointConfig(client, name, index)

    @overload
    def __getitem__(self, key: int) -> SingleItemBuilder[T]: ...
    @overload
    def __getitem__(self, key: slice) -> RangeBuilder[T]: ...

    def __getitem__(self, key: Union[int, slice]) -> Union[SingleItemBuilder[T], RangeBuilder[T]]:
        """Access single item or slice.

        Examples:
            endpoint[5]        # Single item at index 5
            endpoint[:10]      # First 10
            endpoint[-5:]      # Last 5
            endpoint[100:110]  # Range 100-109
        """
        if isinstance(key, int):
            return SingleItemBuilder(_EndpointConfig(
                self._config.client, self._config.name, self._config.index,
                key, key + 1
            ))
        return RangeBuilder(_EndpointConfig(
            self._config.client, self._config.name, self._config.index,
            key.start, key.stop
        ))

    def head(self, n: int = 10) -> RangeBuilder[T]:
        """Get the first n items (pandas-style)."""
        return RangeBuilder(_EndpointConfig(
            self._config.client, self._config.name, self._config.index,
            None, n
        ))

    def tail(self, n: int = 10) -> RangeBuilder[T]:
        """Get the last n items (pandas-style)."""
        start, end = (None, 0) if n == 0 else (-n, None)
        return RangeBuilder(_EndpointConfig(
            self._config.client, self._config.name, self._config.index,
            start, end
        ))

    def skip(self, n: int) -> SkippedBuilder[T]:
        """Skip the first n items. Chain with take() to get a range."""
        return SkippedBuilder(_EndpointConfig(
            self._config.client, self._config.name, self._config.index,
            n, None
        ))

    def fetch(self) -> MetricData[T]:
        """Fetch all data as parsed JSON."""
        return self._config.get_json()

    def fetch_csv(self) -> str:
        """Fetch all data as CSV string."""
        return self._config.get_csv()

    def path(self) -> str:
        """Get the base endpoint path."""
        return self._config.path()


# Type alias for non-generic usage
AnyMetricEndpointBuilder = MetricEndpointBuilder[Any]


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

    writeln!(output, "# Index accessor classes\n").unwrap();

    for pattern in patterns {
        let by_class_name = format!("_{}By", pattern.name);

        // Generate the By class with lazy endpoint methods
        writeln!(output, "class {}(Generic[T]):", by_class_name).unwrap();
        writeln!(output, "    \"\"\"Index endpoint methods container.\"\"\"").unwrap();
        writeln!(output, "    ").unwrap();
        writeln!(
            output,
            "    def __init__(self, client: BrkClientBase, name: str):"
        )
        .unwrap();
        writeln!(output, "        self._client = client").unwrap();
        writeln!(output, "        self._name = name").unwrap();
        writeln!(output).unwrap();

        // Generate methods for each index
        for index in &pattern.indexes {
            let method_name = index_to_field_name(index);
            let index_name = index.serialize_long();
            writeln!(output, "    def {}(self) -> MetricEndpointBuilder[T]:", method_name).unwrap();
            writeln!(
                output,
                "        return MetricEndpointBuilder(self._client, self._name, '{}')",
                index_name
            )
            .unwrap();
            writeln!(output).unwrap();
        }

        // Generate the main accessor class
        writeln!(output, "class {}(Generic[T]):", pattern.name).unwrap();
        writeln!(
            output,
            "    \"\"\"Index accessor for metrics with {} indexes.\"\"\"",
            pattern.indexes.len()
        )
        .unwrap();
        writeln!(output, "    ").unwrap();
        writeln!(
            output,
            "    def __init__(self, client: BrkClientBase, name: str):"
        )
        .unwrap();
        writeln!(output, "        self._client = client").unwrap();
        writeln!(output, "        self._name = name").unwrap();
        writeln!(
            output,
            "        self.by: {}[T] = {}(client, name)",
            by_class_name, by_class_name
        )
        .unwrap();
        writeln!(output).unwrap();
        writeln!(output, "    @property").unwrap();
        writeln!(output, "    def name(self) -> str:").unwrap();
        writeln!(output, "        \"\"\"Get the metric name.\"\"\"").unwrap();
        writeln!(output, "        return self._name").unwrap();
        writeln!(output).unwrap();
        writeln!(output, "    def indexes(self) -> List[str]:").unwrap();
        writeln!(output, "        \"\"\"Get the list of available indexes.\"\"\"").unwrap();
        write!(output, "        return [").unwrap();
        for (i, index) in pattern.indexes.iter().enumerate() {
            if i > 0 {
                write!(output, ", ").unwrap();
            }
            write!(output, "'{}'", index.serialize_long()).unwrap();
        }
        writeln!(output, "]").unwrap();
        writeln!(output).unwrap();

        // Generate get(index) method
        writeln!(output, "    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]:").unwrap();
        writeln!(output, "        \"\"\"Get an endpoint builder for a specific index, if supported.\"\"\"").unwrap();
        for (i, index) in pattern.indexes.iter().enumerate() {
            let method_name = index_to_field_name(index);
            let index_name = index.serialize_long();
            if i == 0 {
                writeln!(output, "        if index == '{}': return self.by.{}()", index_name, method_name).unwrap();
            } else {
                writeln!(output, "        elif index == '{}': return self.by.{}()", index_name, method_name).unwrap();
            }
        }
        writeln!(output, "        return None").unwrap();
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
