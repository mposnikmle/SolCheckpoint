use crate::{models::user_model::User, repository::user_repo::MongoRepo};
use actix_web::{
    post, get, put, delete,
    web::{Data, Json, Path},
    HttpResponse,
};
use mongodb::bson::oid::ObjectId;

#[post("/user")]
pub async fn create_user(db: Data<MongoRepo>, new_user: Json<User>) -> HttpResponse {
    let hashed_password = argon2::hash_encoded(new_user.password.as_bytes(), b"randomsalt", &argon2::Config::default());

    // Check if hashing was successful
    let hashed_password = match hashed_password {
        Ok(hash) => hash,
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };

    let data = User {
        id: None,
        email: new_user.email.to_owned(),
        password: hashed_password.to_owned(),
        username: new_user.username.to_owned(),
    };
    let user_detail = db.create_user(data).await;
    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/user/{id}")]
pub async fn get_user(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    let user_detail = db.get_user(&id).await;
    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

