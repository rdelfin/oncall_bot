/**
 * @prettier
 */

import CircularProgress from "@mui/material/CircularProgress";
import Backdrop from "@mui/material/Backdrop";

interface LoadCardProps {
  open: boolean;
}

export default function LoadCard(props: LoadCardProps) {
  return (
    <Backdrop open={props.open}>
      <CircularProgress color="inherit" />
    </Backdrop>
  );
}
