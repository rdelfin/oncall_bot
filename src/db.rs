use crate::{
    models::{NewOncallSync, OncallSync},
    schema::oncall_syncs,
};
use diesel::{prelude::*, result::QueryResult, sqlite::SqliteConnection};
use dotenv::dotenv;
use std::env;

no_arg_sql_function!(
    last_insert_rowid,
    diesel::sql_types::Integer,
    "Represents the SQL last_insert_row() function"
);

pub fn connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn add_sync<'a>(
    conn: &SqliteConnection,
    oncall_name: &'a str,
    user_group: &'a str,
) -> QueryResult<OncallSync> {
    let new_oncall_sync = NewOncallSync {
        oncall_name,
        user_group,
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

pub fn get_syncs(conn: &SqliteConnection, oncall_name_q: &str) -> QueryResult<Vec<OncallSync>> {
    use crate::schema::oncall_syncs::dsl::*;
    oncall_syncs
        .filter(oncall_name.eq(oncall_name_q))
        .load::<OncallSync>(conn)
}
