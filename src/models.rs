use super::schema::*;
use std::time::SystemTime;
#[derive(Queryable, Insertable, PartialEq, Debug)]
#[table_name = "clc4"]
pub struct Clc4 {
    pub id: i64,
    pub headword: String,
    pub dict_form: String,
    pub meaning: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Queryable, Insertable, PartialEq, Debug)]
#[table_name = "lewis_short_lemmata"]
pub struct Lemma {
    pub id: i64,
    pub form: String,
    pub analysis: String,
}
