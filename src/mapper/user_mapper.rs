use crate::{model::user::User, dto::{user_request_dto::UserRequestDto, user_response_dto::UserResponseDto}};


impl From<&UserRequestDto> for User {
    fn from(value: &UserRequestDto) -> Self {
        User::new(value.email.to_string(), value.password.to_string())
    }
}

impl From<&User> for UserResponseDto {
    fn from(value: &User) -> Self {
        UserResponseDto {
            id: value.id.to_string(),
            email: value.email.to_string(),
            email_verified_at: value.email_verified_at,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}