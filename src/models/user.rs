//! User model module

use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub lastname: String,
    pub firstname: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing)]
    pub deleted_at: Option<DateTime<Utc>>,
}

impl User {
    #[warn(clippy::too_many_arguments)]
    // TODO: Consider grouping some parameters into a new type.
    pub fn init(
        id: String,
        lastname: String,
        firstname: String,
        email: String,
        password: String,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        deleted_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id,
            lastname,
            firstname,
            email,
            password,
            created_at,
            updated_at,
            deleted_at,
        }
    }

    pub fn new(user: UserCreation) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            lastname: user.lastname,
            firstname: user.firstname,
            email: user.email,
            password: user.password,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        }
    }

    pub fn _fullname(&self) -> String {
        let mut fullname = String::new();

        if !self.firstname.is_empty() {
            fullname.push_str(&self.firstname);
        }
        fullname.push(' ');
        fullname.push_str(&self.lastname);

        fullname.trim().to_owned()
    }
}

#[derive(Deserialize, Debug, Validate)]
pub struct Login {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Serialize, Debug, Validate)]
pub struct LoginResponse {
    pub id: String,
    pub lastname: String,
    pub firstname: String,
    #[validate(email)]
    pub email: String,
    pub token: String,
    pub expires_at: String,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct UserCreation {
    pub lastname: String,
    pub firstname: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

#[test]
fn test_fullname() {
    let mut user = User {
        id: Uuid::new_v4().to_string(),
        lastname: String::from("Bellanger"),
        firstname: String::from("Fabien"),
        email: String::from(""),
        password: String::from(""),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        deleted_at: None,
    };
    assert_eq!("Fabien Bellanger", user._fullname());

    user.firstname = String::from("");
    assert_eq!("Bellanger", user._fullname());

    user.firstname = String::from("Fabien");
    user.lastname = String::from("");
    assert_eq!("Fabien", user._fullname());

    user.firstname = String::from("");
    user.lastname = String::from("");
    assert_eq!("", user._fullname());
}
