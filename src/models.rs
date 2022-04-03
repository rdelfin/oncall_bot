use crate::schema::oncall_syncs;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Clone, Serialize, Deserialize)]
pub struct OncallSync {
    pub id: i32,
    pub oncall_id: String,
    pub user_group: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "oncall_syncs"]
pub struct NewOncallSync<'a> {
    pub oncall_id: &'a str,
    pub user_group: &'a str,
}
