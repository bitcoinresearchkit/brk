use tabled::{Table, builder::Builder};

pub trait Tabled {
    fn to_table(&self, ids: Vec<String>) -> Table;
}

impl Tabled for Vec<Vec<serde_json::Value>> {
    fn to_table(&self, ids: Vec<String>) -> Table {
        let mut builder = Builder::default();

        builder.push_record(ids);

        if let Some(first) = self.first() {
            let len = first.len();

            (0..len).for_each(|index| {
                builder.push_record(self.iter().map(|vec| vec.get(index).unwrap().to_string()));
            });
        }

        builder.build()
    }
}
