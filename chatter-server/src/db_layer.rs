//#[macro_use] extern crate serde_json;
//#[macro_use] extern crate log;
use serde::{Deserialize, Serialize};

use mysql::*;
use mysql::prelude::*;


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
    pub username: String
}

//functional
pub fn insert_user(conn: &mut mysql::PooledConn, uname: &String, pass: &String) -> Result<User> {

    let user = NewUserPayload {
        username: String::from(uname),
        password: String::from(pass),
    };

    let mut stmt = conn.prep(
        "INSERT INTO users (username, pw) VALUES (:username, :pw)"
        )
        .unwrap();

    conn.exec_drop(&stmt, params! {
        "username" => &user.username,
        "pw" => &user.password,
    }).unwrap();

    let new_id = conn.last_insert_id();

    let inserted_user = User {
        id: new_id,
        username: user.username,
    };

    return Ok(inserted_user);

}


// single item operation, no route, not implemented
fn select_user(pool: mysql::Pool, uname: &str) {
    let mut conn = pool.get_conn().unwrap();

    let res = conn
        .exec_first("SELECT id, username FROM users WHERE username=:username",
        params! {
            "username" => uname,
        })
        .map(|row| {
            row.map(|(id, username)| User {
                id: id,
                username: username,
            })
        });

    match res.unwrap() {
        Some(user) => println!("Returned user:\nid: {}\nUsername: {}", user.id, user.username),
        None => println!("User not found."),
    };
}

//works (streamed query), no route, not implemented
fn show_all_users(pool: mysql::Pool) {
    let mut conn = pool.get_conn().unwrap();

    conn.query_iter("SELECT id, username, pw FROM users")
    .unwrap()
    .for_each(|row| {
        let r:(i32, String, String) = from_row(row.unwrap());
        println!("{}, {}, {:?}", r.0, r.1, r.2);
    });
}
