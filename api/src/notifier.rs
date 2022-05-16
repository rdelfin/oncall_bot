use crate::{db, opsgenie, slack};
use log::{info, warn};
use std::time::Duration;
use tokio::{
    join,
    sync::oneshot::{self, error::TryRecvError, Receiver, Sender},
    time::sleep,
};

const TOPIC_PREFIX: &str = "Current oncall: ";
const TOPIC_SEPARATOR: &str = " | ";

#[derive(Debug)]
pub struct SlackNotifier {
    oncall_id: String,
    slack_channel_id: String,
    stop_tx: Option<Sender<()>>,
}

impl SlackNotifier {
    pub fn new(oncall_id: String, slack_channel_id: String) -> SlackNotifier {
        let (stop_tx, stop_rx) = oneshot::channel();
        let oncall_id_clone = oncall_id.clone();
        let slack_channel_id_clone = slack_channel_id.clone();
        tokio::spawn(async move {
            slack_notifier(oncall_id_clone, slack_channel_id_clone, stop_rx).await
        });
        SlackNotifier {
            stop_tx: Some(stop_tx),
            slack_channel_id,
            oncall_id,
        }
    }
}

impl Drop for SlackNotifier {
    fn drop(&mut self) {
        // Extract out `stop_tx` (which should never be none)
        match std::mem::take(&mut self.stop_tx) {
            Some(stop_tx) => {
                // Sending a stop will be sufficient to stop the oncall syncer on the next iteration
                if let Err(_) = stop_tx.send(()) {
                    warn!("Slack notifier for oncall ID \"{}\" and slack channel ID \"{}\" failed to send stop. It's likely something went wrong with the syncer task.", self.oncall_id, self.slack_channel_id);
                }
            }
            None => {
                warn!("self.stop_tx for slack notifier with oncall ID \"{}\" and slack channel ID \"{}\" was none. Did you double-drop?", self.oncall_id, self.slack_channel_id);
            }
        }
    }
}

async fn slack_notifier(oncall_id: String, slack_channel_id: String, mut stop_rx: Receiver<()>) {
    let sleep_time = Duration::from_secs(60);
    let mut first_iter = true;

    loop {
        // While putting the sleep at the end gets rid of this if, putting it here allows us to use
        // continues to break flow cleanly and avoid relentless retries if there's an issue
        if first_iter {
            first_iter = false;
        } else {
            sleep(sleep_time).await;
        }

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
            "Checking notification for oncall_id {} and slack_channel_id {}",
            oncall_id, slack_channel_id
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

        // Generate @ section of topic
        let users_string = slack_users.iter().fold("".to_string(), |full, user_id| {
            let user_at = format!("<@{}>", user_id);
            if full == "" {
                user_at
            } else {
                format!("{} {}", full, user_at)
            }
        });
        let users_string = if users_string == "" {
            "nobody".to_string()
        } else {
            users_string
        };

        // Check the channel's topic to see if it needs updating
        let channel = match slack::get_channel(&slack_channel_id).await {
            Err(e) => {
                warn!("Error fetching slack channel {}: {}", slack_channel_id, e);
                continue;
            }
            Ok(c) => c,
        };
        let mut needs_update = true;
        let mut has_topic = false;
        let new_topic = channel
            .topic
            .value
            .split(TOPIC_SEPARATOR)
            .map(|element| {
                if element.starts_with(TOPIC_PREFIX) {
                    has_topic = true;
                    needs_update = element[TOPIC_PREFIX.len()..] != users_string;
                    format!("{}{}", TOPIC_PREFIX, users_string)
                } else {
                    element.to_string()
                }
            })
            .fold("".to_string(), |full, element| {
                if full == "" {
                    element.to_string()
                } else {
                    format!("{}{}{}", full, TOPIC_SEPARATOR, element)
                }
            });
        let new_topic = if has_topic {
            new_topic
        } else if new_topic == "" {
            format!("{}{}", TOPIC_PREFIX, users_string)
        } else {
            format!(
                "{}{}{}{}",
                new_topic, TOPIC_SEPARATOR, TOPIC_PREFIX, users_string
            )
        };

        // Finally, if needed, update the slack channel topic and send a message.
        if needs_update {
            let posted_message = if slack_users.is_empty() {
                format!("This channel's oncall is out of hours. Please wait for the next oncall for urgent requests.")
            } else {
                format!(
                    "There's a new oncall! Please direct all questions to {}",
                    users_string
                )
            };
            let (post_result, topic_result) = join!(
                slack::post_message(&slack_channel_id, &posted_message),
                slack::set_channel_topic(&slack_channel_id, &new_topic)
            );

            if let Err(e) = post_result {
                warn!(
                    "Failed to send message to channel {}: {}",
                    &slack_channel_id, e
                );
            }
            if let Err(e) = topic_result {
                warn!(
                    "Failed to update topic on channel {}: {}",
                    &slack_channel_id, e
                );
            }
        }
    }
}
