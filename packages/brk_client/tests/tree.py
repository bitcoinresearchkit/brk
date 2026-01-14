# Run:
# uv run pytest tests/tree.py -s

"""Comprehensive test that fetches all endpoints in the tree."""

from brk_client import BrkClient


def is_metric_pattern(obj):
    """Check if an object is a metric pattern (has indexes() method and by attribute)."""
    return (
        hasattr(obj, "indexes")
        and callable(getattr(obj, "indexes", None))
        and hasattr(obj, "by")
    )


def get_all_metrics(obj, path=""):
    """Recursively collect all MetricPattern instances from the tree."""
    metrics = []

    for attr_name in dir(obj):
        # Skip dunder methods and internal attributes like _client
        if attr_name.startswith("__") or attr_name == "_client":
            continue

        try:
            attr = getattr(obj, attr_name)
        except Exception:
            continue

        if attr is None or callable(attr):
            continue

        current_path = f"{path}.{attr_name}" if path else attr_name

        # Check if this is a metric pattern using the indexes() method
        if is_metric_pattern(attr):
            metrics.append((current_path, attr))

        # Recurse into nested tree nodes
        if hasattr(attr, "__dict__"):
            metrics.extend(get_all_metrics(attr, current_path))

    return metrics


def test_all_endpoints():
    """Test fetching last value from all metric endpoints."""
    client = BrkClient("http://localhost:3110")

    metrics = get_all_metrics(client.metrics)
    print(f"\nFound {len(metrics)} metrics")

    success = 0

    for path, metric in metrics:
        # Use the indexes() method to get all available indexes
        indexes = metric.indexes()

        for idx_name in indexes:
            full_path = f"{path}.by.{idx_name}"

            try:
                # Verify both access methods work: .by.index() and .get(index)
                by = metric.by
                endpoint_by_property = getattr(by, idx_name)()
                endpoint_by_get = metric.get(idx_name)

                if endpoint_by_property is None:
                    raise Exception(f"metric.by.{idx_name}() returned None")
                if endpoint_by_get is None:
                    raise Exception(f"metric.get('{idx_name}') returned None")

                endpoint_by_property.tail(1).fetch()
                success += 1
                print(f"OK: {full_path}")
            except Exception as e:
                print(f"FAIL: {full_path} -> {e}")
                return

    print("\n=== Results ===")
    print(f"Success: {success}")


if __name__ == "__main__":
    test_all_endpoints()
