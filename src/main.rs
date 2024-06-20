use actix_web::{
    get, post, delete, put, patch, web,
    App, HttpResponse, HttpServer, Responder
};

use lazy_static::lazy_static;
use tera::Tera;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        const source = "templates/**/*";
        let mut tera = match Tera::new(source) {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}". e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html", ".sql"]);
        tera.register_filter("do_nothing", do_nothing_filter);
        tera
    };
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("<h1>This is a tittle</h1><h2>This is a smaller heading</h2><p>And this is a paragraph</p>")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| { App::new().service(index) })
        .bind(("127.0.0.1", 9090))?
        .run()
        .await
}
