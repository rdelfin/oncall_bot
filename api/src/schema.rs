table! {
    oncall_syncs (id) {
        id -> Integer,
        oncall_id -> Text,
        user_group_id -> Text,
    }
}

table! {
    user_mapping (id) {
        id -> Integer,
        opsgenie_id -> Text,
        slack_id -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    oncall_syncs,
    user_mapping,
);
