use std::net::TcpListener;
use actix_web::{
    web,
    dev::{ Server },
    App,
    HttpServer,
};
use crate::routes::{ health_check, subscribe, greet };
use sqlx::{ PgPool };
use tracing_actix_web::TracingLogger;
use crate::email_client::EmailClient;



pub fn run(
    listener: TcpListener, 
    db_pool: PgPool,
    email_client: EmailClient
) -> Result<Server, std::io::Error> {
    // Wrap pool using web::Data
    let db_pool = web::Data::new(db_pool);
    let email_client = web::Data::new(email_client);
    // move values into closures
    let server = HttpServer::new(move || {
            App::new()
            .wrap(TracingLogger::default())
            .service(subscribe)
            .service(health_check)
            .route("/{name}", web::get().to(greet))
            .route("/", web::get().to(greet))
            // Get a pointer copy and attach it to the application state
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
            
        })
        .listen(listener)?
        .run();
    Ok(server)
}