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

pub fn select_messages(conn: &mut mysql::PooledConn, c_id: u64) -> std::result::Result<Vec<Messages>, mysql::error::Error> {
    let mut stmt = conn.prep(
        "SELECT m_id, chat_message, author FROM messages WHERE c_id=:c_id ORDER BY m_id LIMIT 10"
    );

    conn.exec_map(&stmt, 
        params! {
            "c_id" => c_id,
        },
        |(m_id, chat_message, author)| Messages {
            message: message,
            author: author,
        }
    )
}

pub fn user_chats(conn: &mut mysql::PooledConn, user: User) -> std::result::Result<Vec<u64>> {
    let mut stmt = conn.prep(
        "SELECT m_id, author, chat_message WHERE user_1 OR user_2 = (:username)"
    ).unwrap();

    conn.exec_map(stmt, 
        params! {
            "username" => user.username,
        }
    )
}

pub fn find_product_in_price_range(
    conn: &mut PooledConn,
    price_from: f32,
    price_to: f32) -> std::result::Result<Vec<Product>, mysql::error::Error> {
    conn.exec_map(
        "select product_id, product_code, price, name, last_update from PRODUCT where price>=:price_from and price <=:price_to",
        params! {
            "price_from" => price_from,
            "price_to" => price_to,
        },
        |(product_id, product_code, price, name, last_update)| Product {
            id: product_id,
            code: product_code,
            price: price,
            product_name: name,
            last_changed_on: last_update
        }
    )
}

pub fn login(conn: &mut mysql::PooledConn, login: NewUserPayload) -> Result<Option<User>> {
    let qry = conn.exec_first(
        "SELECT id, username FROM users WHERE username=:username AND pw=:password",
        params!{
            "username" => &login.username,
            "password" => &login.password,
        })
        .map(|row| {
            row.map(|(id, username)| User {
                id: id,
                username: username
            })
        });
        return Ok(qry.unwrap());
}

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
        Some(user) => println!("Returned user:\nid: {:?}\nUsername: {:?}", user.id, user.username),
        None => println!("User not found."),
    };
}

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
