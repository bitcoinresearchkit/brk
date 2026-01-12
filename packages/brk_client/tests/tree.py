"""Comprehensive test that fetches all endpoints in the tree."""

from brk_client import BrkClient


def get_all_metrics(obj, path=""):
    """Recursively collect all MetricPattern instances from the tree."""
    metrics = []

    for attr_name in dir(obj):
        if attr_name.startswith("_"):
            continue

        try:
            attr = getattr(obj, attr_name)
        except Exception:
            continue

        current_path = f"{path}.{attr_name}" if path else attr_name

        # Check if this is a metric pattern (has 'by' attribute with index methods)
        if hasattr(attr, "by"):
            by = attr.by
            indexes = []
            for idx_name in dir(by):
                if not idx_name.startswith("_") and callable(
                    getattr(by, idx_name, None)
                ):
                    indexes.append(idx_name)
            if indexes:
                metrics.append((current_path, attr, indexes))

        # Recurse into nested tree nodes
        if hasattr(attr, "__dict__") and not callable(attr):
            metrics.extend(get_all_metrics(attr, current_path))

    return metrics


def test_all_endpoints():
    """Test fetching last 3 values from all metric endpoints."""
    client = BrkClient("http://localhost:3110")

    metrics = get_all_metrics(client.metrics)
    print(f"\nFound {len(metrics)} metrics")

    success = 0
    failed = 0
    errors = []

    for path, metric, indexes in metrics:
        for idx_name in indexes:
            try:
                by = metric.by
                endpoint = getattr(by, idx_name)()
                # Use the new idiomatic API: tail(3).fetch() or [-3:].fetch()
                res = endpoint.tail(3).fetch()
                count = len(res["data"])
                if count != 3:
                    failed += 1
                    error_msg = (
                        f"FAIL: {path}.by.{idx_name}() -> expected 3, got {count}"
                    )
                    errors.append(error_msg)
                    print(error_msg)
                else:
                    success += 1
                    print(f"OK: {path}.by.{idx_name}() -> {count} items")
            except Exception as e:
                failed += 1
                error_msg = f"FAIL: {path}.by.{idx_name}() -> {e}"
                errors.append(error_msg)
                print(error_msg)

    print("\n=== Results ===")
    print(f"Success: {success}")
    print(f"Failed: {failed}")

    if errors:
        print("\nErrors:")
        for err in errors[:10]:  # Show first 10 errors
            print(f"  {err}")
        if len(errors) > 10:
            print(f"  ... and {len(errors) - 10} more")

    assert failed == 0, f"{failed} endpoints failed"


if __name__ == "__main__":
    test_all_endpoints()
