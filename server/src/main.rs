use std::path::Path;

use computer::Computer;
use indexer::Indexer;
use storable_vec::STATELESS;

#[tokio::main]
pub async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    logger::init_log(None);

    let path = Path::new("../_outputs");
    let indexer: Indexer<STATELESS> = Indexer::import(&path.join("indexes"))?;
    let computer: Computer<STATELESS> = Computer::import(&path.join("computed"))?;

    berver::main(indexer, computer).await.unwrap();

    Ok(())
}
