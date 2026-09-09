use std::cell::Cell;

thread_local! {
    static PAGES: Cell<usize> = const { Cell::new(0) };
}

pub(crate) fn page() {
    PAGES.with(|count| count.set(count.get() + 1));
}

/// Reset and return page decode attempts on this thread, including raw pages.
pub fn take() -> usize {
    PAGES.replace(0)
}
