use super::{CohortContext, Filter};

pub trait Filtered {
    fn filter(&self) -> &Filter;

    fn is_all(&self) -> bool {
        self.filter().is_all()
    }

    fn includes_first_day(&self) -> bool {
        self.filter().includes_first_day()
    }

    fn name_suffix(&self) -> String {
        self.filter().to_name_suffix()
    }

    fn full_name(&self, context: CohortContext) -> String {
        self.filter().to_full_name(context)
    }
}
