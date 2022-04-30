/**
 * @prettier
 */

import Card from "@mui/material/Card";
import CardActions from "@mui/material/CardActions";
import CardContent from "@mui/material/CardContent";
import Typography from "@mui/material/Typography";
import { green, red } from "@mui/material/colors";

import { SlackUser } from "../Api";
import UserMapDialog from "./UserMapDialog";

interface UserCardProps {
  slack_user: SlackUser;
  is_synced: boolean;
}

export default function UserCard(props: UserCardProps) {
  let color = props.is_synced ? green[200] : red[200];
  if (props.slack_user.real_name && props.slack_user.name) {
    return (
      <Card sx={{ minWidth: 275, bgcolor: color }}>
        <CardContent>
          <Typography variant="h5" component="div">
            {props.slack_user.real_name}
          </Typography>
          <Typography sx={{ mb: 1.5 }} color="text.secondary">
            {props.slack_user.name}
          </Typography>
          <Typography sx={{ mb: 1.5 }} color="text.secondary"></Typography>
        </CardContent>
        <CardActions>
          <UserMapDialog slack_user={props.slack_user} />
        </CardActions>
      </Card>
    );
  } else {
    let used_name = props.slack_user.real_name || props.slack_user.name;

    return (
      <Card sx={{ minWidth: 275, bgcolor: color }}>
        <CardContent>
          <Typography variant="h5" component="div">
            {used_name}
          </Typography>
          <Typography sx={{ mb: 1.5 }} color="text.secondary">
            .
          </Typography>
          <Typography sx={{ mb: 1.5 }} color="text.secondary"></Typography>
        </CardContent>
        <CardActions>
          <UserMapDialog slack_user={props.slack_user} />
        </CardActions>
      </Card>
    );
  }
}
