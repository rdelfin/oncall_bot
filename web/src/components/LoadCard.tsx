import CircularProgress from "@mui/material/CircularProgress";
import Card from "@mui/material/Card";

export default function LoadCard() {
  return (
    <Card sx={{ minWidth: 275 }}>
      <CircularProgress />
    </Card>
  );
}
