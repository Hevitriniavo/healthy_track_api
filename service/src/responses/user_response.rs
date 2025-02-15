use crate::models::user::UserModel;
use entity::user;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use sea_orm::Set;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub birth_date: chrono::NaiveDate,
    pub weight: Decimal,
    pub height: Decimal,
    pub gender: String,
    pub health_goal: String,
}

impl From<user::ActiveModel> for UserResponse {
    fn from(user: user::ActiveModel) -> Self {
        UserResponse {
            id: user.user_id.unwrap(),
            first_name: user.first_name.unwrap(),
            last_name: user.last_name.unwrap(),
            email: user.email.unwrap(),
            birth_date: user.birth_date.unwrap(),
            weight: user.weight.unwrap(),
            height: user.height.unwrap(),
            gender: user.gender.unwrap(),
            health_goal: user.health_goal.unwrap(),
        }
    }
}

pub trait To {
    fn to(self) -> user::ActiveModel;
}

impl To for UserModel {
    fn to(self) -> user::ActiveModel {
        user::ActiveModel {
            first_name: Set(self.first_name),
            last_name: Set(self.last_name),
            email: Set(self.email),
            password: Set(self.password),
            birth_date: Set(self.birth_date),
            weight: Set(Decimal::from_f64(self.weight).unwrap_or_default()),
            height: Set(Decimal::from_f64(self.height).unwrap_or_default()),
            gender: Set(self.gender.to_string_value()),
            health_goal: Set(self.health_goal),
            ..Default::default()
        }
    }
}
