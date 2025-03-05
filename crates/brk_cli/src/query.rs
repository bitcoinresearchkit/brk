use brk_computer::Computer;
use brk_indexer::Indexer;
use brk_query::{Index, Output, Params as QueryParams, Query, Tabled, Value};
use tabled::settings::Style;

use crate::run::RunConfig;

pub fn query(params: QueryParams) -> color_eyre::Result<()> {
    let config = RunConfig::import(None)?;

    let mut indexer = Indexer::new(config.indexeddir())?;
    indexer.import_vecs()?;

    let mut computer = Computer::new(config.computeddir());
    computer.import_vecs()?;

    let query = Query::build(&indexer, &computer);

    let ids = params
        .values
        .iter()
        .flat_map(|v| v.split(","))
        .collect::<Vec<_>>();

    let index = Index::try_from(params.index.as_str())?;

    let res = query.search(index, &ids, params.from, params.to, params.format)?;

    if params.format.is_some() {
        println!("{}", res);
    } else {
        println!(
            "{}",
            match res {
                Output::Json(v) => match v {
                    Value::Single(v) => v.to_string().replace("\"", ""),
                    v => {
                        let v = match v {
                            Value::Single(_) => unreachable!("Already processed"),
                            Value::List(v) => vec![v],
                            Value::Matrix(v) => v,
                        };
                        let mut table =
                            v.to_table(ids.iter().map(|id| id.to_string()).collect::<Vec<_>>());
                        table.with(Style::psql());
                        table.to_string()
                    }
                },
                _ => unreachable!(),
            }
        );
    }

    Ok(())
}
