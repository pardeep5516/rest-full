use crate::models::user::User;
use axum::Json;

pub async fn index() -> &'static str {
    "welcome to axum dev server and tokio async"
}

pub async fn create_user() -> Json<User> {
    let user: User = User {
        id: 1,
        name: "John".to_string(),
        email: "xyz@gmail".to_string(),
        password: "123".to_string(),
    };
    Json(user)
}

//to run the server
//cargo watch -q -c src -x run
