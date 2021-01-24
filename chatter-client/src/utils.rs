use serde::{ Deserialize, Serialize };


struct NewUserPayload {
    username: String,
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
