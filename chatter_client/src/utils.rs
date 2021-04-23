use std::collections::HashMap;
use async_std::{ 
    sync::{ Arc, Mutex }, task, io 
};
use std::sync::atomic::{AtomicU64, Ordering};
use serde::{ Deserialize, Serialize };
use serde_json::json;

use crate::requests;
use crate::LOGGED;
use crate::cui;

#[derive(Deserialize, Serialize)]
pub struct NewUserPayload {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessagePayload {
    pub success: bool,
    pub messages: Vec<(u64, String, String)>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Chats {
    pub chats: Vec<(u64)>,
}

#[derive(Serialize, Deserialize)]
pub struct NewMessage {
    pub c_id: u64,
    pub username: String,
    pub message: String,
}

pub struct LoggedUser {
    pub id: u64,
    pub username: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginPayload {
    pub success: bool,
    pub id: u64,
    pub username: String,
}

pub struct Message {
    m_id: u64,
    c_id: u64,
    message: String,
    printed: bool,
}

pub async fn menu() {
    println!("Welcome to Chatter.\n
        Enter: 'login', 'signup', or 'exit.'");
    
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).await;

        match input.trim().to_lowercase().as_str() {
            "login" => login_input().await,
            "signup" => new_user_input().await,
            "exit" => {
                exit();
                break;
            },
            _ => {},
        };
    }
}

pub async fn login_input() {

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut pass = String::new();
    let mut name = String::new();

    println!("Login:\n");
    println!("Enter username:");

    io::stdin().read_line(&mut input1).await.unwrap();

    println!("Enter password:");

    io::stdin().read_line(&mut input2).await.unwrap();

    let name = &input1.trim();
    let pass = &input2.trim();

    let login = 
        requests::request_login(NewUserPayload {
            username: name.to_lowercase().to_string(),
            password: pass.to_string(),
        }).await;

    match login {
        Ok(u) => {
            LOGGED.store(u.parse::<u64>().unwrap(), Ordering::Relaxed);
            logged_menu().await
        },
        Err(m) => {
            println!("{}", m);
            return;
        },
    };


    return;
}

pub async fn logged_menu() {
    loop {
        println!("Menu: chat, exit");

        let mut input = String::new();

        io::stdin().read_line(&mut input).await;

        match input.trim().to_lowercase().as_str() {
            "exit" => {
                exit();
                return;
            },
            "chat" => chat_menu().await,
            _ => {},
        };
    };
}

pub async fn new_user_input() {
   println!("New user:\n");
    let name: String =
        loop {
            println!("Enter new username:");

            let mut input = String::new();

            io::stdin().read_line(&mut input).await;

            match input.trim() {
                i if i.len() > 14 => {
                    println!("Entered username too long.");
                    {};
                },
                i if i.len() < 2 => {
                    println!("Entered username too short.");
                    {};
                },
                i => {
                    println!("Entered username: {}", i);
                    break i.to_lowercase().to_string();
                },
            };
        };

        let pass: String =
            loop {
                println!("Enter new password:");

                let mut input = String::new();

                io::stdin().read_line(&mut input).await;


                let input = match input.trim() {
                    i if i.len() > 14 => {
                        println!("Entered password too long.");
                        {};
                    },
                    i if i.len() < 2 => {
                        println!("Entered password too short.");
                        {};
                    },
                    i => {
                        println!("Entered password: {}", i);
                        break i.to_string();
                    },
                };
            };

        let response = requests::request_new_user( NewUserPayload {
            username: name,
            password: pass,
        }).await;

        match response {
            Ok(_) => println!("New user created."),
            Err(e) => println!("{}", e),
        };
}

async fn chat_menu() {
    println!("Logged user: {}", LOGGED.load(Ordering::Relaxed));
    loop {
        println!("Enter: 'chats', 'new', or 'exit'");
        let mut input = String::new();

        io::stdin().read_line(&mut input).await;

        match input.trim() {
            "back" => break,
            "chats" => chat_select().await,
            "new" => chat_new().await,
            _ => {},
        };
    };

}

async fn chat_select() {
    let logged_user = LOGGED.load(Ordering::Relaxed);

    println!("Select chat:");

    //let mut input = String::new();
    let chats = requests::get_chats(logged_user).await;
    
    let chat = match chats {
        Ok(c) => c,
        Err(m) => {
            println!("{}", m);
            return;
        },
    };

    for c in chat.iter() {
        println!("{:?}", c);
    };

    //loop {
    //    println!("Select chat, or type 'back'");
    //
    //    io::stdin().read_line(&mut input).await;
    //
    //    let selection = match input.trim() {
    //        "back" => return,
    //        sel => sel,
    //    };
    //    
    //    let select: u64 = selection.parse().unwrap();
    //    
    //    if chat.contains(&select) {
    //        //cui::chatting(select);
    //        println!("{}", select);
    //    };
    //};
}

async fn chat_new() {
    println!("Enter recipients:");

    let mut input = String::new();
    
    io::stdin().read_line(&mut input).await;

    let input = String::from(input.trim().to_lowercase());

    let user = LOGGED.load(Ordering::Relaxed);

    let newchat = requests::put_new_chat(user, input).await;

    match newchat {
        Ok(m) => println!("{}", m),
        Err(e) => println!("{}", e),
    };
}

pub fn exit() {
    println!("Goodbye.");
}
