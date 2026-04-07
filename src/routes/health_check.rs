use actix_web:: { get, HttpResponse, HttpRequest, Responder };

#[get("/health_check")]
pub async fn health_check() -> HttpResponse  {
    HttpResponse::Ok().finish()
}

pub async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}\n", &name)
}