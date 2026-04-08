use email_newsletter::startup::run;
use email_newsletter::configuration::get_configuration;
use actix_web;
use std::net::TcpListener;
use sqlx:: { Connection, PgConnection }; // Connection has to be brougt to scope.

#[actix_web::main]
async fn main() -> std::io::Result<()>{

    // Read configuration and panic if there is an issue
    let configuration = get_configuration().expect("Failed to read configurations");
    let coonection = PgConnection::connect(&configuration.database.connection_string());
    let listener = TcpListener::bind(
        format!("127.0.0.1:{}", configuration.application_port)
    )?;
    
    run(listener, connection)?.await
}