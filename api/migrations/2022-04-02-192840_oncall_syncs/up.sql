CREATE TABLE oncall_syncs (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  oncall_id VARCHAR NOT NULL,
  user_group_id VARCHAR UNIQUE NOT NULL,
  UNIQUE(oncall_id, user_group_id)
);

CREATE TABLE user_mapping (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  opsgenie_id VARCHAR UNIQUE NOT NULL,
  slack_id VARCHAR UNIQUE NOT NULL
);

CREATE TABLE notified_slack_channel (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  oncall_id VARCHAR NOT NULL,
  slack_channel_id VARCHAR UNIQUE NOT NULL,
  UNIQUE(oncall_id, slack_channel_id)
);
