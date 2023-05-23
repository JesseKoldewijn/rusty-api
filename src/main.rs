use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::Serialize;

#[derive(Serialize)]
struct ResponseMsg {
    msg: String,
}

#[get("/welcome")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Hello there! Thanks for visiting this rusty api ;)")
}

#[get("/hello")]
async fn hello(req_body: String) -> impl Responder {
    let mut return_string: String = "Hello ".to_owned();
    let body_string: &str = req_body.as_str();
    return_string.push_str(body_string);

    HttpResponse::Ok().body("Hello!")
}

#[post("/echo")]
async fn echo(req_body: web::Path<String>) -> Result<impl Responder> {
    if req_body.len() > 0 {
        let res = ResponseMsg {
            msg: req_body.to_string(),
        };

        Ok(web::Json(res))
    } else {
        let res = ResponseMsg {
            msg: "What? No echo? Well send me something then! ;)".to_string(),
        };

        Ok(web::Json(res))
    }
}

#[get("/echo")]
async fn echo_get() -> Result<impl Responder> {
    let res = ResponseMsg {
        msg: "What? No echo? Well send me something then! ;)".to_string(),
    };

    Ok(web::Json(res))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(home)
            .service(hello)
            .service(echo)
            .service(echo_get)
    })
    .bind(("127.0.0.1", 3002))?
    .run()
    .await
}
