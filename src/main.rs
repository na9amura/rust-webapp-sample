use actix_web::{App, HttpServer, post, web, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct Post {
    user_id: u32,
    content: String,
}

#[post("/send_message")]
async fn send_message(data: web::Json<Post>) -> Result<String> {
    Ok(format!("{}: {}", data.user_id, data.content))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| 
            App::new()
                .service(send_message)
        )
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
