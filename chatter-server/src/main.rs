use actix_web::{ App, Error, HttpResponse, HttpServer, web ;
use actix_web::http::{StatusCode};
use serde::{ Deserialize, Serialize };


#[derive(Deserialize)]
pub struct NewUserPayload {
    username: String,
}


async fn home() -> Result<HttpResponse, Error> {
    Ok(
        HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../templates/index.html"))
    )
}


fn insert_new_user(conn: &mysql::PooledConn, new_user: User) -> Result<()> {
    conn.exec_batch(
        r"INSERT INTO users (username)
          VALUES (:username)", new_user.iter().map(|u| params! {
              "username" => &u.username,
          })
    )?;
    Ok()
}

async fn new_user_index(pool: web::Data<mysql::Pool>, body: web::Json<NewUserPayload>) -> impl Responder {
    let conn = pool.get_conn();

    let user = web::block(move || actions::insert_new_user(&conn, &body))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok())

}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let url = "mysql://root:password@db:3306/chatter_db";

    let pool = Pool::new(url)?;

    HttpServer::new(move || {
        App::new::data(pool.clone())
            .resource("/new_user", web::post().to(new_user_index)
            .route("/", web::get().to(home))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await

    Ok()
}
