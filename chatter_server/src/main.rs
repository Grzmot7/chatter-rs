#[macro_use] extern crate serde_json;
#[macro_use] extern crate log;
use actix_web::{get, web, App, Error, HttpResponse, HttpServer, Responder, Result as ActixResult};
use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};
use mysql::*;
use mysql::prelude::*;

mod db_layer;
use db_layer::{ NewUser, NewUserPayload, User };


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

async fn new_user(pool: web::Data<mysql::Pool>, body: web::Json<NewUserPayload>) -> ActixResult<HttpResponse> {
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
        .route("/user/new", web::post().to(new_user))
    })
        .bind("0.0.0.0:8088")?
        .run()
        .await
}
