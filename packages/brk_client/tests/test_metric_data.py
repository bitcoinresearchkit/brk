# Tests for MetricData helper methods including polars/pandas conversion
# Run: uv run pytest tests/test_metric_data.py -v

from datetime import date, datetime, timezone

import pytest

from brk_client import MetricData


# Test data fixtures
@pytest.fixture
def date_based_metric():
    """MetricData with day1 (date-based)."""
    return MetricData(
        version=1,
        index="day1",
        total=100,
        start=0,
        end=5,
        stamp="2024-01-01T00:00:00Z",
        data=[100, 200, 300, 400, 500],
    )


@pytest.fixture
def height_based_metric():
    """MetricData with height (non-date-based)."""
    return MetricData(
        version=1,
        index="height",
        total=1000,
        start=800000,
        end=800005,
        stamp="2024-01-01T00:00:00Z",
        data=[1.5, 2.5, 3.5, 4.5, 5.5],
    )


@pytest.fixture
def month_based_metric():
    """MetricData with month1."""
    return MetricData(
        version=1,
        index="month1",
        total=200,
        start=0,
        end=3,
        stamp="2024-01-01T00:00:00Z",
        data=[1000, 2000, 3000],
    )


# ============ is_date_based tests ============


class TestIsDateBased:
    """Test the is_date_based property."""

    def test_day1_is_date_based(self, date_based_metric):
        assert date_based_metric.is_date_based is True

    def test_height_is_not_date_based(self, height_based_metric):
        assert height_based_metric.is_date_based is False

    def test_month1_is_date_based(self, month_based_metric):
        assert month_based_metric.is_date_based is True


# ============ Date conversion tests ============


class TestIndexToDate:
    """Test the _index_to_date function via MetricData.dates()."""

    def test_day1_zero(self, date_based_metric):
        """day1 0 is genesis: Jan 3, 2009."""
        dates = date_based_metric.dates()
        assert dates[0] == date(2009, 1, 3)

    def test_day1_one(self, date_based_metric):
        """day1 1 is Jan 9, 2009 (6 day gap after genesis)."""
        dates = date_based_metric.dates()
        assert dates[1] == date(2009, 1, 9)

    def test_day1_two(self, date_based_metric):
        """day1 2 is Jan 10, 2009."""
        dates = date_based_metric.dates()
        assert dates[2] == date(2009, 1, 10)

    def test_month1_dates(self, month_based_metric):
        """month1 returns correct dates."""
        dates = month_based_metric.dates()
        assert dates[0] == date(2009, 1, 1)
        assert dates[1] == date(2009, 2, 1)
        assert dates[2] == date(2009, 3, 1)


# ============ Smart keys/items/to_dict/iter tests ============


class TestSmartHelpers:
    """Test smart MetricData helpers that auto-detect date vs numeric keys."""

    def test_keys_date_based(self, date_based_metric):
        """keys() returns dates for date-based metric."""
        keys = date_based_metric.keys()
        assert len(keys) == 5
        assert keys[0] == date(2009, 1, 3)  # genesis
        assert keys[1] == date(2009, 1, 9)  # day1 1

    def test_keys_height_based(self, height_based_metric):
        """keys() returns index numbers for non-date-based metric."""
        keys = height_based_metric.keys()
        assert keys == [800000, 800001, 800002, 800003, 800004]

    def test_items_date_based(self, date_based_metric):
        """items() returns (date, value) pairs for date-based metric."""
        items = date_based_metric.items()
        assert items[0] == (date(2009, 1, 3), 100)
        assert items[1] == (date(2009, 1, 9), 200)

    def test_items_height_based(self, height_based_metric):
        """items() returns (index, value) pairs for height-based metric."""
        items = height_based_metric.items()
        assert items[0] == (800000, 1.5)
        assert items[4] == (800004, 5.5)

    def test_to_dict_date_based(self, date_based_metric):
        """to_dict() returns {date: value} for date-based metric."""
        d = date_based_metric.to_dict()
        assert d[date(2009, 1, 3)] == 100
        assert d[date(2009, 1, 9)] == 200

    def test_to_dict_height_based(self, height_based_metric):
        """to_dict() returns {index: value} for height-based metric."""
        d = height_based_metric.to_dict()
        assert d[800000] == 1.5
        assert d[800004] == 5.5

    def test_iter_date_based(self, date_based_metric):
        """Default iteration yields (date, value) for date-based metric."""
        result = list(date_based_metric)
        assert result[0] == (date(2009, 1, 3), 100)
        assert result[1] == (date(2009, 1, 9), 200)
        assert len(result) == 5

    def test_iter_height_based(self, height_based_metric):
        """Default iteration yields (index, value) for height-based metric."""
        result = list(height_based_metric)
        assert result == [
            (800000, 1.5),
            (800001, 2.5),
            (800002, 3.5),
            (800003, 4.5),
            (800004, 5.5),
        ]


# ============ Explicit indexes/dates tests ============


class TestExplicitAccessors:
    """Test explicit indexes() and dates() methods."""

    def test_indexes_returns_range(self, date_based_metric):
        """indexes() returns list of index values."""
        assert date_based_metric.indexes() == [0, 1, 2, 3, 4]

    def test_indexes_with_offset(self, height_based_metric):
        """indexes() respects start/end offsets."""
        assert height_based_metric.indexes() == [800000, 800001, 800002, 800003, 800004]

    def test_dates_raises_for_non_date(self, height_based_metric):
        """dates() raises for non-date-based index."""
        with pytest.raises(ValueError):
            height_based_metric.dates()


# ============ Polars tests ============


class TestPolarsConversion:
    """Test MetricData.to_polars() conversion."""

    @pytest.fixture(autouse=True)
    def check_polars(self):
        """Skip if polars not installed."""
        pytest.importorskip("polars")

    def test_to_polars_with_dates(self, date_based_metric):
        """to_polars() includes date column for date-based index."""
        import polars as pl

        df = date_based_metric.to_polars()
        assert isinstance(df, pl.DataFrame)
        assert "date" in df.columns
        assert "value" in df.columns
        assert len(df) == 5
        assert df["value"].to_list() == [100, 200, 300, 400, 500]

    def test_to_polars_without_dates(self, date_based_metric):
        """to_polars(with_dates=False) uses index column."""
        import polars as pl

        df = date_based_metric.to_polars(with_dates=False)
        assert "index" in df.columns
        assert "date" not in df.columns
        assert df["index"].to_list() == [0, 1, 2, 3, 4]

    def test_to_polars_non_date_index(self, height_based_metric):
        """to_polars() uses index column for non-date-based index."""
        import polars as pl

        df = height_based_metric.to_polars()
        assert "index" in df.columns
        assert "date" not in df.columns
        assert df["index"].to_list() == [800000, 800001, 800002, 800003, 800004]
        assert df["value"].to_list() == [1.5, 2.5, 3.5, 4.5, 5.5]

    def test_to_polars_month1(self, month_based_metric):
        """to_polars() works with month1."""
        import polars as pl

        df = month_based_metric.to_polars()
        assert "date" in df.columns
        assert len(df) == 3
        dates = df["date"].to_list()
        assert dates[0] == date(2009, 1, 1)
        assert dates[1] == date(2009, 2, 1)
        assert dates[2] == date(2009, 3, 1)


# ============ Pandas tests ============


class TestPandasConversion:
    """Test MetricData.to_pandas() conversion."""

    @pytest.fixture(autouse=True)
    def check_pandas(self):
        """Skip if pandas not installed."""
        pytest.importorskip("pandas")

    def test_to_pandas_with_dates(self, date_based_metric):
        """to_pandas() includes date column for date-based index."""
        import pandas as pd

        df = date_based_metric.to_pandas()
        assert isinstance(df, pd.DataFrame)
        assert "date" in df.columns
        assert "value" in df.columns
        assert len(df) == 5
        assert df["value"].tolist() == [100, 200, 300, 400, 500]

    def test_to_pandas_without_dates(self, date_based_metric):
        """to_pandas(with_dates=False) uses index column."""
        import pandas as pd

        df = date_based_metric.to_pandas(with_dates=False)
        assert "index" in df.columns
        assert "date" not in df.columns
        assert df["index"].tolist() == [0, 1, 2, 3, 4]

    def test_to_pandas_non_date_index(self, height_based_metric):
        """to_pandas() uses index column for non-date-based index."""
        import pandas as pd

        df = height_based_metric.to_pandas()
        assert "index" in df.columns
        assert "date" not in df.columns
        assert df["index"].tolist() == [800000, 800001, 800002, 800003, 800004]
        assert df["value"].tolist() == [1.5, 2.5, 3.5, 4.5, 5.5]

    def test_to_pandas_month1(self, month_based_metric):
        """to_pandas() works with month1."""
        import pandas as pd

        df = month_based_metric.to_pandas()
        assert "date" in df.columns
        assert len(df) == 3
        dates = df["date"].tolist()
        assert dates[0] == date(2009, 1, 1)
        assert dates[1] == date(2009, 2, 1)
        assert dates[2] == date(2009, 3, 1)
