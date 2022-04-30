/**
 * @prettier
 */

import Card from "@mui/material/Card";
import CardActions from "@mui/material/CardActions";
import CardContent from "@mui/material/CardContent";
import Button from "@mui/material/Button";
import Typography from "@mui/material/Typography";

import { Oncall } from "../Api";

interface OncallCardProps {
  oncall: Oncall;
}

export default function OncallCard(props: OncallCardProps) {
  return (
    <Card sx={{ minWidth: 275 }}>
      <CardContent>
        <Typography variant="h5" component="div">
          {props.oncall.name}
        </Typography>
      </CardContent>
      <CardActions>
        <Button size="large">Settings</Button>
      </CardActions>
    </Card>
  );
}
