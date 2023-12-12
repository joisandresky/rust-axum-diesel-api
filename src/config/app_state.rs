
use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::Pool;

use crate::{service::user_service::UserService, repository::pg_user_repo::PgUserRepository};


pub struct AppState {
    pub user_service: UserService,
}

impl AppState {
    pub fn new(db_pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        let user_repo = PgUserRepository::new(db_pool);

        Self {
            user_service: UserService::new(user_repo),
        }
    }
}