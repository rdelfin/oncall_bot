/**
 * @prettier
 */

import React, { useState } from "react";

import Grid from "@mui/material/Grid";

import { useSnackbar } from "notistack";

import { SlackUser, ListSlackUsers, ListUserMappings } from "./Api";
import UserCard from "./components/UserCard";
import LoadCard from "./components/LoadCard";

export default function Users() {
  const [slackUsers, setSlackUsers] = useState<SlackUser[]>([]);
  const [userMappings, setUserMappings] = useState<{
    [slack_name: string]: string;
  }>({});
  const [loaded, setLoaded] = useState<boolean>(false);

  const { enqueueSnackbar } = useSnackbar();

  React.useEffect(() => {
    let slack_users_promise = ListSlackUsers().then(
      (result) => {
        if (result.users !== undefined && result.users !== null) {
          return result.users;
        }
        enqueueSnackbar(`Error fetching slack users: ${result.error}`, {
          variant: "error",
        });
        return [];
      },
      (error) => {
        enqueueSnackbar(`Error fetching slack users: ${error}`, {
          variant: "error",
        });
        return [];
      }
    );

    let user_mappings_promise = ListUserMappings().then(
      (result) => {
        if (
          result.user_mappings !== undefined &&
          result.user_mappings !== null
        ) {
          return result.user_mappings;
        }
        enqueueSnackbar(`Error fetching slack users: ${result.error}`, {
          variant: "error",
        });
        return [];
      },
      (error) => {
        enqueueSnackbar(`Error fetching slack users: ${error}`, {
          variant: "error",
        });
        return [];
      }
    );

    Promise.all([slack_users_promise, user_mappings_promise]).then(
      ([slack_users, user_mappings]) => {
        setSlackUsers(slack_users);
        setUserMappings(
          Object.assign(
            {},
            ...user_mappings.map((user_mapping) => ({
              [user_mapping.slack_user_id]: user_mapping.opsgenie_user_id,
            }))
          )
        );
        setLoaded(true);
      }
    );
  }, [enqueueSnackbar]);

  return (
    <div>
      <Grid container spacing={2}>
        {slackUsers.map((slack_user) => {
          let is_synced = slack_user.id in userMappings;
          return (
            <Grid item xs={4}>
              <UserCard slack_user={slack_user} is_synced={is_synced} />
            </Grid>
          );
        })}
      </Grid>
      <LoadCard open={!loaded} />
    </div>
  );
}
