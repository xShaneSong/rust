use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

// This struct represents state
struct AppState {
    app_name: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

#[get("/state")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    format!("Hello {app_name}!") // <- response with app_name
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, World! (manual)")
}

async fn index_html() -> impl Responder {
    "Hello, shane!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                    app_name: String::from("Actix Web"),
                }))
            .service(index) // <- register index handler
            .service(hello)
            .service(echo)
            .route("/manual", web::get().to(manual_hello))
            .service(
                web::scope("/app")
                    .route("/index.html", web::get().to(index_html))
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
    .unwrap();
    Ok(())
}