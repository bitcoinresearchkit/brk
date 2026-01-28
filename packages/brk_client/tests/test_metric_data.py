# Tests for MetricData helper methods including polars/pandas conversion
# Run: uv run pytest tests/test_metric_data.py -v

from datetime import date
import pytest

from brk_client import MetricData, index_to_date, is_date_index


# Test data fixtures
@pytest.fixture
def date_based_metric():
    """MetricData with dateindex (date-based)."""
    return MetricData(
        version=1,
        index="dateindex",
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
    """MetricData with monthindex."""
    return MetricData(
        version=1,
        index="monthindex",
        total=200,
        start=0,
        end=3,
        stamp="2024-01-01T00:00:00Z",
        data=[1000, 2000, 3000],
    )


# ============ Date conversion tests ============

class TestIndexToDate:
    """Test the index_to_date function."""

    def test_dateindex_zero(self):
        """DateIndex 0 is genesis: Jan 3, 2009."""
        assert index_to_date("dateindex", 0) == date(2009, 1, 3)

    def test_dateindex_one(self):
        """DateIndex 1 is Jan 9, 2009 (6 day gap after genesis)."""
        assert index_to_date("dateindex", 1) == date(2009, 1, 9)

    def test_dateindex_two(self):
        """DateIndex 2 is Jan 10, 2009."""
        assert index_to_date("dateindex", 2) == date(2009, 1, 10)

    def test_weekindex_zero(self):
        """WeekIndex 0 is genesis: Jan 3, 2009."""
        assert index_to_date("weekindex", 0) == date(2009, 1, 3)

    def test_weekindex_one(self):
        """WeekIndex 1 is Jan 10, 2009 (one week after genesis)."""
        assert index_to_date("weekindex", 1) == date(2009, 1, 10)

    def test_monthindex_zero(self):
        """MonthIndex 0 is Jan 1, 2009."""
        assert index_to_date("monthindex", 0) == date(2009, 1, 1)

    def test_monthindex_one(self):
        """MonthIndex 1 is Feb 1, 2009."""
        assert index_to_date("monthindex", 1) == date(2009, 2, 1)

    def test_monthindex_twelve(self):
        """MonthIndex 12 is Jan 1, 2010."""
        assert index_to_date("monthindex", 12) == date(2010, 1, 1)

    def test_yearindex_zero(self):
        """YearIndex 0 is Jan 1, 2009."""
        assert index_to_date("yearindex", 0) == date(2009, 1, 1)

    def test_yearindex_one(self):
        """YearIndex 1 is Jan 1, 2010."""
        assert index_to_date("yearindex", 1) == date(2010, 1, 1)

    def test_quarterindex_zero(self):
        """QuarterIndex 0 is Q1 2009: Jan 1, 2009."""
        assert index_to_date("quarterindex", 0) == date(2009, 1, 1)

    def test_quarterindex_one(self):
        """QuarterIndex 1 is Q2 2009: Apr 1, 2009."""
        assert index_to_date("quarterindex", 1) == date(2009, 4, 1)

    def test_quarterindex_four(self):
        """QuarterIndex 4 is Q1 2010: Jan 1, 2010."""
        assert index_to_date("quarterindex", 4) == date(2010, 1, 1)

    def test_semesterindex_zero(self):
        """SemesterIndex 0 is H1 2009: Jan 1, 2009."""
        assert index_to_date("semesterindex", 0) == date(2009, 1, 1)

    def test_semesterindex_one(self):
        """SemesterIndex 1 is H2 2009: Jul 1, 2009."""
        assert index_to_date("semesterindex", 1) == date(2009, 7, 1)

    def test_semesterindex_two(self):
        """SemesterIndex 2 is H1 2010: Jan 1, 2010."""
        assert index_to_date("semesterindex", 2) == date(2010, 1, 1)

    def test_decadeindex_zero(self):
        """DecadeIndex 0 is 2009: Jan 1, 2009."""
        assert index_to_date("decadeindex", 0) == date(2009, 1, 1)

    def test_decadeindex_one(self):
        """DecadeIndex 1 is 2019: Jan 1, 2019."""
        assert index_to_date("decadeindex", 1) == date(2019, 1, 1)

    def test_invalid_index_raises(self):
        """Non-date-based index raises ValueError."""
        with pytest.raises(ValueError):
            index_to_date("height", 0)


class TestIsDateIndex:
    """Test the is_date_index function."""

    def test_dateindex_is_date_based(self):
        assert is_date_index("dateindex") is True

    def test_weekindex_is_date_based(self):
        assert is_date_index("weekindex") is True

    def test_monthindex_is_date_based(self):
        assert is_date_index("monthindex") is True

    def test_yearindex_is_date_based(self):
        assert is_date_index("yearindex") is True

    def test_quarterindex_is_date_based(self):
        assert is_date_index("quarterindex") is True

    def test_semesterindex_is_date_based(self):
        assert is_date_index("semesterindex") is True

    def test_decadeindex_is_date_based(self):
        assert is_date_index("decadeindex") is True

    def test_height_is_not_date_based(self):
        assert is_date_index("height") is False

    def test_txindex_is_not_date_based(self):
        assert is_date_index("txindex") is False


# ============ MetricData helper method tests ============

class TestMetricDataHelpers:
    """Test MetricData helper methods."""

    def test_indexes_returns_range(self, date_based_metric):
        """indexes() returns list of index values."""
        assert date_based_metric.indexes() == [0, 1, 2, 3, 4]

    def test_indexes_with_offset(self, height_based_metric):
        """indexes() respects start/end offsets."""
        assert height_based_metric.indexes() == [800000, 800001, 800002, 800003, 800004]

    def test_dates_for_dateindex(self, date_based_metric):
        """dates() returns correct dates for dateindex."""
        dates = date_based_metric.dates()
        assert dates[0] == date(2009, 1, 3)  # dateindex 0 = genesis
        assert dates[1] == date(2009, 1, 9)  # dateindex 1 = day one
        assert dates[2] == date(2009, 1, 10)  # dateindex 2

    def test_dates_for_monthindex(self, month_based_metric):
        """dates() returns correct dates for monthindex."""
        dates = month_based_metric.dates()
        assert dates[0] == date(2009, 1, 1)
        assert dates[1] == date(2009, 2, 1)
        assert dates[2] == date(2009, 3, 1)

    def test_to_index_dict(self, date_based_metric):
        """to_index_dict() returns {index: value} mapping."""
        d = date_based_metric.to_index_dict()
        assert d[0] == 100
        assert d[1] == 200
        assert d[4] == 500

    def test_to_date_dict(self, date_based_metric):
        """to_date_dict() returns {date: value} mapping."""
        d = date_based_metric.to_date_dict()
        assert d[date(2009, 1, 3)] == 100  # genesis
        assert d[date(2009, 1, 9)] == 200  # day one

    def test_index_items(self, date_based_metric):
        """index_items() returns [(index, value), ...] pairs."""
        items = date_based_metric.index_items()
        assert items[0] == (0, 100)
        assert items[4] == (4, 500)

    def test_date_items(self, date_based_metric):
        """date_items() returns [(date, value), ...] pairs."""
        items = date_based_metric.date_items()
        assert items[0] == (date(2009, 1, 3), 100)
        assert items[1] == (date(2009, 1, 9), 200)

    def test_iter(self, date_based_metric):
        """iter() yields (index, value) pairs."""
        result = list(date_based_metric.iter())
        assert result == [(0, 100), (1, 200), (2, 300), (3, 400), (4, 500)]

    def test_iter_dates(self, date_based_metric):
        """iter_dates() yields (date, value) pairs."""
        result = list(date_based_metric.iter_dates())
        assert result[0] == (date(2009, 1, 3), 100)
        assert result[1] == (date(2009, 1, 9), 200)

    def test_default_iteration(self, date_based_metric):
        """Default iteration uses iter()."""
        result = list(date_based_metric)
        assert result == [(0, 100), (1, 200), (2, 300), (3, 400), (4, 500)]


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

    def test_to_polars_monthindex(self, month_based_metric):
        """to_polars() works with monthindex."""
        import polars as pl

        df = month_based_metric.to_polars()
        assert "date" in df.columns
        assert len(df) == 3
        # Polars stores dates as date type
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

    def test_to_pandas_monthindex(self, month_based_metric):
        """to_pandas() works with monthindex."""
        import pandas as pd

        df = month_based_metric.to_pandas()
        assert "date" in df.columns
        assert len(df) == 3
        dates = df["date"].tolist()
        assert dates[0] == date(2009, 1, 1)
        assert dates[1] == date(2009, 2, 1)
        assert dates[2] == date(2009, 3, 1)
