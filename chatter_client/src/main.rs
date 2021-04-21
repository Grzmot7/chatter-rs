use std::sync::atomic::{ AtomicU64, Ordering };

mod utils;
mod requests;
mod cui;
mod events;

static LOGGED: AtomicU64 = AtomicU64::new(0);

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

   requests::test_connection().await;

    utils::menu().await;

    //cui::chatting().await;
    Ok(())
}
