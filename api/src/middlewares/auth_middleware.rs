use actix_web::{
    body::MessageBody, dev::ServiceRequest, dev::ServiceResponse, Error, HttpResponse, ResponseError,
    http::header::AUTHORIZATION, middleware::Next
};
use serde::Serialize;
use std::fmt::{Display, Formatter};
use service::extract_claims_from_jwt;

pub struct AuthMiddleware;

impl AuthMiddleware {
    pub async fn check(
        req: ServiceRequest,
        next: Next<impl MessageBody>,
    ) -> Result<ServiceResponse<impl MessageBody>, Error> {
        let auth_header = match req.headers().get(AUTHORIZATION) {
            Some(header) => header.to_str().map_err(|_| ErrorResponse::new("Invalid header format"))?,
            None => return Err(ErrorResponse::new("Unauthorized access").into()),
        };

        let token = match auth_header.strip_prefix("Bearer ") {
            Some(t) => t.to_owned(),
            None => return Err(ErrorResponse::new("Unauthorized access").into()),
        };

        let _claims = extract_claims_from_jwt(token)
            .map_err(|_| ErrorResponse::new("Invalid token"))?;

        next.call(req).await.map_err(|_| ErrorResponse::new("Unauthorized access").into())
    }
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

impl ErrorResponse {
    pub fn new(msg: &str) -> Self {
        Self { message: msg.to_string() }
    }
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}

impl ResponseError for ErrorResponse {
    fn error_response(&self) -> HttpResponse {
        match self.message.as_str() {
            "Unauthorized access" => HttpResponse::Unauthorized().json(self),
            "Invalid token" => HttpResponse::BadRequest().json(self),
            "Forbidden access" => HttpResponse::Forbidden().json(self),
            _ => HttpResponse::InternalServerError().json(self),
        }
    }
}
