use actix_web::{ get, post, delete, put, patch, web, App, HttpResponse, HttpServer, Responder };

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("<h1>This is a tittle</h1><p>And this is a paragraph</p>")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| { App::new().service(index) })
        .bind(("127.0.0.1", 9090))?
        .run()
        .await
}
