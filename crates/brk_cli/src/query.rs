use brk_computer::Computer;
use brk_indexer::Indexer;
use brk_query::{Index, Output, Params as QueryParams, Query, Tabled, Value};
use tabled::settings::Style;

use crate::run::RunConfig;

pub fn query(params: QueryParams) -> color_eyre::Result<()> {
    let config = RunConfig::import(None)?;

    let compressed = config.compressed();

    let mut indexer = Indexer::new(&config.outputsdir(), compressed, config.check_collisions())?;
    indexer.import_vecs()?;

    let mut computer = Computer::new(&config.outputsdir(), config.fetcher(), compressed);
    computer.import_vecs(&indexer, config.computation())?;

    let query = Query::build(&indexer, &computer);

    let index = Index::try_from(params.index.as_str())?;

    let ids = params.values.iter().map(|s| s.as_str()).collect::<Vec<_>>();

    let res = query.search_and_format(index, &ids, params.from, params.to, params.format)?;

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
