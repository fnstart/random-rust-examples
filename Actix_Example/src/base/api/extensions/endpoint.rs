use crate::base::database::entities::user::{self, ActiveModel};
use actix_web::{web, HttpResponse};
use actix_web_validator::Json;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 3, max = 32))]
    pub username: String,
    #[validate(email)]
    pub email: String,
}

pub struct UserEndpoint;

impl UserEndpoint {
    pub async fn list(db: web::Data<DatabaseConnection>) -> HttpResponse {
        match user::Entity::find().all(db.get_ref()).await {
            Ok(users) => HttpResponse::Ok().json(users),
            Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
        }
    }

    pub async fn create(
        db: web::Data<DatabaseConnection>,
        body: Json<CreateUserRequest>,
    ) -> HttpResponse {
        let new_user = ActiveModel {
            username: Set(ammonia::clean(&body.username)),
            email: Set(body.email.clone()),
            ..Default::default()
        };
        match new_user.insert(db.get_ref()).await {
            Ok(user) => HttpResponse::Ok().json(user),
            Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
        }
    }
}
