// Should be async
// anything related to IO should use
//
// Sync function
// fn get(db: &CandyStore, key: &str) -> Option<Vec<u8>> {
//     db.get(key).ok().flatten()
// }

use crate::Query;

// // Async function
// async fn get_async(db: Arc<CandyStore>, key: String) -> Option<Vec<u8>> {
//     tokio::task::spawn_blocking(move || {
//         db.get(&key).ok().flatten()
//     }).await.ok()?
// }
#[derive(Clone)]
pub struct AsyncQuery(Query);
