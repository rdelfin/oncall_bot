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
} from "./Api";
import UserCard from "./components/UserCard";
import LoadCard from "./components/LoadCard";

export default function Users() {
  const [slackUsers, setSlackUsers] = useState<SlackUser[]>([]);
  const [opsgenieUsers, setOpsgenieUsers] = useState<OpsgenieUser[]>([]);
  const [loaded, setLoaded] = useState<boolean>(false);

  React.useEffect(() => {
    ListSlackUsers().then(
      (result) => {
        if (result.users !== undefined && result.users !== null) {
          setSlackUsers(result.users);
        } else {
          console.log("Error fetching slack users: " + result.error);
        }
        setLoaded(true);
      },
      (error) => {
        console.log("Error fetching slack users: " + error);
        setLoaded(true);
      }
    );
  }, []);

  React.useEffect(() => {
    ListOpsgenieUsers().then(
      (result) => {
        if (result.users !== undefined && result.users !== null) {
          setOpsgenieUsers(result.users);
        } else {
          console.log("Error fetching slack users: " + result.error);
        }
        setLoaded(true);
      },
      (error) => {
        console.log("Error fetching slack users: " + error);
        setLoaded(true);
      }
    );
  }, []);

  if (loaded) {
    return (
      <Grid container spacing={2}>
        {slackUsers.map((slack_user) => {
          return (
            <Grid item xs={4}>
              <UserCard slack_user={slack_user} />
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
