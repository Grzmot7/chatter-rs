use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use mysql::*;
use mysql::prelude::*;



#[derive(Serialize, Deserialize)]
pub struct NewMessage {
    pub c_id: u64,
    pub message: String,
    pub author: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub c_id: u64,
    pub message: String,
    pub author: u64,
    pub m_id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Messages {
    pub id: u64,
    pub message: String,
    pub author: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewUserPayload {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    id: u64,
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub username: String,
}

pub struct LoginUser {
    pub id: Option<u64>,
    pub username: Option<String>,
}

#[derive(Deserialize)]
pub struct Id{
    pub id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct NewChat {
    pub user_1: u64,
    pub user_2: u64,
}

pub fn insert_message(conn: &mut mysql::PooledConn, msg: NewMessage) -> Result<u64> {
    conn.exec_drop(
        "INSERT INTO messages (message, author, c_id) VALUES (message=:message, author=:author, c_id=:c_id)",
        params!{
            "message" => &msg.message,
            "author" => &msg.author,
            "c_id" => &msg.c_id,
        }
    );

    Ok(conn.last_insert_id())
}

//pub fn select_messages(conn: &mut mysql::PooledConn, c_id: u64) -> std::result::Result<Vec<Messages>, mysql::error::Error> {
//    let mut stmt = conn.prep(
//        "SELECT m_id, chat_message, author FROM messages WHERE c_id=:c_id ORDER BY m_id LIMIT 10"
//    );
//
//    conn.exec_map(&stmt, 
//        params! {
//            "c_id" => c_id,
//        },
//        |(m_id, message, author)| Messages {
//            message: message,
//            author: author,
//        }
//    )
//}

pub fn user_chats(conn: &mut mysql::PooledConn, id: u64) -> std::result::Result<HashMap<u64, String>, String> {
    let username: Result<Option<u64>> = conn.exec_first(
        "SELECT username FROM users WHERE u_id=:user",
        params! {
            "u_id" => id,
        }
    );

    let username = match username {
        Ok(u) => u,
        Err(_) => return Err(String::from("Database error.")),
    };

    if let Some(n) = username {
        let username = n;
    } else {
        return Err(String::from("Author not recognized.") );
    };

    let mut stmt = conn.prep(
        "SELECT c_id FROM chatrooms WHERE user_1=:user OR user_2=:user"
    ).unwrap();

    let user_chats: Result<Vec<u64>> = conn.exec(stmt, 
        params! {
            "user" => username,
        }
    );

    let user_chats = match user_chats {
        Ok(c) => c,
        Err(_) => return Err(String::from("Database error.")),
    };

    if user_chats.len() == 0 {
        return Err(String::from("User does not have any chats yet."));
    };

    let mut stmt = conn.prep(
        "SELECT (user_1, user_2) FROM chatrooms WHERE user_1=:user OR user_2=:user"
    ).unwrap();

    let participents: Result<Vec<(String, String)>> = conn.exec(stmt, 
        params!{
            "user" => username,
        }
    );

    let participents = match participents {
        Ok(p) => p,
        Err(_) => return Err(String::from("Database error.")),
    };

    let recips: Vec<String> = participents.iter().map(|x| 
        match &x.0 {
            username => x.1.to_string(),
            _ => x.0.to_string(),
        }
    ).collect();

    let chat_list: HashMap<u64, String> = user_chats.into_iter().zip(recips.into_iter()).collect();

    Ok(chat_list)

}

pub fn show_messages() {

}


pub fn login(conn: &mut mysql::PooledConn, login: NewUserPayload) -> std::result::Result<u64, String> {
    let qry = conn.exec_first(
        "SELECT id FROM users WHERE username=:username AND pw=:password",
        params!{
            "username" => &login.username,
            "password" => &login.password,
        });
        
        let qry = match qry {
            Ok(q) => q,
            Err(_) => return Err(String::from("Database error.")),
        };

        if let Some(q) = qry {
            return Err(String::from("Bad username or password"));
        };

        return Ok(qry.unwrap());
}

pub fn insert_user(conn: &mut mysql::PooledConn, user: NewUserPayload) -> std::result::Result<User, String> {
    let check: Result<Option<u64>> = conn.exec_first(
        "SELECT u_id FROM users WHERE username=:username",
        params! {
            "username" => user.username,
        }
    );

    let check = match check {
        Ok(c) => c,
        Err(_) => return Err(String::from("Database error.")),
    };

    if let Some(u) = check {
        let err_m = String::from("User already exists");
        return Err(err_m);
    };

    let mut stmt = conn.prep(
        "INSERT INTO users (username, pw) VALUES (:username, :pw)"
        );

    let stmt = match stmt {
        Ok(s) => s,
        Err(_) => return Err(String::from("Database error.")),
    };

    conn.exec_drop(&stmt, params! {
        "username" => &user.username,
        "pw" => &user.password,
    });

    //let conn = match conn {
    //    Ok(c) => c,
    //    Err(_) => return Err(String::from("Database error: error inserting user.")),
    //};

    let new_id = conn.last_insert_id();

    let inserted_user = User {
        id: new_id,
        username: user.username,
    };

    return Ok(inserted_user);
}

