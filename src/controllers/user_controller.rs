use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};

use crate::daos::user::implementation::user_mongodb_impl::UserDAOMongoDB;
use crate::daos::user::user_dao::{User, UserDAO};
use crate::domain::dtos::user_dto::UserDTO;


#[get("/")]
pub async fn hello(_: HttpRequest) -> HttpResponse {
    return HttpResponse::Ok().body("Hello world!");
}

#[get("/users")]
pub async fn query_all_users(_: HttpRequest) -> HttpResponse {
    let user_dao = UserDAOMongoDB {};
    let res = user_dao.find_all().await;
    HttpResponse::Ok().insert_header(ContentType::json()).body(serde_json::to_string(&res).unwrap())
}

#[get("/users/{id}")]
pub async fn query_user_by_id(id: web::Path<String>) -> HttpResponse {
    let user_dao = UserDAOMongoDB {};
    let res = user_dao.find_by_id(&id).await;
    if res.is_none() {
        return HttpResponse::new(StatusCode::NOT_FOUND);
    }
    return HttpResponse::Ok().insert_header(ContentType::json()).body(serde_json::to_string(&res.unwrap()).unwrap());
}

#[post("/users")]
pub async fn create_user(request_body: web::Json<UserDTO>) -> HttpResponse {
    let user_dao = UserDAOMongoDB {};
    let user = User::new(String::from("null"), String::from(&request_body.name), request_body.age, true);
    let res = user_dao.insert_one(&user).await;
    if res {
        return HttpResponse::new(StatusCode::CREATED);
    }
    return HttpResponse::new(StatusCode::UNPROCESSABLE_ENTITY);
}

#[delete("/users/{id}")]
pub async fn delete_user(id: web::Path<String>) -> HttpResponse {
    let user_dao = UserDAOMongoDB {};
    let res = user_dao.delete(&id).await;
    if res {
        return HttpResponse::new(StatusCode::OK);
    }
    return HttpResponse::new(StatusCode::UNPROCESSABLE_ENTITY);
}

#[put("/users/{id}")]
pub async fn update_user(request_body: web::Json<UserDTO>, id: web::Path<String>) -> HttpResponse {
    let user_dao = UserDAOMongoDB {};
    let user_response = user_dao.find_by_id(&id).await;
    if user_response.is_none() {
        return HttpResponse::new(StatusCode::UNPROCESSABLE_ENTITY);
    }
    let mut user = user_response.unwrap();
    user.name = String::from(&request_body.name);
    user.age = request_body.age;
    let res = user_dao.update(&user).await;
    if res {
        return HttpResponse::new(StatusCode::OK);
    }
    return HttpResponse::new(StatusCode::UNPROCESSABLE_ENTITY);
}

#[post("/users/batch")]
pub async fn batch_create_user(request_body: web::Json<Vec<UserDTO>>) -> HttpResponse {
    let user_dao = UserDAOMongoDB {};
    let mut users: Vec<User> = Vec::new();
    for u in request_body.iter() {
        users.push(User {id: String::new(), name: String::from(&u.name), age: u.age, active: true});
    }
    let res = user_dao.insert_many(&users).await;
    if res {
        return HttpResponse::new(StatusCode::CREATED);
    }
    return HttpResponse::new(StatusCode::UNPROCESSABLE_ENTITY);
}