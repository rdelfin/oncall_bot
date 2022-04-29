/**
 * @prettier
 */

import React, { useState } from "react";
import { styled } from "@mui/material/styles";
import Card from "@mui/material/Card";
import Grid from "@mui/material/Grid";
import CircularProgress from "@mui/material/CircularProgress";
import Typography from "@mui/material/Typography";
import {
  SlackUser,
  UserMapping,
  ListSlackUsers,
  OpsgenieUser,
  ListOpsgenieUsers,
  ListUserMappings,
} from "./Api";
import UserCard from "./components/UserCard";
import LoadCard from "./components/LoadCard";

export default function Users() {
  const [slackUsers, setSlackUsers] = useState<SlackUser[]>([]);
  const [opsgenieUsers, setOpsgenieUsers] = useState<OpsgenieUser[]>([]);
  const [userMappings, setUserMappings] = useState<{
    [slack_name: string]: string;
  }>({});
  const [loaded, setLoaded] = useState<boolean>(false);

  React.useEffect(() => {
    let slack_users_promise = ListSlackUsers().then(
      (result) => {
        if (result.users !== undefined && result.users !== null) {
          return result.users;
        }
        console.log("Error fetching slack users: " + result.error);
        return [];
      },
      (error) => {
        console.log("Error fetching slack users: " + error);
        return [];
      }
    );

    let opsgenie_users_promise = ListOpsgenieUsers().then(
      (result) => {
        if (result.users !== undefined && result.users !== null) {
          return result.users;
        }
        console.log("Error fetching slack users: " + result.error);
        return [];
      },
      (error) => {
        console.log("Error fetching slack users: " + error);
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
        console.log("Error fetching slack users: " + result.error);
        return [];
      },
      (error) => {
        console.log("Error fetching slack users: " + error);
        return [];
      }
    );

    Promise.all([
      slack_users_promise,
      opsgenie_users_promise,
      user_mappings_promise,
    ]).then(([slack_users, opsgenie_users, user_mappings]) => {
      setSlackUsers(slack_users);
      setOpsgenieUsers(opsgenie_users);
      setUserMappings(
        Object.assign(
          {},
          ...user_mappings.map((user_mapping) => ({
            [user_mapping.slack_user_id]: user_mapping.opsgenie_user_id,
          }))
        )
      );
      setLoaded(true);
    });
  }, []);

  if (loaded) {
    return (
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
    );
  } else {
    return (
      <Grid container>
        <Grid item xs={12}>
          <LoadCard />
        </Grid>
      </Grid>
    );
  }
}
