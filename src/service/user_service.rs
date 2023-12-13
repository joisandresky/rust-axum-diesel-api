use chrono::Local;
use crate::{repository::{pg_user_repo::PgUserRepository, user_repo::UserRepostory}, dto::{user_request_dto::UserRequestDto, user_response_dto::UserResponseDto}, model::user::User};
use crate::custom_error::service_error::ServiceError;


pub struct UserService {
    repo: PgUserRepository,
}

impl UserService {
    pub fn new(repo: PgUserRepository) -> Self {
        Self { repo }
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
                let user = User::from(&dto);

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

    pub fn set_verified(&self, user_id: String) -> Result<Option<UserResponseDto>, ServiceError> {
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
                    Ok(
                        self.repo
                            .set_verified_by_id(user, local_now)
                            .map(|u| UserResponseDto::from(&u))
                    )
                    // match user.email_verified_at {
                    //     Some(_) => Err(ServiceError{
                    //         message: format!("User with given id [{}] is already being verified", user_id),
                    //         success: true,
                    //         status: 200
                    //     }),
                    //     None => {
                    //
                    //     }
                    // }
                }
            )

    }

    pub fn delete_by_id(&self, user_id: String) -> Option<UserResponseDto> {
        self.repo
            .delete_by_id(user_id)
            .map(|user| UserResponseDto::from(&user))
    }
}