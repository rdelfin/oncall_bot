/**
 * @prettier
 */

import { styled } from "@mui/material/styles";
import Grid from "@mui/material/Grid";
import Paper from "@mui/material/Paper";

const Item = styled(Paper)(({ theme }) => ({
  backgroundColor: theme.palette.mode === "dark" ? "#1A2027" : "#fff",
  ...theme.typography.body2,
  padding: theme.spacing(1),
  textAlign: "center",
  color: theme.palette.text.secondary,
}));

export default function Home() {
  return (
    <Grid container spacing={2}>
      <Grid item xs={4}>
        <Item>Wat</Item>
      </Grid>
      <Grid item xs={4}>
        <Item>Wat</Item>
      </Grid>
      <Grid item xs={4}>
        <Item>Wat</Item>
      </Grid>
    </Grid>
  );
}
