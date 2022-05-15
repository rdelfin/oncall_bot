/**
 * @prettier
 */

import Button from "@mui/material/Button";
import Card from "@mui/material/Card";
import CardActions from "@mui/material/CardActions";
import CardContent from "@mui/material/CardContent";
import Typography from "@mui/material/Typography";

import { useSetRecoilState } from "recoil";

import { Oncall } from "../Api";
import { oncallCardState } from "../State";

interface OncallCardProps {
  oncall: Oncall;
}

export default function OncallCard(props: OncallCardProps) {
  const setOncallCard = useSetRecoilState<Oncall | null>(oncallCardState);

  const handleClickOpen = () => {
    setOncallCard(props.oncall);
  };

  return (
    <Card sx={{ minWidth: 275 }}>
      <CardContent>
        <Typography variant="h5" component="div">
          {props.oncall.name}
        </Typography>
      </CardContent>
      <CardActions>
        <Button size="large" onClick={handleClickOpen}>
          Settings
        </Button>
      </CardActions>
    </Card>
  );
}
