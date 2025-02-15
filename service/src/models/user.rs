use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Debug, Validate, Deserialize)]
pub struct UserModel {
    #[validate(length(min = 2, max = 50))]
    #[serde(rename = "firstName")]
    pub first_name: String,

    #[validate(length(min = 2, max = 50))]
    #[serde(rename = "lastName")]
    pub last_name: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8))]
    pub password: String,

    #[validate(custom(function = "validate_birth_date"))]
    #[serde(rename = "birthDate")]
    pub birth_date: NaiveDate,

    #[validate(range(min = 30.0, max = 300.0))]
    pub weight: f64,

    #[validate(range(min = 100.0, max = 250.0))]
    pub height: f64,

    #[validate(custom(function = "validate_gender"))]
    pub gender: Gender,

    #[validate(length(min = 3))]
    #[serde(rename = "healthGoal")]
    pub health_goal: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Gender {
    Male,
    Female,
    Other,
}

#[derive(Debug, Validate, Deserialize)]
pub struct LoginModel {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

impl Gender {
    pub fn from_string_value(value: &str) -> Self {
        match value {
            "male" => Gender::Male,
            "female" => Gender::Female,
            _ => Gender::Other,
        }
    }

    pub fn to_string_value(&self) -> String {
        match self {
            Gender::Male => "male".to_string(),
            Gender::Female => "female".to_string(),
            Gender::Other => "other".to_string(),
        }
    }
}

 fn validate_gender(gender: &Gender) -> Result<(), ValidationError> {
    match gender {
        Gender::Male | Gender::Female | Gender::Other => Ok(()),
    }
}

 fn validate_birth_date(birth_date: &NaiveDate) -> Result<(), ValidationError> {
    let today = chrono::Utc::now().date_naive();
    if *birth_date > today {
        return Err(ValidationError::new("birth_date_in_future"));
    }
    Ok(())
}
