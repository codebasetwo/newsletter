use std::net::TcpListener;
use actix_web::{
    web,
    dev::{ Server },
    App,
    HttpServer,
};
use crate::routes::{ health_check, subscribe, greet };



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