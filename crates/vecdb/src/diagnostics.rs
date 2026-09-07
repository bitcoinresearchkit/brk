use std::cell::Cell;

thread_local! {
    static PAGES: Cell<usize> = const { Cell::new(0) };
    static COLUMNS: Cell<usize> = const { Cell::new(0) };
}

pub(crate) fn page() {
    PAGES.with(|count| count.set(count.get() + 1));
}

pub(crate) fn column() {
    COLUMNS.with(|count| count.set(count.get() + 1));
}

/// Reset and return (page decode attempts, sparse column reads) on this thread.
/// Includes raw pages. Counts operations, not unique pages or columns.
pub fn take() -> (usize, usize) {
    (PAGES.replace(0), COLUMNS.replace(0))
}
