#[macro_use] extern crate serde_json;
#[macro_use] extern crate log;
use actix_web::{get, web, App, Error, HttpResponse, HttpServer, Responder, Result as ActixResult};
use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};
use mysql::*;
use mysql::prelude::*;

mod db_layer;
use db_layer::{ NewUser, NewUserPayload, User, Message, Messages, NewMessage, NewChat, Id };


async fn home() -> Result<HttpResponse> {
    Ok(
        HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../templates/index.html"))
    )
}

async fn test() -> impl Responder {
    HttpResponse::Ok().body("Hello from chatter server.")
}

#[get("/{id}/{name}/index.html")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    info!("get/index");
    format!("Hello {}! id:{}", name, id)
}

async fn user_new(pool: web::Data<mysql::Pool>, body: web::Json<NewUserPayload>) -> ActixResult<HttpResponse> {
    info!("adding new user:{}", &body.username);

    let mut conn = pool.get_conn().unwrap();

    let inserted = web::block(move ||
        db_layer::insert_user(&mut conn, &body.username, &body.password))
        .await.unwrap();

    return Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "user" : User {
            username: inserted.username,
            id: inserted.id,
        },
    })));
}

async fn user_login(pool: web::Data<mysql::Pool>, user: web::Json<NewUserPayload>) -> ActixResult<HttpResponse> {
    info!("Attempting to login user: {:?}", &user.username);

    let mut conn = pool.get_conn().unwrap();

    let login = web::block(move || 
        db_layer::login(&mut conn, user.into_inner())).await.unwrap();

    if let Some(u) = login {
        return Ok(HttpResponse::Ok().json(json!({
            "success": true,
            "message": User {
                username: u.username,
                id: u.id,
            },
        })));
    };
    return Ok(HttpResponse::BadRequest().json(json!({
        "success": false,
        "message": "invalid username or password",
    })));
}

async fn new_chat(pool: web::Data<mysql::Pool>, chat: web::Json<NewChat>) -> ActixResult<HttpResponse> {
    let mut conn = pool.get_conn().unwrap();

    let newchat = web::block(move || 
        db_layer::insert_chat(&mut conn, chat.into_inner())
    );

    return Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "message": newchat,
    })));
}

async fn get_chats(pool: web::Data<mysql::Pool>, user: web::Json<Id>) -> ActixResult<HttpResponse> {
    let mut conn = pool.get_conn().unwrap();

    let chat_list = web::block(move ||
        db_layer::user_chats(&mut conn, user.id)
    );

    match chat_list {
        Ok(list) => return Ok(HttpResponse::Ok().json(json!({
            "success": true,
            "chat list": json!(list)
        }))),
        Err(e) => return Ok(HttpResponse::Ok().json(json!({
            "success": false,
            "message": "error retrieving chat list"
        }))),
    }
}

async fn get_messages(pool: web::Data<mysql::Pool>, c_id: web::Json<u64>) -> ActixResult<HttpResponse> {
    let mut conn = pool.get_conn().unwrap();

    let messages = web::block(move || 
        db_layer::show_chats(&mut conn, c_id)
    );

    match messages {
        Ok(m) => return Ok(HttpResponse::Ok().json(json!(
            "success": true,
            "messages": messages,
        ))),
        Err(_) => return Ok(HttpResponse::Ok().json(json!(
            "success": false,
            "message": "error retrieving messages",
        ))),
    };
}

async fn push_message(pool: web::Data<mysql::Pool>, message: web::Json<NewMessage>) -> ActixResult<HttpResponse> {
    info!("Attempting to insert message for chat: {:?}", &message.c_id);

    let mut conn = pool.get_conn().unwrap();
    
    let inserted = web::block(move || 
        db_layer::insert_message(&mut conn, message.into_inner()))
        .await
        .unwrap();

    return Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "chat id": inserted,
    })));

    //if let Some(m) = inserted {
    //    return Ok(HttpResponse::Ok().json(json!({
    //        "success": true,
    //        "message": m,
    //    })))
    //};

    //return Ok(HttpResponse::BadRequest().json(json!({
    //    "success": false,
    //    "message": "error inserting chat"
    //})));

}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Starting the server...");

    let pool = Pool::new("mysql://root:password@db:3306/chatter_db").unwrap();

    HttpServer::new(move || {
        App::new()
        .data(pool.clone())
        .service(index)
        .route("/test", web::get().to(test))
        .route("/user/new", web::post().to(user_new))
        .route("/user/login", web::post().to(user_login))
        .route("/user/chats", web::get().to(get_chats))
        .route("/message/new", web::put().to(push_message))
        .route("/message/new_chat", web::put().to(new_chat))
        .route("/message/chatting", web::get().to(get_messages))
    })
        .bind("0.0.0.0:8088")?
        .run()
        .await
}
