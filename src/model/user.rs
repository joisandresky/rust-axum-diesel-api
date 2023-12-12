use crate::schemas::schema::users;
use chrono::{NaiveDateTime, Local};
use diesel::{deserialize::Queryable, Selectable, prelude::Insertable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: String,
    pub email: String,
    pub email_verified_at: Option<NaiveDateTime>,
    pub password: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl User {
    pub fn new(
        email: String,
        password: String,
    ) -> Self {
        let local_now = Local::now().naive_local();
        let id = Uuid::new_v4().to_string();

        Self {
            id, 
            email, 
            password,
            email_verified_at: None,
            created_at: Some(local_now),
            updated_at: None
        }
    }
}