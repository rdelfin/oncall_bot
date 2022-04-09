use crate::{db, opsgenie, slack};
use log::{info, warn};
use std::time::Duration;
use tokio::{
    task::{JoinError, JoinHandle},
    time::sleep,
};

const MAX_CONCURRENT_USERMAPS: usize = 10;

#[derive(Debug)]
pub struct Syncer {
    jh: JoinHandle<()>,
}

impl Syncer {
    pub fn new(oncall_id: String, user_group_id: String) -> Syncer {
        Syncer {
            jh: tokio::spawn(async move { oncall_sync(oncall_id, user_group_id).await }),
        }
    }

    pub async fn wait(self) -> Result<(), JoinError> {
        self.jh.await
    }
}

async fn oncall_sync(oncall_id: String, user_group_id: String) {
    loop {
        info!(
            "Updating oncall_id {} and user_group_id {}",
            oncall_id, user_group_id
        );

        let current_oncalls = match opsgenie::get_current_oncalls(&oncall_id).await {
            Err(e) => {
                warn!(
                    "Error fetching current oncall data for {}: {}",
                    oncall_id, e
                );
                continue;
            }
            Ok(oncalls) => oncalls,
        };

        // This filters out any users we don't have a mapping for
        let tasks = current_oncalls.into_iter().map(|opsgenie_user_id| {
            tokio::spawn(async move {
                let connection = db::connection();
                db::get_opsgenie_user_mapping(&connection, &opsgenie_user_id)
            })
        });
        let slack_users = futures::future::join_all(tasks)
            .await
            .into_iter()
            .filter_map(|user_mapping| match user_mapping {
                Err(e) => {
                    warn!("Error fetching user mapping: {}", e);
                    None
                }
                Ok(Err(e)) => {
                    warn!("Error fetching user mapping: {}", e);
                    None
                }
                Ok(Ok(user_mapping)) => user_mapping.map(|user_mapping| user_mapping.slack_id),
            })
            .collect::<Vec<_>>();

        // Finally, update slack's user group with the users that are left
        if let Err(e) = slack::set_user_group(&user_group_id, &slack_users).await {
            warn!("Failed to update user group {}: {}", user_group_id, e);
        }

        sleep(Duration::from_secs(10)).await;
    }
}
