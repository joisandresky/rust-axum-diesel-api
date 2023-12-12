use crate::{repository::{pg_user_repo::PgUserRepository, user_repo::UserRepostory}, dto::{user_request_dto::UserRequestDto, user_response_dto::UserResponseDto}, model::user::User};


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

    pub fn save(&mut self, dto: UserRequestDto) -> String {
        let user = User::from(&dto);
        
        self.repo.save(user)
    }
}