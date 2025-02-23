use actix_web::{get, HttpRequest, HttpResponse};


#[get("/")]
pub async fn hello(req: HttpRequest) -> HttpResponse {
    let iter = req.headers().iter();
    for x in iter {
        let b = x.1.to_str().ok().unwrap();
        println!("{} = {}", x.0.as_str(), b);
    }
    HttpResponse::Ok().body("Hello world!")
}