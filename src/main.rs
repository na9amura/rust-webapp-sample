use actix_web::{App, HttpServer, get, HttpRequest, Responder, body::BoxBody, HttpResponse, http::header::ContentType, Error, web, Either};
use serde::Serialize;
use futures::{future::ok, stream::once};

#[derive(Serialize)]
struct MyResponse {
    name: &'static str,
}

impl Responder for MyResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    MyResponse { name: "user" }
}

#[get("/stream")]
async fn stream() -> HttpResponse {
    let body = once(ok::<_, Error>(web::Bytes::from_static(b"test")));

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .streaming(body)
}

type RegisterResult = Either<HttpResponse, Result<&'static str, Error>>;

#[get("/either")]
async fn either(_req: HttpRequest) -> RegisterResult {
    if true {
        Either::Left(HttpResponse::BadRequest().body("Bad Request"))
    } else {
        Either::Right(Ok("Hello!"))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| 
            App::new()
                .service(index)
                .service(stream)
                .service(either)
        )
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
