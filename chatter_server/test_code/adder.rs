use mysql::*;
use mysql::prelude::*;
use std::io;

pub struct User {
    username: i32,
}

pub fn menu() -> Result<()> {
    loop {
        println!("menu: show, add, exit");

        let mut input = String::new();

        io::stdin().read_line(&mut input);

        match input.trim().to_lowercase() {
            "show" => print_users(),
            "add" => new_user_input(),
            "exit" => exit(),
            _ => continue,
        };
    };
    Ok()
}


pub fn add_user(name: &Str) -> Result<()> {
    let url = "mysql://root:password@db:3306/chatter_db";

    let pool = Pool::new(url)?;

    let mut conn = pool.get_conn()?;

    let new_user = User { username: &name };

    conn.exec_batch(
        r"INSERT INTO users (username)
          VALUES (:username)", new_user.iter().map(|u| params! {
              "username" => &u.username,
          })
    )?;
    Ok()
}

pub fn exit() {
    println!("Goodbye.");
}

pub fn new_user_input() -> Result<()> {
    loop {
        println!("Enter new username:");

        let mut input = String::new();

        io::stdin().read_line(&mut input);

        match input.trim() {
            "exit" => break,
            i if i > 14 => {
                println!("Entered username too long.");
                continue;
            },
            i if i > 6 => {
                println!("Entered username too short.");
                continue;
            },
            i => {
                add_user(i);
                break;
            },
        };
    };
}

pub fn print_users() -> Result<()> {
    let url = "mysql://root:password@db:3306/chatter_db";

    let pool = Pool::new(url)?;

    let mut conn = pool.get_conn()?;

}
