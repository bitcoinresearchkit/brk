use std::path::Path;

use brk_computer::Computer;
use brk_indexer::Indexer;
use storable_vec::STATELESS;

#[tokio::main]
pub async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    brk_logger::init(None);

    let path = Path::new("../../_outputs");
    let indexer: Indexer<STATELESS> = Indexer::import(&path.join("indexes"))?;
    let computer: Computer<STATELESS> = Computer::import(&path.join("computed"))?;

    brk_server::main(indexer, computer).await.unwrap();

    Ok(())
}
