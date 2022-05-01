use crate::{db, opsgenie, slack};
use log::{info, warn};
use std::time::Duration;
use tokio::{
    sync::oneshot::{self, error::TryRecvError, Receiver, Sender},
    time::sleep,
};

#[derive(Debug)]
pub struct Syncer {
    oncall_id: String,
    user_group_id: String,
    stop_tx: Option<Sender<()>>,
}

impl Syncer {
    pub fn new(oncall_id: String, user_group_id: String) -> Syncer {
        let (stop_tx, stop_rx) = oneshot::channel();
        let oncall_id_clone = oncall_id.clone();
        let user_group_id_clone = user_group_id.clone();
        tokio::spawn(
            async move { oncall_sync(oncall_id_clone, user_group_id_clone, stop_rx).await },
        );
        Syncer {
            stop_tx: Some(stop_tx),
            user_group_id,
            oncall_id,
        }
    }
}

impl Drop for Syncer {
    fn drop(&mut self) {
        // Extract out `stop_tx` (which should never be none)
        match std::mem::take(&mut self.stop_tx) {
            Some(stop_tx) => {
                // Sending a stop will be sufficient to stop the oncall syncer on the next iteration
                if let Err(_) = stop_tx.send(()) {
                    warn!("Syncer for oncall ID \"{}\" and user group ID \"{}\" failed to send stop. It's likely something went wrong with the syncer task.", self.oncall_id, self.user_group_id);
                }
            }
            None => {
                warn!("self.stop_tx for syncer with oncall ID \"{}\" and user group ID \"{}\" was none. Did you double-drop?", self.oncall_id, self.user_group_id);
            }
        }
    }
}

async fn oncall_sync(oncall_id: String, user_group_id: String, mut stop_rx: Receiver<()>) {
    let sleep_time = Duration::from_secs(60);

    loop {
        // First, make sure we haven't gotten a stop message
        match stop_rx.try_recv() {
            // We've been asked to stop, or the process above is dead. Stop, so return immediately
            Ok(_) | Err(TryRecvError::Closed) => {
                return;
            }
            // Do nothing if empty
            Err(TryRecvError::Empty) => {}
        };

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

        sleep(sleep_time).await;
    }
}
