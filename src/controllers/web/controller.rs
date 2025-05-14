use axum::{Json, http::StatusCode};

use crate::models::user_model::{CreateUser, User};

pub async fn home() -> (StatusCode, Json<User>) {
    let user = User::new(1337, "marcuwynu23".to_string());
    (StatusCode::CREATED, Json(user))
}

pub async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User::new(1337, payload.get_username().to_string());

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}
