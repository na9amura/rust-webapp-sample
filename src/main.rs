use actix_web::{App, HttpServer, get, web, Result, HttpRequest};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    id: u32,
    name: String,
}

#[get("/users/{user_id}/{friend}")]
async fn friend(path: web::Path<(u32, String)>) -> Result<String> {
    let (user_id, friend) = path.into_inner();
    Ok(format!("Welcome {}, user_id: {}", friend, user_id))
}

#[get("/items/{id}/{name}")]
async fn items(info: web::Path<Info>) -> Result<String> {
    Ok(format!("Item {}, item_id: {}", info.name, info.id))
}

#[get("/unsafe/{user_id}/{friend}")]
async fn unsafe_users(req: HttpRequest) -> Result<String> {
    let name: String = req.match_info().get("friend").unwrap().parse().unwrap();
    let user_id: i32 = req.match_info().query("user_id").parse().unwrap();
    Ok(format!("Welcome {}, user_id: {}", name, user_id))
}

#[derive(Deserialize)]
struct QueryParams {
    username: String,
}

#[get("/users")]
async fn users(info: web::Query<QueryParams>) -> String {
    format!("Finding {} ...", info.username)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| 
            App::new()
                .service(friend)
                .service(items)
                .service(unsafe_users)
                .service(users)
        )
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
