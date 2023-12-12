use diesel::prelude::*;
use diesel::r2d2::{Pool, ConnectionManager};

use crate::repository::user_repo::UserRepostory;
use crate::model::user::User;
use crate::schemas::schema::users::{id, table};
use crate::schemas::schema::users::dsl::users;

#[derive(Clone)]
pub struct PgUserRepository {
    db_pool: Pool<ConnectionManager<PgConnection>>,
}

impl PgUserRepository {
    pub fn new(db_pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { db_pool }
    }
}

impl UserRepostory for PgUserRepository {
    fn find_all(&self) -> Vec<User> {
        let db_conn = &mut self.db_pool.get().unwrap();

        users.load(db_conn).unwrap()
    }

    fn find_by_id(&self, user_id: String) -> Option<User> {
        let db_conn = &mut self.db_pool.get().unwrap();
        users
            .filter(id.eq(user_id))
            .first::<User>(db_conn)
            .ok()
    }

    fn save(&self, user: User) -> String {
        let db_conn = &mut self.db_pool.get().unwrap();
        
        let new_user = User::new(user.email, user.password);

        diesel::insert_into(table)
            .values(&new_user)
            .returning(User::as_returning())
            .get_result(db_conn)
            .expect("Error Saving new User");

        new_user.id
    }
}