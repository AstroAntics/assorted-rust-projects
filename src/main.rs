use actix_web::{
    get,
    post,
    web,
    App,
    HttpResponse,
    HttpServer,
    Responder
};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
             // ^ <https://docs.rs/actix-web/latest/src/actix_web/app.rs.html#257-276>
            .default_service(web::to(||HttpResponse::NotFound()))
            .route("/hey", web::get().to(manual_hello))
    }).bind(("127.0.0.1", 8080))?.run().await
}