//! Python base client and pattern factory generation.

use std::fmt::Write;

use brk_cohort::{
    AGE_RANGE_NAMES, AMOUNT_RANGE_NAMES, EPOCH_NAMES, GE_AMOUNT_NAMES, LT_AMOUNT_NAMES,
    MAX_AGE_NAMES, MIN_AGE_NAMES, SPENDABLE_TYPE_NAMES, TERM_NAMES, YEAR_NAMES,
};
use brk_types::{pools, Index};
use serde::Serialize;

use crate::{
    ClientMetadata, GenericSyntax, IndexSetPattern, PatternField, PythonSyntax,
    StructuralPattern, VERSION, generate_parameterized_field, generate_tree_path_field,
    index_to_field_name,
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
        self.base_url = base_url
        self.timeout = timeout
        self._client = httpx.Client(timeout=timeout)

    def get(self, path: str) -> Any:
        """Make a GET request."""
        try:
            base = self.base_url.rstrip('/')
            response = self._client.get(f"{{base}}{{path}}")
            response.raise_for_status()
            return response.json()
        except httpx.HTTPStatusError as e:
            raise BrkError(f"HTTP error: {{e.response.status_code}}", e.response.status_code)
        except httpx.RequestError as e:
            raise BrkError(str(e))

    def close(self):
        """Close the HTTP client."""
        self._client.close()

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        self.close()


def _m(acc: str, s: str) -> str:
    """Build metric name with optional prefix."""
    return f"{{acc}}_{{s}}" if acc else s

"#
    )
    .unwrap();
}

/// Generate the MetricData and MetricEndpoint classes
pub fn generate_endpoint_class(output: &mut String) {
    writeln!(
        output,
        r#"class MetricData(TypedDict, Generic[T]):
    """Metric data with range information."""
    total: int
    from_: int  # 'from' is reserved in Python
    to: int
    data: List[T]


# Type alias for non-generic usage
AnyMetricData = MetricData[Any]


class MetricEndpoint(Generic[T]):
    """An endpoint for a specific metric + index combination."""

    def __init__(self, client: BrkClientBase, name: str, index: str):
        self._client = client
        self._name = name
        self._index = index

    def get(self) -> MetricData[T]:
        """Fetch all data points for this metric/index."""
        return self._client.get(self.path())

    def range(self, from_val: Optional[int] = None, to_val: Optional[int] = None) -> MetricData[T]:
        """Fetch data points within a range."""
        params = []
        if from_val is not None:
            params.append(f"from={{from_val}}")
        if to_val is not None:
            params.append(f"to={{to_val}}")
        query = "&".join(params)
        p = self.path()
        return self._client.get(f"{{p}}?{{query}}" if query else p)

    def path(self) -> str:
        """Get the endpoint path."""
        return f"/api/metric/{{self._name}}/{{self._index}}"


# Type alias for non-generic usage
AnyMetricEndpoint = MetricEndpoint[Any]


class MetricPattern(Protocol[T]):
    """Protocol for metric patterns with different index sets."""

    @property
    def name(self) -> str:
        """Get the metric name."""
        ...

    def indexes(self) -> List[str]:
        """Get the list of available indexes for this metric."""
        ...

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
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
            writeln!(output, "    def {}(self) -> MetricEndpoint[T]:", method_name).unwrap();
            writeln!(
                output,
                "        return MetricEndpoint(self._client, self._name, '{}')",
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
        writeln!(output, "    def get(self, index: str) -> Optional[MetricEndpoint[T]]:").unwrap();
        writeln!(output, "        \"\"\"Get an endpoint for a specific index, if supported.\"\"\"").unwrap();
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
        let is_parameterizable = pattern.is_parameterizable();

        // For generic patterns, inherit from Generic[T]
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

        if is_parameterizable {
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
        } else {
            writeln!(
                output,
                "    def __init__(self, client: BrkClientBase, base_path: str):"
            )
            .unwrap();
        }

        let syntax = PythonSyntax;
        for field in &pattern.fields {
            if is_parameterizable {
                generate_parameterized_field(output, &syntax, field, pattern, metadata, "        ");
            } else {
                generate_tree_path_field(output, &syntax, field, metadata, "        ");
            }
        }

        writeln!(output).unwrap();
    }
}

/// Get Python type annotation for a field with optional generic value type.
pub fn field_type_with_generic(
    field: &PatternField,
    metadata: &ClientMetadata,
    is_generic: bool,
    generic_value_type: Option<&str>,
) -> String {
    metadata.field_type_annotation(field, is_generic, generic_value_type, GenericSyntax::PYTHON)
}
