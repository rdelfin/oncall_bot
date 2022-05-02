/**
 * @prettier
 */

import React, { useState } from "react";

import { useSetRecoilState, SetterOrUpdater } from "recoil";

import Autocomplete from "@mui/material/Autocomplete";
import Button from "@mui/material/Button";
import Dialog from "@mui/material/Dialog";
import DialogActions from "@mui/material/DialogActions";
import DialogContent from "@mui/material/DialogContent";
import DialogContentText from "@mui/material/DialogContentText";
import DialogTitle from "@mui/material/DialogTitle";
import TextField from "@mui/material/TextField";
import Fab from "@mui/material/Fab";
import DeleteIcon from "@mui/icons-material/Delete";

import {
  useSnackbar,
  SnackbarKey,
  SnackbarMessage,
  OptionsObject as SnackbarOptionsObject,
} from "notistack";

import {
  GetSlackUserMapping,
  SlackUser,
  OpsgenieUser,
  ListOpsgenieUsers,
  AddUserMap,
  RemoveUserMap,
  ListUserMappings,
} from "../Api";
import { userMappingState, usersLoadedState } from "../State";

interface UserMapDialogProps {
  slack_user: SlackUser;
}

export default function UserMapDialog(props: UserMapDialogProps) {
  const [open, setOpen] = useState<boolean>(false);
  const [updating, setUpdating] = useState<boolean>(false);
  const [opsgenieUsers, setOpsgenieUsers] = useState<OpsgenieUser[]>([]);
  const [userMappingId, setUserMappingId] = useState<number | null>(null);
  const [selectedId, setSelectedId] = useState<string | null>(null);

  const setLoaded = useSetRecoilState<boolean>(usersLoadedState);
  const setUserMappings = useSetRecoilState<{
    [slack_name: string]: string;
  }>(userMappingState);

  const { enqueueSnackbar } = useSnackbar();

  const handleClickOpen = () => {
    setUpdating(true);
    setUserMappingId(null);
    setOpen(true);
    GetSlackUserMapping(props.slack_user.id)
      .then((result) => {
        if (result.user_mapping !== undefined && result.user_mapping !== null) {
          setUserMappingId(result.user_mapping.id);
        }
      })
      .finally(() => {
        setUpdating(false);
      });

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
      AddUserMap(props.slack_user.id, selectedId)
        .then(
          (result) => {
            if (result.error !== undefined && result.error !== null) {
              enqueueSnackbar(
                `Error submitting user mapping: ${result.error}`,
                {
                  variant: "error",
                }
              );
            }
          },
          (error) => {
            enqueueSnackbar(`Error submitting user mapping: ${error}`, {
              variant: "error",
            });
          }
        )
        .finally(() => {
          setOpen(false);
        })
        .then(() =>
          updateUserMappings(setLoaded, setUserMappings, enqueueSnackbar)
        );
    }
  };

  const handleRemove = (event: React.MouseEvent<HTMLElement>) => {
    if (userMappingId === null) {
      enqueueSnackbar(
        "User mapping ID not set while trying to delete user mapping. Please refresh",
        {
          variant: "error",
        }
      );
    } else {
      setUpdating(true);
      RemoveUserMap(userMappingId)
        .then(
          (result) => {
            if (result.error !== undefined && result.error !== null) {
              enqueueSnackbar(`Error removing user mapping: ${result.error}`, {
                variant: "error",
              });
            }
          },
          (error) => {
            enqueueSnackbar(`Error removing user mapping: ${error}`, {
              variant: "error",
            });
          }
        )
        .finally(() => {
          setOpen(false);
        })
        .then(() =>
          updateUserMappings(setLoaded, setUserMappings, enqueueSnackbar)
        );
    }
  };

  let opsgenieUserFields = opsgenieUsers.map((user) => {
    return { label: user.fullName, key: user.id };
  });

  const remove_input_elements =
    userMappingId !== null ? (
      <Fab
        variant="extended"
        color="primary"
        aria-label="add"
        disabled={updating}
        onClick={handleRemove}
      >
        <DeleteIcon sx={{ mr: 1 }} />
        Remove Link
      </Fab>
    ) : (
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
    );

  return (
    <div>
      <Button size="large" onClick={handleClickOpen}>
        Link to Opsgenie
      </Button>
      <Dialog open={open} onClose={handleClose} fullWidth maxWidth="sm">
        <DialogTitle>Link to Opsgenie</DialogTitle>
        <DialogContent>
          <DialogContentText>
            {props.slack_user.real_name ?? props.slack_user.name}
          </DialogContentText>
          {remove_input_elements}
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

function updateUserMappings(
  setLoaded: SetterOrUpdater<boolean>,
  setUserMappings: SetterOrUpdater<{
    [slack_name: string]: string;
  }>,
  enqueueSnackbar: (
    message: SnackbarMessage,
    options?: SnackbarOptionsObject
  ) => SnackbarKey
): Promise<void> {
  return Promise.resolve()
    .then(() => {
      setLoaded(false);
      return ListUserMappings();
    })
    .then(
      (result) => {
        if (
          result.user_mappings !== undefined &&
          result.user_mappings !== null
        ) {
          setUserMappings(
            Object.assign(
              {},
              ...result.user_mappings.map((user_mapping) => ({
                [user_mapping.slack_user_id]: user_mapping.opsgenie_user_id,
              }))
            )
          );
        } else {
          enqueueSnackbar(`Error fetching slack users: ${result.error}`, {
            variant: "error",
          });
        }
      },
      (error) => {
        enqueueSnackbar(`Error fetching slack users: ${error}`, {
          variant: "error",
        });
      }
    )
    .finally(() => {
      setLoaded(true);
    });
}
