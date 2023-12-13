use chrono::NaiveDateTime;
use crate::model::user::User;

pub trait UserRepostory {
    fn find_all(&self) -> Vec<User>;
    fn find_by_id(&self, id: String) -> Option<User>;
    fn find_by_email(&self, user_email: String) -> Option<User>;
    fn save(&self, user: User) -> String;
    fn set_verified_by_id(&self, user: User, verified_time: NaiveDateTime) -> Option<User>;
    fn delete_by_id(&self, user_id: String) -> Option<User>;
}