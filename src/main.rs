use email_newsletter::startup::run;
use email_newsletter::configuration::get_configuration;
use actix_web;
use std::net::TcpListener;
use sqlx:: { PgPool };

#[actix_web::main]
async fn main() -> std::io::Result<()>{

    // Read configuration and panic if there is an issue
    let configuration = get_configuration().expect("Failed to read configurations");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to postgres");
    let listener = TcpListener::bind(
        format!("127.0.0.1:{}", configuration.application_port)
    )?;
    
    run(listener, connection_pool)?.await
}