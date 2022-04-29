/**
 * @prettier
 */

import Card from "@mui/material/Card";
import CardActions from "@mui/material/CardActions";
import CardContent from "@mui/material/CardContent";
import Button from "@mui/material/Button";
import Typography from "@mui/material/Typography";

import { SlackUser } from "../Api";

interface UserCardProps {
  slack_user: SlackUser;
}

export default function UserCard(props: UserCardProps) {
  if (props.slack_user.real_name && props.slack_user.name) {
    return (
      <Card sx={{ minWidth: 275 }}>
        <CardContent>
          <Typography variant="h5" component="div">
            {props.slack_user.real_name}
          </Typography>
          <Typography sx={{ mb: 1.5 }} color="text.secondary">
            {props.slack_user.name}
          </Typography>
        </CardContent>
        <CardActions>
          <Button size="large">Link</Button>
        </CardActions>
      </Card>
    );
  } else {
    let used_name = props.slack_user.real_name || props.slack_user.name;

    return (
      <Card sx={{ minWidth: 275 }}>
        <CardContent>
          <Typography variant="h5" component="div">
            {used_name}
          </Typography>
          <Typography sx={{ mb: 1.5 }} color="text.secondary">
            .
          </Typography>
        </CardContent>
        <CardActions>
          <Button size="large">Link</Button>
        </CardActions>
      </Card>
    );
  }
}
