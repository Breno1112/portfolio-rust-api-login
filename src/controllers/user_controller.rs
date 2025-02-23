use actix_web::http::header::ContentType;
use actix_web::{get, HttpRequest, HttpResponse};

use crate::daos::user::implementation::user_mongodb_impl::UserDAOMongoDB;
use crate::daos::user::user_dao::UserDAO;


#[get("/")]
pub async fn hello(req: HttpRequest) -> HttpResponse {
    let iter = req.headers().iter();
    for x in iter {
        let b = x.1.to_str().ok().unwrap();
        println!("{} = {}", x.0.as_str(), b);
    }
    return HttpResponse::Ok().body("Hello world!");
}

#[get("/users/all")]
pub async fn query_all_users(_: HttpRequest) -> HttpResponse {
    let user_dao = UserDAOMongoDB {};
    let res = user_dao.find_all().await;
    HttpResponse::Ok().insert_header(ContentType::json()).body(serde_json::to_string(&res).unwrap())
}