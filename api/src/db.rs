use crate::{
    models::{
        NewNotifiedSlackChannel, NewOncallSync, NewUserMapping, NotifiedSlackChannel, OncallSync,
        UserMapping,
    },
    schema::{notified_slack_channel, oncall_syncs, user_mapping},
    ErrorResponse,
};
use actix_web::HttpResponse;
use diesel::{prelude::*, result::Error as DieselError, sqlite::SqliteConnection};
use std::env;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("error making a query")]
    QueryError(#[from] DieselError),
    #[error("oncall sync for oncall {oncall_id} and user group {user_group_id} already exists")]
    OncallSyncAlreadyExists {
        oncall_id: String,
        user_group_id: String,
    },
    #[error("user mapping for opsgenie user ID {opsgenie_id} and slack user ID {slack_id} already exists")]
    UserMappingAlreadyExists {
        opsgenie_id: String,
        slack_id: String,
    },
    #[error("channel with ID {0} is already being notified")]
    ChannelAlreadyNotified(String),
    #[error("Oncall sync with ID {0} does not exist")]
    OncallSyncDoesNotExist(i32),
    #[error("user mapping with ID {0} does not exist")]
    UserMappingDoesNotExist(i32),
    #[error("channel notification ID {0} does not exist")]
    ChannelNotificationDoesNotExist(i32),
}

impl From<Error> for HttpResponse {
    fn from(error: Error) -> HttpResponse {
        match error {
            Error::QueryError(_) => HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("{}", error),
            }),
            Error::OncallSyncAlreadyExists {
                oncall_id: _,
                user_group_id: _,
            }
            | Error::UserMappingAlreadyExists {
                opsgenie_id: _,
                slack_id: _,
            }
            | Error::UserMappingDoesNotExist(_)
            | Error::OncallSyncDoesNotExist(_)
            | Error::ChannelAlreadyNotified(_)
            | Error::ChannelNotificationDoesNotExist(_) => {
                HttpResponse::BadRequest().json(ErrorResponse {
                    error: format!("{}", error),
                })
            }
        }
    }
}

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

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
    oncall_id_q: &'a str,
    user_group_id_q: &'a str,
) -> Result<OncallSync> {
    conn.transaction(|| {
        // If sync already exists, error out
        {
            use crate::schema::oncall_syncs::dsl::*;
            if let Some(_) = oncall_syncs
                .filter(oncall_id.eq(oncall_id_q))
                .filter(user_group_id.eq(user_group_id_q))
                .limit(1)
                .load::<OncallSync>(conn)?
                .first()
            {
                return Err(Error::OncallSyncAlreadyExists {
                    oncall_id: oncall_id_q.into(),
                    user_group_id: user_group_id_q.into(),
                });
            }
        }

        let new_oncall_sync = NewOncallSync {
            oncall_id: oncall_id_q,
            user_group_id: user_group_id_q,
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
    })
}

pub fn remove_sync(conn: &SqliteConnection, id_q: i32) -> Result<OncallSync> {
    use crate::schema::oncall_syncs::dsl::*;

    let removed_sync = oncall_syncs
        .filter(id.eq(id_q))
        .limit(1)
        .load::<OncallSync>(conn)?
        .first()
        .ok_or_else(|| Error::OncallSyncDoesNotExist(id_q))?
        .clone();

    diesel::delete(oncall_syncs.filter(id.eq(id_q))).execute(conn)?;

    Ok(removed_sync)
}

pub fn get_syncs(conn: &SqliteConnection, oncall_id_q: &str) -> Result<Vec<OncallSync>> {
    use crate::schema::oncall_syncs::dsl::*;
    Ok(oncall_syncs
        .filter(oncall_id.eq(oncall_id_q))
        .load::<OncallSync>(conn)?)
}

pub fn add_user_mapping<'a>(
    conn: &SqliteConnection,
    opsgenie_id_q: &'a str,
    slack_id_q: &'a str,
) -> Result<UserMapping> {
    conn.transaction(|| {
        // Ensure user mapping doesn't already exist
        {
            use crate::schema::user_mapping::dsl::*;
            if let Some(_) = user_mapping
                .filter(opsgenie_id.eq(opsgenie_id_q))
                .filter(slack_id.eq(slack_id_q))
                .limit(1)
                .load::<UserMapping>(conn)?
                .first()
            {
                // If sync already exists, error out
                return Err(Error::UserMappingAlreadyExists {
                    opsgenie_id: opsgenie_id_q.into(),
                    slack_id: slack_id_q.into(),
                });
            }
        }

        let new_user_mapping = NewUserMapping {
            opsgenie_id: opsgenie_id_q,
            slack_id: slack_id_q,
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
    })
}

pub fn remove_user_mapping<'a>(conn: &SqliteConnection, id_q: i32) -> Result<UserMapping> {
    use crate::schema::user_mapping::dsl::*;

    let removed_user_mapping = user_mapping
        .filter(id.eq(id_q))
        .load::<UserMapping>(conn)?
        .first()
        .ok_or_else(|| Error::UserMappingDoesNotExist(id_q))?
        .clone();

    diesel::delete(user_mapping.filter(id.eq(id_q))).execute(conn)?;

    Ok(removed_user_mapping)
}

pub fn list_user_mappings(conn: &SqliteConnection) -> Result<Vec<UserMapping>> {
    use crate::schema::user_mapping::dsl::*;
    Ok(user_mapping.load::<UserMapping>(conn)?)
}

pub fn get_slack_user_mapping(
    conn: &SqliteConnection,
    slack_id_q: &str,
) -> Result<Option<UserMapping>> {
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
) -> Result<Option<UserMapping>> {
    use crate::schema::user_mapping::dsl::*;
    Ok(user_mapping
        .filter(opsgenie_id.eq(opsgenie_id_q))
        .load::<UserMapping>(conn)?
        .first()
        .map(|um| um.clone()))
}

pub fn list_oncall_syncs(conn: &SqliteConnection) -> Result<Vec<OncallSync>> {
    use crate::schema::oncall_syncs::dsl::*;
    Ok(oncall_syncs.load::<OncallSync>(conn)?)
}

pub fn list_notified_slack_channels(conn: &SqliteConnection) -> Result<Vec<NotifiedSlackChannel>> {
    use crate::schema::notified_slack_channel::dsl::*;
    Ok(notified_slack_channel.load::<NotifiedSlackChannel>(conn)?)
}

pub fn get_channels_notified_for_oncall(
    conn: &SqliteConnection,
    oncall_id_q: &str,
) -> Result<Vec<NotifiedSlackChannel>> {
    use crate::schema::notified_slack_channel::dsl::*;
    Ok(notified_slack_channel
        .filter(oncall_id.eq(oncall_id_q))
        .load::<NotifiedSlackChannel>(conn)?)
}

pub fn get_oncall_notified_in_channel(
    conn: &SqliteConnection,
    channel_id: &str,
) -> Result<Option<NotifiedSlackChannel>> {
    use crate::schema::notified_slack_channel::dsl::*;
    Ok(notified_slack_channel
        .limit(1)
        .filter(slack_channel_id.eq(channel_id))
        .load::<NotifiedSlackChannel>(conn)?
        .first()
        .cloned())
}

pub fn add_channel_oncall_notification(
    conn: &SqliteConnection,
    slack_channel_id_q: &str,
    oncall_id_q: &str,
) -> Result<NotifiedSlackChannel> {
    conn.transaction(|| {
        // First, confirm the channel's not already been mapped
        {
            use crate::schema::notified_slack_channel::dsl::*;
            if let Some(_) = notified_slack_channel
                .limit(1)
                .filter(slack_channel_id.eq(slack_channel_id_q))
                .load::<NotifiedSlackChannel>(conn)?
                .first()
            {
                return Err(Error::ChannelAlreadyNotified(slack_channel_id_q.into()));
            }
        }

        let new_notified_slack_channel = NewNotifiedSlackChannel {
            slack_channel_id: slack_channel_id_q,
            oncall_id: oncall_id_q,
        };

        // Insert and get ID
        diesel::insert_into(notified_slack_channel::table)
            .values(&new_notified_slack_channel)
            .execute(conn)?;

        let generated_id: i32 = diesel::select(last_insert_rowid).first(conn).unwrap();

        {
            use crate::schema::notified_slack_channel::dsl::*;
            Ok(notified_slack_channel
                .filter(id.eq(generated_id))
                .limit(1)
                .load::<NotifiedSlackChannel>(conn)?
                .first()
                .expect("Item does not exist after insert")
                .clone())
        }
    })
}

pub fn remove_channel_oncall_notification(
    conn: &SqliteConnection,
    id_q: i32,
) -> Result<NotifiedSlackChannel> {
    conn.transaction(|| {
        // First, confirm the mapping exists
        use crate::schema::notified_slack_channel::dsl::*;
        let deleted_notification = match notified_slack_channel
            .limit(1)
            .filter(id.eq(id_q))
            .load::<NotifiedSlackChannel>(conn)?
            .first()
            .cloned()
        {
            Some(notification) => notification,
            None => {
                return Err(Error::ChannelNotificationDoesNotExist(id_q));
            }
        };

        diesel::delete(notified_slack_channel.filter(id.eq(id_q))).execute(conn)?;

        Ok(deleted_notification)
    })
}
