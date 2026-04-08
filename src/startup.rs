use std::net::TcpListener;
use actix_web::{
    web,
    dev::{ Server },
    App,
    HttpServer,
};
use crate::routes::{ health_check, subscribe, greet };
use sqlx::{ PgPool };



pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    // Wrap pool using web::Data
    let db_pool = web::Data::new(db_pool);
    // move values into closures
    let server = HttpServer::new(move || {
            App::new()
            .service(subscribe)
            .service(health_check)
            .route("/{name}", web::get().to(greet))
            .route("/", web::get().to(greet))
            // Get a pointer copy and attach it to the application state
            .app_data(db_pool.clone())
        })
        .listen(listener)?
        .run();
    Ok(server)
}