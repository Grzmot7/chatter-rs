
use std::io;
use serde::{ Deserialize, Serialize };

use crate::requests;


#[derive(Deserialize, Serialize)]
pub struct NewUserPayload {
    pub username: String,
    pub password: String,
}

pub fn menu() {
    loop {
        println!("menu: show, add, exit");

        let mut input = String::new();

        io::stdin().read_line(&mut input);

        match input.trim().to_lowercase().as_str() {
            "add" => new_user_input(),
            "exit" => exit(),
            "show" => continue,
            _ => continue,
        };
    };
}


pub fn new_user_input() {

    let mut input = String::new();

    let name: String =
        loop {
            println!("Enter new username:");

            let mut input = String::new();

            io::stdin().read_line(&mut input);


            match input.trim() {
                i if i.len() > 14 => {
                    println!("Entered username too long.");
                    continue;
                },
                i if i.len() < 6 => {
                    println!("Entered username too short.");
                    continue;
                },
                i => {
                    println!("Entered username: {}", i);
                    break i.to_string();
                },
            };
        };

        let pass: String =
            loop {
                println!("Enter new username:");

                let mut input = String::new();

                io::stdin().read_line(&mut input);


                match input.trim() {
                    i if i.len() > 14 => {
                        println!("Entered username too long.");
                        continue;
                    },
                    i if i.len() < 6 => {
                        println!("Entered username too short.");
                        continue;
                    },
                    i => {
                        println!("Entered username: {}", i);
                        break i.to_string();
                    },
                };
            };

        requests::request_new_user(NewUserPayload {
            username: name,
            password: pass,
        });
}



pub fn exit() {
    println!("Goodbye.");
}
