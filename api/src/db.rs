use crate::{
    models::{NewOncallSync, NewUserMapping, OncallSync, UserMapping},
    schema::{oncall_syncs, user_mapping},
};
use diesel::{prelude::*, result::QueryResult, sqlite::SqliteConnection};
use std::env;

no_arg_sql_function!(
    last_insert_rowid,
    diesel::sql_types::Integer,
    "Represents the SQL last_insert_row() function"
);

pub fn connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn add_sync<'a>(
    conn: &SqliteConnection,
    oncall_id: &'a str,
    user_group_id: &'a str,
) -> QueryResult<OncallSync> {
    let new_oncall_sync = NewOncallSync {
        oncall_id,
        user_group_id,
    };

    // Insert and get ID
    diesel::insert_into(oncall_syncs::table)
        .values(&new_oncall_sync)
        .execute(conn)?;
    let generated_id: i32 = diesel::select(last_insert_rowid).first(conn).unwrap();

    {
        use crate::schema::oncall_syncs::dsl::*;
        Ok(oncall_syncs
            .filter(id.eq(generated_id))
            .limit(1)
            .load::<OncallSync>(conn)?
            .first()
            .expect("Item does not exist after insert")
            .clone())
    }
}

pub fn get_syncs(conn: &SqliteConnection, oncall_id_q: &str) -> QueryResult<Vec<OncallSync>> {
    use crate::schema::oncall_syncs::dsl::*;
    oncall_syncs
        .filter(oncall_id.eq(oncall_id_q))
        .load::<OncallSync>(conn)
}

pub fn add_user_mapping<'a>(
    conn: &SqliteConnection,
    opsgenie_id: &'a str,
    slack_id: &'a str,
) -> QueryResult<UserMapping> {
    let new_user_mapping = NewUserMapping {
        opsgenie_id,
        slack_id,
    };

    // Insert and get ID
    diesel::insert_into(user_mapping::table)
        .values(&new_user_mapping)
        .execute(conn)?;
    let generated_id: i32 = diesel::select(last_insert_rowid).first(conn).unwrap();

    {
        use crate::schema::user_mapping::dsl::*;
        Ok(user_mapping
            .filter(id.eq(generated_id))
            .limit(1)
            .load::<UserMapping>(conn)?
            .first()
            .expect("Item does not exist after insert")
            .clone())
    }
}

pub fn list_user_mappings(conn: &SqliteConnection) -> QueryResult<Vec<UserMapping>> {
    use crate::schema::user_mapping::dsl::*;
    user_mapping.load::<UserMapping>(conn)
}

pub fn get_slack_user_mapping(
    conn: &SqliteConnection,
    slack_id_q: &str,
) -> QueryResult<Option<UserMapping>> {
    use crate::schema::user_mapping::dsl::*;
    Ok(user_mapping
        .filter(slack_id.eq(slack_id_q))
        .load::<UserMapping>(conn)?
        .first()
        .map(|um| um.clone()))
}

pub fn get_opsgenie_user_mapping(
    conn: &SqliteConnection,
    opsgenie_id_q: &str,
) -> QueryResult<Option<UserMapping>> {
    use crate::schema::user_mapping::dsl::*;
    Ok(user_mapping
        .filter(opsgenie_id.eq(opsgenie_id_q))
        .load::<UserMapping>(conn)?
        .first()
        .map(|um| um.clone()))
}

pub fn list_oncall_syncs(conn: &SqliteConnection) -> QueryResult<Vec<OncallSync>> {
    use crate::schema::oncall_syncs::dsl::*;
    oncall_syncs.load::<OncallSync>(conn)
}
