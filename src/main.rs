use actix_web::{
    get, post, delete, put, patch, web,
    App, HttpResponse, HttpServer, Responder
};

use lazy_static::lazy_static;
use tera::Tera;

const SOURCE: &str = "templates/**/*";
// * Initialize the Tera template engine
lazy_static! {
    // * Create a static reference to the Tera instance
    pub static ref TEMPLATES: Tera = {
        let tera = Tera::new(SOURCE).expect("Error parsing templates / initializing Tera");
        tera
    };
}

#[get("/")]
async fn index() -> impl Responder {
    // * Create a context to pass to the template
    let mut context = tera::Context::new();
    // * Add a key-value pair to the context
    context.insert("rustacean_message", "Hello from Rust back!!");
    // * Render the template with the context
    let page_content = TEMPLATES.render("index.html", &context).unwrap();
    // * Return the rendered template as the response
    HttpResponse::Ok().body(page_content)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // * Start the server with the following routes/services
    HttpServer::new(|| {
            App::new()
                .service(index)
        })
        .bind(("127.0.0.1", 9090))?
        .run()
        .await
}


