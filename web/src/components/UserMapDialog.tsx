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

import { useSnackbar } from "notistack";

import {
  GetSlackUserMapping,
  SlackUser,
  OpsgenieUser,
  ListOpsgenieUsers,
  AddUserMap,
} from "../Api";

interface UserMapDialogProps {
  slack_user: SlackUser;
}

export default function UserMapDialog(props: UserMapDialogProps) {
  const [open, setOpen] = useState<boolean>(false);
  const [updating, setUpdating] = useState<boolean>(false);
  const [opsgenieUsers, setOpsgenieUsers] = useState<OpsgenieUser[]>([]);
  const [hasUserMapping, setHasUserMapping] = useState<boolean | null>(null);
  const [selectedId, setSelectedId] = useState<string | null>(null);

  const { enqueueSnackbar } = useSnackbar();

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
          enqueueSnackbar(`Error fetching opsgenie users: ${result.error}`, {
            variant: "error",
          });
        }
      },
      (error) => {
        enqueueSnackbar(`Error fetching opsgenie users: ${error}`, {
          variant: "error",
        });
      }
    );
  };

  const handleClose = () => {
    if (!updating) {
      setOpen(false);
    }
  };

  const handleSubmit = () => {
    if (selectedId === null) {
      enqueueSnackbar("You need to select a mapped user", {
        variant: "error",
      });
    } else {
      setUpdating(true);
      AddUserMap(props.slack_user.id, selectedId).then(
        (result) => {
          if (result.error !== undefined && result.error !== null) {
            enqueueSnackbar(`Error submitting user mapping: ${result.error}`, {
              variant: "error",
            });
          } else {
            // If all goes well
            setOpen(false);
          }
        },
        (error) => {
          enqueueSnackbar(`Error submitting user mapping: ${error}`, {
            variant: "error",
          });
        }
      );
      setOpen(false);
    }
  };

  let has_user_mapping_text = hasUserMapping
    ? "User already mapped to opsgenie"
    : "No user mapping";
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
            id="opsgenie-user-link"
            options={opsgenieUserFields}
            fullWidth
            disabled={updating}
            onChange={(event, opsgenie_user) => {
              if (opsgenie_user === null || opsgenie_user === undefined) {
                setSelectedId(null);
              } else {
                setSelectedId(opsgenie_user.key);
              }
            }}
            renderInput={(params) => (
              <TextField {...params} label="Opsgenie User" />
            )}
          />
        </DialogContent>
        <DialogActions>
          <Button onClick={handleClose} disabled={updating}>
            Cancel
          </Button>
          <Button onClick={handleSubmit} disabled={updating}>
            Update
          </Button>
        </DialogActions>
      </Dialog>
    </div>
  );
}
