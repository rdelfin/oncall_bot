/**
 * @prettier
 */

import React, { useState } from "react";
import { styled } from "@mui/material/styles";
import Grid from "@mui/material/Grid";
import Paper from "@mui/material/Paper";
import CircularProgress from "@mui/material/CircularProgress";
import { SlackUser, OpsgenieUser, UserMapping, ListSlackUsers } from "./Api";

const Item = styled(Paper)(({ theme }) => ({
  backgroundColor: theme.palette.mode === "dark" ? "#1A2027" : "#fff",
  ...theme.typography.body2,
  padding: theme.spacing(1),
  textAlign: "center",
  color: theme.palette.text.secondary,
}));

export default function Users() {
  const [slackUsers, setSlackUsers] = useState<SlackUser[]>([]);
  const [loaded, setLoaded] = useState<boolean>(false);

  React.useEffect(() => {
    ListSlackUsers().then(
      (result) => {
        if (result.users) {
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

  if (loaded) {
    return (
      <Grid container spacing={2}>
        {slackUsers.map((slack_user) => {
          return (
            <Grid item xs={4}>
              <Item>
                {slack_user.real_name}
                <br />
                {slack_user.name}
              </Item>
            </Grid>
          );
        })}
      </Grid>
    );
  } else {
    return (
      <Grid container>
        <Grid item xs={12}>
          <Item>
            <CircularProgress />
          </Item>
        </Grid>
      </Grid>
    );
  }
}
