use std::sync::Mutex;
use actix_web::{get, web, App, HttpServer, HttpResponse, Responder};

struct AppState {
    app_name: String,
    counter: Mutex<i32>,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {app_name}!")
}

#[get("/count")]
async fn count(data: web::Data<AppState>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("Request number: {counter}")
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(index)
       .service(count);
}

async fn app_index() -> impl Responder {
    "Hello world!"
}

fn app_config(cfg: &mut web::ServiceConfig) {
    cfg.route("/index.html", web::get().to(app_index));
}

fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/api")
            .route(web::get().to(|| async { HttpResponse::Ok().body("API") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState {
        app_name: String::from("Actix Web"),
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .configure(config)
            .service(
                web::scope("/app").configure(app_config)
            )
            .configure(api_config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
