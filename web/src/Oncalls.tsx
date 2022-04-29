/**
 * @prettier
 */

import React, { useState } from "react";
import { styled } from "@mui/material/styles";
import Grid from "@mui/material/Grid";
import Paper from "@mui/material/Paper";
import CircularProgress from "@mui/material/CircularProgress";
import { Oncall, ListOncalls } from "./Api";

const Item = styled(Paper)(({ theme }) => ({
  backgroundColor: theme.palette.mode === "dark" ? "#1A2027" : "#fff",
  ...theme.typography.body2,
  padding: theme.spacing(1),
  textAlign: "center",
  color: theme.palette.text.secondary,
}));

export default function Oncalls() {
  const [oncalls, setOncalls] = useState<Oncall[]>([]);
  const [loaded, setLoaded] = useState<boolean>(false);

  React.useEffect(() => {
    ListOncalls().then(
      (result) => {
        if (result.oncalls) {
          setOncalls(result.oncalls);
        } else {
          console.log("Error fetching oncalls: " + result.error);
        }
        setLoaded(true);
      },
      (error) => {
        console.log("Error fetching oncalls: " + error);
        setLoaded(true);
      }
    );
  }, []);

  if (loaded) {
    return (
      <Grid container spacing={2}>
        {oncalls.map((oncall) => {
          return (
            <Grid item xs={4}>
              <Item>{oncall.name}</Item>
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
