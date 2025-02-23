mod controllers;
mod daos;
mod domain;

use actix_web::{App, HttpServer};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(controllers::user_controller::hello)
            .service(controllers::user_controller::query_all_users)
            .service(controllers::user_controller::query_user_by_id)
            .service(controllers::user_controller::create_user)
            .service(controllers::user_controller::delete_user)
            .service(controllers::user_controller::update_user)
            .service(controllers::user_controller::batch_create_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}