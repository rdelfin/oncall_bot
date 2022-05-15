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
  notificationsCardLoadingState,
  notificationsCardDeletingState,
  notificationsCardAddingState,
} from "../State";
import UserMapSyncBox from "./UserMapSyncBox";
import NotificationBox from "./NotificationBox";

export default function OncallDialog() {
  const [oncallCard, setOncallCard] = useRecoilState<Oncall | null>(
    oncallCardState
  );
  const oncallCardLoading = useRecoilValue<boolean>(oncallCardLoadingState);
  const oncallCardDeleting = useRecoilValue<boolean>(oncallCardDeletingState);
  const oncallCardAdding = useRecoilValue<boolean>(oncallCardAddingState);
  const notificationsCardLoading = useRecoilValue<boolean>(
    notificationsCardLoadingState
  );
  const notificationsCardDeleting = useRecoilValue<boolean>(
    notificationsCardDeletingState
  );
  const notificationsCardAdding = useRecoilValue<boolean>(
    notificationsCardAddingState
  );

  const handleClose = () => {
    if (!(oncallCardLoading || oncallCardDeleting || oncallCardAdding)) {
      setOncallCard(null);
    }
  };

  const loading =
    oncallCardLoading ||
    oncallCardDeleting ||
    oncallCardAdding ||
    notificationsCardLoading ||
    notificationsCardDeleting ||
    notificationsCardAdding;

  return (
    <div>
      <Dialog open={oncallCard !== null} onClose={handleClose}>
        <DialogTitle>{oncallCard?.name ?? ""} - Syncs</DialogTitle>
        <DialogContent>
          <UserMapSyncBox oncall={oncallCard} />
          <Divider variant="middle" />
          <NotificationBox oncall={oncallCard} />
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
