
mod utils;
mod requests;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {


    requests::test_connection();
/*
    let test = requests::test_connection();

    match test {
        Ok(r) => println!("Connected to server."),
        Err(e) => panic!("Could not connect to the server."),
    };
*/

    //utils::menu();

    Ok(())
}
