use bcrypt::{DEFAULT_COST, hash};
use chrono::Local;
use redis::Commands;
use crate::{repository::{pg_user_repo::PgUserRepository, user_repo::UserRepostory}, dto::{user_request_dto::UserRequestDto, user_response_dto::UserResponseDto}, model::user::User};
use crate::custom_error::service_error::ServiceError;


pub struct UserService {
    repo: PgUserRepository,
    redis_conn: redis::Connection,
}

impl UserService {
    pub fn new(repo: PgUserRepository, redis_conn: redis::Connection) -> Self {
        Self { repo, redis_conn }
    }

    pub fn find_all(&self) -> Vec<UserResponseDto> {
        self.repo
            .find_all()
            .iter()
            .map(UserResponseDto::from)
            .collect()
    }

    pub fn find_by_id(&self, user_id: String) -> Option<UserResponseDto> {
        self.repo
            .find_by_id(user_id)
            .map(|user| UserResponseDto::from(&user))
    }

    pub fn save(&mut self, dto: UserRequestDto) -> Result<String, ServiceError> {
        self.repo
            .find_by_email(dto.email.clone())
            .map_or_else(|| {
                // If user with given email is not exist then create user
                let mut user = User::from(&dto);
                user.password = hash(&user.password, DEFAULT_COST).unwrap_or(user.password);

                Ok(self.repo.save(user))
            }, |user| {
                // if user is exist, throw an custom_error
                Err(ServiceError{
                    message: format!("user with given id [{}] is already exist", user.email),
                    status: 422,
                    success: false,
                })
            })
    }

    pub fn set_verified(&mut self, user_id: String) -> Result<Option<UserResponseDto>, ServiceError> {
        self.repo
            .find_by_id(user_id.clone())
            .map_or_else(|| {
                Err(ServiceError{
                    message: format!("User with given id [{}] is not exist", user_id),
                    success: false,
                    status: 404
                })
            },
                |user| {
                    if let Some(_) = user.email_verified_at {
                        return Err(ServiceError{
                            message: format!("User with given id [{}] is already being verified", user_id),
                            success: true,
                            status: 200
                        })
                    }

                    let local_now = Local::now().naive_local();
                    let redis_key = format!("USER_VERIFIED_{}", &user_id);

                    let _: () = self.redis_conn
                        .set(&redis_key, "OK")
                        .unwrap();
                    let _: () = self.redis_conn
                        .expire(&redis_key, 20)
                        .unwrap();

                    Ok(
                        self.repo
                            .set_verified_by_id(user, local_now)
                            .map(|u| UserResponseDto::from(&u))
                    )
                }
            )
    }

    pub fn delete_by_id(&self, user_id: String) -> Option<UserResponseDto> {
        self.repo
            .delete_by_id(user_id)
            .map(|user| UserResponseDto::from(&user))
    }
}