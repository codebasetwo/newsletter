use email_newsletter::startup::run;
use email_newsletter::configuration::get_configuration;
use email_newsletter::telemetry::{ init_subscriber, get_subscriber };
use actix_web;
use std::net::TcpListener;
use sqlx:: { PgPool };
use secrecy::ExposeSecret;

#[actix_web::main]
async fn main() -> std::io::Result<()>{

    let subscriber = get_subscriber("email_newsletter".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Read configuration and panic if there is an issue
    let configuration = get_configuration().expect("Failed to read configurations");
    // Only try to establish a connection when the pool is used for the first time.
    let connection_pool = PgPool::connect_lazy(&configuration.database.connection_string().expose_secret())
        .expect("Failed to connect to postgres");
    let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(
        address
    )?;
    
    run(listener, connection_pool)?.await
}