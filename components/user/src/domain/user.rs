//! User model module

use validator::{Validate};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Uuid,
    pub lastname: String,
    pub firstname: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    #[serde(skip_serializing)]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

impl User {
    pub fn init(
        id: Uuid,
        lastname: String,
        firstname: String,
        email: String,
        password: String,
        created_at: chrono::NaiveDateTime,
        updated_at: chrono::NaiveDateTime,
        deleted_at: Option<chrono::NaiveDateTime>,
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
        User {
            id: Uuid::new_v4(),
            lastname: user.lastname,
            firstname: user.firstname,
            email: user.email,
            password: user.password,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
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

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct UpdateUserModel {
    pub lastname: String,
    pub firstname: String,
}

#[test]
fn test_fullname() {
    let mut user = User {
        id: Uuid::new_v4(),
        lastname: String::from("Chung"),
        firstname: String::from("Thang"),
        email: String::from(""),
        password: String::from(""),
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
        deleted_at: None,
    };
    assert_eq!("Thang Chung", user._fullname());

    user.firstname = String::from("");
    assert_eq!("Thang", user._fullname());

    user.firstname = String::from("Thang");
    user.lastname = String::from("");
    assert_eq!("Thang", user._fullname());

    user.firstname = String::from("");
    user.lastname = String::from("");
    assert_eq!("", user._fullname());
}
