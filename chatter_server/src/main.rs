use std::collections::HashMap;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate log;
use actix_web::{get, web, App, Error, HttpResponse, HttpServer, Responder, Result as ActixResult};
use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};
use mysql::*;
use mysql::prelude::*;

mod db_layer;
use db_layer::{ NewUser, NewUserPayload, User, Message, Messages, NewMessage, NewChat, Id, LoginUser, NewChatroom };


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
    info!("Attempting to add new user: {}", &body.username);

    let mut conn = pool.get_conn();

    let mut conn = match conn {
        Ok(c) => c,
        Err(_) => return Ok(HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": "Internal server error: could not connect to database",
        }))),
    };

    let inserted = web::block(move ||
        db_layer::insert_user(&mut conn, body.into_inner()))
        .await;

    let inserted = match inserted {
        Ok(i) => i,
        Err(_) => return Ok(HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": "Internal server error: internal database connection error.",
        }))),
    };   

    //let payload = match inserted {
    //    Ok(i) => i,
    //    Err(e) => return Ok(HttpResponse::BadRequest().json(json!({
    //        "success": false,
    //        "message": e,
    //    }))),
    //};

    return Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "message" : "Inserted user",
    })));

    //return Ok(HttpResponse::Ok().json(json!({
    //    "success": true,
    //    "message" : User {
    //        username: inserted.username,
    //        id: inserted.id,
    //    },
    //})));
}

async fn user_login(pool: web::Data<mysql::Pool>, user: web::Json<NewUserPayload>) -> ActixResult<HttpResponse> {
    info!("Attempting to login user: {:?}", &user.username);

    let conn = pool.get_conn();

    let mut conn = match conn {
        Ok(c) => c,
        Err(_) => return Ok(HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": "Internal server error: could not connect to database",
        }))),
    };

    let login = web::block(move || 
        db_layer::login(&mut conn, user.into_inner())).await;

    match login {
        Ok(l) => return Ok(HttpResponse::Ok().json(json!({
            "message": l.id.to_string(),
            "success": true,
        }))),
        Err(_) => {
            info!("Login BadRequest!");
            return Ok(HttpResponse::BadRequest().json(json!({
                "success": false,
                "message": "Internal server error (login)."
            })));
        },
    };

    //let login = match login {
    //    Ok(l) => return Ok(HttpResponse::Ok().json(json!({
    //        "success": true,
    //        "message": l,
    //    }))),
    //    Err(e) =>  match e {
    //        actix_web::error::BlockingError => return Ok(HttpResponse::InternalServerError().json(json!({
    //            "success": false,
    //            "message": "Internal server error: internal database error.",
    //        }))),
    //        other_error => return Ok(HttpResponse::BadRequest().json(json!({
    //            "success": false,
    //            "message": other_error
    //        }))),
    //    },
    //};
}

async fn new_chat(pool: web::Data<mysql::Pool>, chat: web::Json<NewChatroom>) -> ActixResult<HttpResponse> {
    info!("Attempting to create new chattroom from user #:{}, to user:{}", &chat.u_id_1, &chat.u_name_2);
    let conn = pool.get_conn();

    let mut conn = match conn {
        Ok(c) => c,
        Err(_) => return Ok(HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": "Internal server error: could not connect to database",
        }))),
    };

    let newchat = web::block(move || 
        db_layer::insert_chatroom(&mut conn, chat.into_inner()
    )).await;

    match newchat {
        Ok(n) => return Ok(HttpResponse::Ok().json(json!({
            "success": true,
            "message": n,
        }))),
        Err(_) => return Ok(HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "Could not create chat"
        }))),
    };
}

async fn get_chats(pool: web::Data<mysql::Pool>, user: web::Json<Id>) -> ActixResult<HttpResponse> {
    info!("Getting chats for user id: {}", &user.id);
    let conn = pool.get_conn();

    let mut conn = match conn {
        Ok(c) => c,
        Err(_) => return Ok(HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": "Internal server error: could not connect to database",
        }))),
    };

    let user = user.into_inner();

    let chat_list = web::block(move ||
        db_layer::user_chats(&mut conn, user.id)
    ).await;

    //let chat_list: Vec<u64> = match chat_list {
    //    Ok(c) => c,
    //    Err(m) => return Ok(HttpResponse::BadRequest().json(json!({
    //        "success": false,
    //        "message": m,
    //    }))),
    //};

    if chat_list.is_err() {
        return Ok(HttpResponse::BadRequest().json(json!({
                    "success": false,
                    "message": "There is an error from the database.",
                })))
    };

    let chats: HashMap<u64, String> = chat_list.unwrap();

    if chats.len() == 0 {
        return Ok(HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "User does not have any chats yet",
        })));
    } else {
        return Ok(HttpResponse::Ok().json(json!({
            "success": true,
            "message": json!(chats),
        })));
    };
}

//async fn get_messages(pool: web::Data<mysql::Pool>, c_id: web::Json<u64>) -> ActixResult<HttpResponse> {
//    let mut conn = pool.get_conn().unwrap();
//
//    let messages = web::block(move || 
//        db_layer::show_messages(&mut conn, c_id)
//    ).await;
//
//    match messages {
//        Ok(m) => return Ok(HttpResponse::Ok().json(json!({
//            "success": true,
//            "messages": messages,
//        }))),
//        Err(_) => return Ok(HttpResponse::BadRequest().json(json!({
//            "success": false,
//            "message": "error retrieving messages",
//        }))),
//    };
//}

async fn push_message(pool: web::Data<mysql::Pool>, message: web::Json<NewMessage>) -> ActixResult<HttpResponse> {
    info!("Attempting to insert message for chat: {:?}", &message.c_id);

    let mut conn = pool.get_conn();

    let mut conn = match conn {
        Ok(c) => c,
        Err(_) => return Ok(HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": "Internal server error: could not connect to database",
        }))),
    };
    
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
    println!{"hello"};
    env_logger::init();
    info!("Starting the server...");

    let pool = Pool::new("mysql://root:password@db:3306/chatter_db").unwrap();

    HttpServer::new(move || {
        App::new()
        .data(pool.clone())
        .service(index)
        .route("/test", web::get().to(test))
        .route("/user/new", web::put().to(user_new))
        .route("/user/login", web::post().to(user_login))
        .route("/user/chats", web::post().to(get_chats))
        .route("/message/new", web::put().to(push_message))
        .route("/message/new_chat", web::put().to(new_chat))
    })
        .bind("0.0.0.0:8088")?
        .run()
        .await
}

        //.route("/message/chatting", web::get().to(get_messages))
