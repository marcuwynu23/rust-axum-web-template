use serde::{Deserialize, Serialize};

// the input to our `create_user` handler
#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
}

impl CreateUser {
    pub fn new(username: String) -> Self {
        Self { username }
    }
    pub fn get_username(&self) -> &str {
        &self.username
    }
}

// the output to our `create_user` handler
#[derive(Serialize)]
pub struct User {
    id: u64,
    username: String,
}

impl User {
    pub fn new(id: u64, username: String) -> Self {
        Self { id, username }
    }
}
