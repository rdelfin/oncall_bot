/**
 * @prettier
 */

import Button from "@mui/material/Button";
import Divider from "@mui/material/Divider";
import Dialog from "@mui/material/Dialog";
import DialogActions from "@mui/material/DialogActions";
import DialogContent from "@mui/material/DialogContent";
import DialogTitle from "@mui/material/DialogTitle";

import { useRecoilState, useRecoilValue } from "recoil";

import { Oncall } from "../Api";
import {
  oncallCardState,
  oncallCardLoadingState,
  oncallCardDeletingState,
  oncallCardAddingState,
} from "../State";
import UserMapSyncBox from "./UserMapSyncBox";

export default function OncallDialog() {
  const [oncallCard, setOncallCard] = useRecoilState<Oncall | null>(
    oncallCardState
  );
  const oncallCardLoading = useRecoilValue<boolean>(oncallCardLoadingState);
  const oncallCardDeleting = useRecoilValue<boolean>(oncallCardDeletingState);
  const oncallCardAdding = useRecoilValue<boolean>(oncallCardAddingState);

  const handleClose = () => {
    if (!(oncallCardLoading || oncallCardDeleting || oncallCardAdding)) {
      setOncallCard(null);
    }
  };

  const loading = oncallCardLoading || oncallCardDeleting || oncallCardAdding;

  return (
    <div>
      <Dialog open={oncallCard !== null} onClose={handleClose}>
        <DialogTitle>{oncallCard?.name ?? ""} - Syncs</DialogTitle>
        <DialogContent>
          <UserMapSyncBox oncall={oncallCard} />
          <Divider variant="middle" />
        </DialogContent>
        <DialogActions>
          <Button onClick={handleClose} disabled={loading}>
            Close
          </Button>
        </DialogActions>
      </Dialog>
    </div>
  );
}
