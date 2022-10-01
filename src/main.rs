extern crate dotenvy;

use actix_web::{App, HttpServer, post, web, Result};
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use dotenvy::dotenv;
use std::env;

#[derive(Deserialize)]
struct Post {
    user_id: i32,
    content: String,
}

#[derive(sqlx::FromRow)]
struct User { id: i32, username: String }

#[post("/send_message")]
async fn send_message(pool: web::Data<sqlx::Pool<sqlx::Postgres>>, data: web::Json<Post>) -> Result<String> {
    let user = sqlx::query_as!(User, "SELECT id, username FROM users WHERE users.id = $1 LIMIT 1", data.user_id)
        .fetch_one(pool.get_ref())
        .await.unwrap();

    Ok(format!("{} (ID: {}): {}", user.username, user.id, data.content))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    let db_url = env::var("DATABASE_URL").unwrap();
    println!("DATABASE_URL: {}", db_url);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url).await.unwrap();

    HttpServer::new(move|| 
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(send_message)
        )
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
