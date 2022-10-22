extern crate dotenvy;

use actix_web::{App, HttpServer, get, post, web, Result, middleware, cookie};
use actix_session::{Session, SessionMiddleware, storage::CookieSessionStore};
use serde::{Deserialize};
use sqlx::postgres::PgPoolOptions;
use dotenvy::dotenv;
use std::{env};
use sqlx::types::chrono::{Utc, NaiveDateTime};
use env_logger::Env;

#[derive(Debug, Deserialize)]
struct ReadMessageParams {
    user_id: i32,
    message_id: i32,
}

#[derive(Deserialize)]
struct Post {
    user_id: i32,
    content: String,
}

struct Message {
    id: i32,
    user_id: i32,
    created_at: NaiveDateTime,
    content: String,
}

#[derive(sqlx::FromRow)]
struct User { id: i32, username: String }

#[post("/send_message")]
async fn send_message(pool: web::Data<sqlx::Pool<sqlx::Postgres>>, data: web::Json<Post>) -> Result<String> {
    let res = sqlx::query_as!(User, "SELECT id, username FROM users WHERE users.id = $1 LIMIT 1", data.user_id)
        .fetch_one(pool.get_ref())
        .await;
    
    let user = match res {
        Err(e) => {
            match e {
                sqlx::Error::RowNotFound => return Err(actix_web::error::ErrorNotFound(e)),
                _ => return Err(actix_web::error::ErrorInternalServerError(e)),
            }
        }
        Ok(u) => u,
    };

    let created_at = NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0);
    let res = sqlx::query!("INSERT INTO messages (user_id, content, created_at) VALUES ($1, $2, $3)", user.id, data.content, created_at)
        .execute(pool.get_ref())
        .await;

    let count = match res {
        Err(e) => return Err(actix_web::error::ErrorInternalServerError(e)),
        Ok(c) => c.rows_affected(),
    };

    Ok(format!("Saved: {}", count))
}

#[get("/users/{user_id}/messages/{message_id}")]
async fn read_message(
    session: Session,
    pool: web::Data<sqlx::Pool<sqlx::Postgres>>, 
    params: web::Path<ReadMessageParams>
) -> Result<String> {
    if let Some(count) = session.get::<i32>("counter")? {
        session.insert("counter", count + 1)?;
    } else {
        session.insert("counter", 1)?;
    }

    let res = sqlx::query_as!(User, "SELECT id, username FROM users WHERE users.id = $1 LIMIT 1", params.user_id)
        .fetch_one(pool.get_ref())
        .await;

    if let Err(e) = res {
        match e {
            sqlx::Error::RowNotFound => return Err(actix_web::error::ErrorNotFound(e)),
            _ => return Err(actix_web::error::ErrorInternalServerError(e)),
        }
    }

    let res = sqlx::query_as!(Message, "SELECT * FROM messages WHERE user_id = $1 and id = $2 LIMIT 1", params.user_id, params.message_id)
        .fetch_one(pool.get_ref())
        .await;

    let message = match res {
        Err(e) => {
            match e {
                sqlx::Error::RowNotFound => return Err(actix_web::error::ErrorNotFound(e)),
                _ => return Err(actix_web::error::ErrorInternalServerError(e)),
            }
        }
        Ok(m) => m,
    };
    let count = session.get::<i32>("counter")?.unwrap();
    Ok(format!("id: {}, counter: {}, user_id: {}, created_at: {}, content: {}", message.id, count, message.user_id, message.created_at, message.content))
}

fn get_secret_key() -> cookie::Key {
    cookie::Key::from(&[0; 64])
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    let db_url = env::var("DATABASE_URL").unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url).await.unwrap();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let secret_key = get_secret_key();

    HttpServer::new(move|| 
            App::new()
                .wrap(middleware::Logger::default())
                .wrap(
                    SessionMiddleware::new(
                        CookieSessionStore::default(),
                        secret_key.clone()
                    )
                )
                .app_data(web::Data::new(pool.clone()))
                .service(send_message)
                .service(read_message)
        )
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
