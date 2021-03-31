use std::sync::atomic::{ AtomicU64, Ordering }

mod utils;
mod requests;
mod cui;
mod events;

static mut LOGGED: AtomicU64 = AtomicU64::new()

#[async_std::main]
async fn main() -> Result<(), reqwest::Error> {

   requests::test_connection().await;
/*
    let test = requests::test_connection();

    match test {
        Ok(r) => println!("Connected to server."),
        Err(e) => panic!("Could not connect to the server."),
    };
*/
    utils::menu().await;
    
    cui::chatting().await;

    

    Ok(())
}
