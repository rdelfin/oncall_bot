/**
 * @prettier
 */

import React, { useState } from "react";

import Grid from "@mui/material/Grid";

import { useSnackbar } from "notistack";

import { Oncall, ListOncalls } from "./Api";
import OncallCard from "./components/OncallCard";
import LoadCard from "./components/LoadCard";

export default function Oncalls() {
  const [oncalls, setOncalls] = useState<Oncall[]>([]);
  const [loaded, setLoaded] = useState<boolean>(false);

  const { enqueueSnackbar } = useSnackbar();

  React.useEffect(() => {
    ListOncalls().then(
      (result) => {
        if (result.oncalls) {
          setOncalls(result.oncalls);
        } else {
          enqueueSnackbar(`Error fetching oncalls: ${result.error}`, {
            variant: "error",
          });
        }
        setLoaded(true);
      },
      (error) => {
        enqueueSnackbar(`Error fetching oncalls: ${error}`, {
          variant: "error",
        });
        setLoaded(true);
      }
    );
  }, [enqueueSnackbar]);

  return (
    <div>
      <Grid container spacing={2}>
        {oncalls.map((oncall) => (
          <Grid item xs={4}>
            <OncallCard oncall={oncall} />
          </Grid>
        ))}
      </Grid>
      <LoadCard open={!loaded} />
    </div>
  );
}
