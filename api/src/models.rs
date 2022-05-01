use crate::schema::{oncall_syncs, user_mapping};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Clone, Serialize, Deserialize)]
pub struct OncallSync {
    pub id: i32,
    pub oncall_id: String,
    pub user_group_id: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "oncall_syncs"]
pub struct NewOncallSync<'a> {
    pub oncall_id: &'a str,
    pub user_group_id: &'a str,
}

#[derive(Queryable, Clone, Serialize, Deserialize, Debug)]
pub struct UserMapping {
    pub id: i32,
    pub opsgenie_id: String,
    pub slack_id: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "user_mapping"]
pub struct NewUserMapping<'a> {
    pub opsgenie_id: &'a str,
    pub slack_id: &'a str,
}
