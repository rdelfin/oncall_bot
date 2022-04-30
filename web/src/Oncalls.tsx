/**
 * @prettier
 */

import React, { useState } from "react";
import Grid from "@mui/material/Grid";
import CircularProgress from "@mui/material/CircularProgress";
import { Oncall, ListOncalls } from "./Api";
import OncallCard from "./components/OncallCard";
import LoadCard from "./components/LoadCard";

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
