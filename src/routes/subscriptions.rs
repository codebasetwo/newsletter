use actix_web::{ web, post, HttpResponse };

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String
}


#[post("/subscriptions")]
async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}