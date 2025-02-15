use crate::{BcryptService, LoginModel, To, UserModel};
use ::entity::user;
use sea_orm::*;

pub struct UserService;

impl UserService {
    pub async fn register(
        db: &DbConn,
        json_data: UserModel,
    ) -> Result<user::ActiveModel, DbErr> {
        match BcryptService::hash_password(&json_data.password) {
            Ok(hashed_password) => {
                let mut active_model = json_data.to();
                active_model.password = Set(hashed_password);
                active_model.save(db).await
            },
            Err(_) => Err(DbErr::Custom("Failed to hash password".into())),
        }
    }

    pub async fn authenticate(
        db: &DbConn,
        json_data: LoginModel,
    ) -> Result<user::ActiveModel, DbErr> {
        let user_from_db = user::Entity::find()
            .filter(user::Column::Email.eq(json_data.email.clone()))
            .one(db)
            .await?;

        if let Some(user_model) = user_from_db {
            if BcryptService::verify_password(&json_data.password, &user_model.password).unwrap_or(false) {
                Ok(user_model.into())
            } else {
                Err(DbErr::Custom("Invalid password".into()))
            }
        } else {
            Err(DbErr::Custom("User not found".into()))
        }
    }

}
