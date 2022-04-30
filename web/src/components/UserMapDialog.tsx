/**
 * @prettier
 */

import React, { useState } from "react";

import Autocomplete from "@mui/material/Autocomplete";
import Button from "@mui/material/Button";
import Dialog from "@mui/material/Dialog";
import DialogActions from "@mui/material/DialogActions";
import DialogContent from "@mui/material/DialogContent";
import DialogContentText from "@mui/material/DialogContentText";
import DialogTitle from "@mui/material/DialogTitle";
import TextField from "@mui/material/TextField";

import {
  GetSlackUserMapping,
  SlackUser,
  OpsgenieUser,
  ListOpsgenieUsers,
} from "../Api";

interface UserMapDialogProps {
  slack_user: SlackUser;
}

export default function UserMapDialog(props: UserMapDialogProps) {
  const [open, setOpen] = useState<boolean>(false);
  const [opsgenieUsers, setOpsgenieUsers] = useState<OpsgenieUser[]>([]);
  const [hasUserMapping, setHasUserMapping] = useState<boolean | null>(null);

  const handleClickOpen = () => {
    setHasUserMapping(null);
    setOpen(true);
    GetSlackUserMapping(props.slack_user.id).then(
      (result) => {
        if (
          result.opsgenie_user_id !== undefined &&
          result.opsgenie_user_id !== null
        ) {
          setHasUserMapping(true);
        } else {
          setHasUserMapping(false);
        }
      },
      (error) => {
        setHasUserMapping(false);
      }
    );

    ListOpsgenieUsers().then(
      (result) => {
        if (result.users !== undefined && result.users !== null) {
          setOpsgenieUsers(result.users);
        } else {
          console.log("Error fetching opsgenie users: " + result.error);
        }
      },
      (error) => {
        console.log("Error fetching opsgenie users: " + error);
      }
    );
  };

  const handleClose = () => {
    setOpen(false);
  };

  let has_user_mapping_text = hasUserMapping ? "" : "No user mapping";
  let opsgenieUserFields = opsgenieUsers.map((user) => {
    return { label: user.fullName, key: user.id };
  });

  return (
    <div>
      <Button size="large" onClick={handleClickOpen}>
        Link to Opsgenie
      </Button>
      <Dialog open={open} onClose={handleClose}>
        <DialogTitle>Link to Opsgenie</DialogTitle>
        <DialogContent>
          <DialogContentText>
            Use this to map slack users to opsgenie users.
          </DialogContentText>
          <DialogContentText>{has_user_mapping_text}</DialogContentText>
          <Autocomplete
            disablePortal
            id="opsgenie-user-link"
            options={opsgenieUserFields}
            fullWidth
            renderInput={(params) => (
              <TextField {...params} label="Opsgenie User" />
            )}
          />
        </DialogContent>
        <DialogActions>
          <Button onClick={handleClose}>Cancel</Button>
          <Button onClick={handleClose}>Update</Button>
        </DialogActions>
      </Dialog>
    </div>
  );
}
