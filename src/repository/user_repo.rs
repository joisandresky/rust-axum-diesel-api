use crate::model::user::User;

pub trait UserRepostory {
    fn find_all(&self) -> Vec<User>;

    fn find_by_id(&self, id: String) -> Option<User>;

    fn save(&self, user: User) -> String;
}