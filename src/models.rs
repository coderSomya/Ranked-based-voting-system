use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Queryable, Serialize)]
pub struct User{
    pub id: Uuid,
    pub username: String,
}

#[derive(Queryable, Serialize)]
pub struct Option{
    pub id: Uuid,
    pub question_id: Uuid,
    pub option_text: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "users"]
pub struct NewUser{
    pub username: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "votes"]
pub struct NewVote{
    pub user_id: Uuid,
    pub option_id: Uuid,
    pub rank: i32
}