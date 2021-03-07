//! User model module

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub lastname: String,
    pub firstname: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub created_at: sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,
    pub updated_at: sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,
    pub deleted_at: Option<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>,
}

impl User {
    pub fn _fullname(&self) -> String {
        let mut fullname = String::new();

        if !self.firstname.is_empty() {
            fullname.push_str(&self.firstname);
        }
        fullname.push_str(" ");
        fullname.push_str(&self.lastname);

        fullname.trim().to_owned()
    }
}

#[derive(Deserialize, Debug)]
pub struct Login {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Debug)]
pub struct LoginResponse {
    pub lastname: String,
    pub firstname: String,
    pub email: String,
    pub token: String,
    pub expires_at: String,
}

#[test]
fn test_fullname() {
    let mut user = User {
        id: String::from(""),
        lastname: String::from("Bellanger"),
        firstname: String::from("Fabien"),
        email: String::from(""),
        password: String::from(""),
        created_at: sqlx::types::chrono::Utc::now(),
        updated_at: sqlx::types::chrono::Utc::now(),
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
