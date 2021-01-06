use serde::{ Deserialize, Serialize };

pub const URL: &str = "https://localhost:8088";

struct NewUserPayload {
    username: String,
}

pub async fn home() -> Result<(), reqwest::Error> {
    let url = "https://localhost:8088/"

    let res = reqwest::get(&*url)
        .await?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    Ok()
}

pub async fn request_new_user(username: &str) -> Result<(), reqwest::Error> {
    let payload = NewUserPayload { username: &username.to_string() };

    let client = reqwest::Client::new();

    let res = client.post(&*URL)
                .body(serde_json::to_string(&payload))
                .send()
                .await?

    Ok()
}

pub fn new_user_input() -> Result<()> {

    loop {
        println!("Enter new username:");

        let mut input = String::new();

        io::stdin().read_line(&mut input);

        match input.trim() {
            "exit" => break,
            i if i.len() > 14 => {
                println!("Entered username too long.");
                continue;
            },
            i if i.len() > 6 => {
                println!("Entered username too short.");
                continue;
            },
            i => {
                request_new_user(i);
                break;
            },
        };
    };

    Ok()
}

pub fn menu() -> Result<()> {
    loop {
        println!("menu: show, add, exit");

        let mut input = String::new();

        io::stdin().read_line(&mut input);

        match input.trim().to_lowercase() {
            "add" => new_user_input(),
            "exit" => exit(),
            _ => continue,
        };
    };
    Ok()
}

pub fn exit() {
    println!("Goodbye.");
}
