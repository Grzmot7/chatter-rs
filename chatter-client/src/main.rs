extern crate chatter-client;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    chatter-client::menu();

    Ok()
}
