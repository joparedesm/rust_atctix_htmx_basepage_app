mod words;
mod pairs;
mod schema;

use actix_files;
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
    // * Send a message to the template
    context.insert("rustacean_message", "Hello from Rust back!!");
    // * Render the template with the context
    let page_content = TEMPLATES.render("index.html", &context).unwrap();
    // * Return the rendered template as the response
    HttpResponse::Ok().body(page_content)
}

#[get("/info")]
async fn info() -> impl Responder {
    // * Create a context to pass to the template
    let context = tera::Context::new();
    // * Render the template with the context
    let page_content = TEMPLATES.render("info.html", &context).unwrap();
    // * Return the rendered template as the response
    HttpResponse::Ok().body(page_content)
}

#[get("/palabras")]
async fn get_words() -> impl Responder {
    // * Get all the words
    let word_list = words::Word::get_words().get_random_words();
    let mut context = tera::Context::new();
    context.insert("word1", &word_list.spanish);
    context.insert("word2", &word_list.english);
    let page_content = TEMPLATES.render("words.html", &context).unwrap();
    HttpResponse::Ok().body(page_content)
}

#[get("/par")]
async fn get_pair() -> impl Responder {
    // * Get all the words
    let word_list = words::Word::get_words().get_random_words();
    let mut context = tera::Context::new();
    context.insert("word1", &word_list.spanish);
    context.insert("word2", &word_list.english);
    let page_content = TEMPLATES.render("pair.html", &context).unwrap();
    HttpResponse::Ok().body(page_content)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // * Start the server with the following routes/services on port 9090
    HttpServer::new(|| {
            App::new()
                .service(index)
                .service(info)
                .service(get_words)
                .service(get_pair)
                .service(
                    actix_files::Files::new("/assets", "./assets")
                )
        })
        .bind(("127.0.0.1", 9090))?
        .run()
        .await
}


