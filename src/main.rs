use email_newsletter::run;
use actix_web;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    // Using port 0 binds to a random free port.
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    run(listener)?.await
}