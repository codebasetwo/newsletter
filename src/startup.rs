use std::net::TcpListener;
use actix_web::{
    web,
    dev::{ Server },
    App,
    HttpServer,
};
use crate::routes::{ health_check, subscribe, greet };
use sqlx::{ PgConnection };



pub fn run(listener: TcpListener, connection: PgConnection) -> Result<Server, std::io::Error> {
    // PgConnection has to be cloneable so we wrap connection in an actix_web smart pointer.
    // Data is an Arc<T> that can be passed to threads. needed as actix-web will spin up a worker process for each
    // available core on your machine.
    // Shadow the connection.
    let connection = web::Data::new(connection);
    // move values into closures
    let server = HttpServer::new(move || {
            App::new()
            .service(subscribe)
            .service(health_check)
            .route("/{name}", web::get().to(greet))
            .route("/", web::get().to(greet))
            // Get a pointer copy and attach it to the application state
            .app_data(connection.clone())
        })
        .listen(listener)?
        .run();
    Ok(server)
}