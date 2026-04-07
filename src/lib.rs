use actix_web::{ 
    dev:: { Server },
    get, web, App, HttpRequest, HttpResponse,HttpServer, Responder 
};
use std::net::TcpListener;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}\n", &name)
}

#[get("/health_check")]
async fn health_check() -> HttpResponse  {
    HttpResponse::Ok().finish()
}

#[post("/subscriptions")]
async fn subscribe() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
            App::new()
            .service(subscribe)
            .service(health_check)
            .route("/{name}", web::get().to(greet))
            .route("/", web::get().to(greet))
        })
        .listen(listener)?
        .run();
    Ok(server)
}