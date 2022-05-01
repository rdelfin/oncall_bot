/**
 * @prettier
 */

import React, { useState } from "react";

import Autocomplete from "@mui/material/Autocomplete";
import Button from "@mui/material/Button";
import Dialog from "@mui/material/Dialog";
import DialogActions from "@mui/material/DialogActions";
import DialogContent from "@mui/material/DialogContent";
import DialogTitle from "@mui/material/DialogTitle";
import Paper from "@mui/material/Paper";
import TextField from "@mui/material/TextField";
import Table from "@mui/material/Table";
import TableCell from "@mui/material/TableCell";
import TableContainer from "@mui/material/TableContainer";
import TableHead from "@mui/material/TableHead";
import TableRow from "@mui/material/TableRow";
import Fab from "@mui/material/Fab";
import AddIcon from "@mui/icons-material/Add";

import { useSnackbar } from "notistack";

import {
  Oncall,
  ListUserGroups,
  UserGroup,
  OncallSync,
  SyncedWith,
  AddSync,
} from "../Api";

interface UserMapDialogProps {
  oncall: Oncall;
}

export default function OncallDialog(props: UserMapDialogProps) {
  const [open, setOpen] = useState<boolean>(false);
  const [userGroups, setUserGroups] = useState<UserGroup[]>([]);
  const [currentSyncs, setCurrentSyncs] = useState<OncallSync[]>([]);
  const [submitting, setSubmitting] = useState<boolean>(false);
  const [selectedId, setSelectedId] = useState<string | null>(null);

  const { enqueueSnackbar } = useSnackbar();

  const handleClickOpen = () => {
    setOpen(true);
    ListUserGroups().then(
      (result) => {
        if (result.user_groups !== undefined && result.user_groups !== null) {
          setUserGroups(result.user_groups);
        } else {
          enqueueSnackbar(`Error fetching user groups: ${result.error}`, {
            variant: "error",
          });
        }
      },
      (error) => {
        enqueueSnackbar(`Error fetching user groups: ${error}`, {
          variant: "error",
        });
      }
    );
    SyncedWith(props.oncall.id).then(
      (result) => {
        if (result.syncs !== undefined && result.syncs !== null) {
          setCurrentSyncs(result.syncs);
        } else {
          enqueueSnackbar(`Error fetching user groups: ${result.error}`, {
            variant: "error",
          });
        }
      },
      (error) => {
        enqueueSnackbar(`Error fetching user groups: ${error}`, {
          variant: "error",
        });
      }
    );
  };

  const handleClose = () => {
    if (!submitting) {
      setOpen(false);
    }
  };

  const handleSubmit = () => {
    if (selectedId === null) {
      console.log("No item selected");
    } else {
      setSubmitting(true);
      AddSync(props.oncall.id, selectedId)
        .then(
          (result) => {
            if (result.error !== undefined && result.error !== null) {
              enqueueSnackbar(`Error adding oncall sync: ${result.error}`, {
                variant: "error",
              });
            }
          },
          (error) => {
            enqueueSnackbar(`Error adding oncall sync: ${error}`, {
              variant: "error",
            });
          }
        )
        .then((result) => SyncedWith(props.oncall.id))
        .then(
          (result) => {
            if (result.syncs !== undefined && result.syncs !== null) {
              setCurrentSyncs(result.syncs);
            } else {
              enqueueSnackbar(`Error fetching user groups: ${result.error}`, {
                variant: "error",
              });
            }
            setSubmitting(false);
          },
          (error) => {
            enqueueSnackbar(`Error fetching user groups: ${error}`, {
              variant: "error",
            });
            setSubmitting(false);
          }
        );
    }
  };

  const userGroupOptions = userGroups.map((userGroup) => {
    return { label: userGroup.name, key: userGroup.id };
  });

  return (
    <div>
      <Button size="large" onClick={handleClickOpen}>
        Settings
      </Button>
      <Dialog open={open} onClose={handleClose}>
        <DialogTitle>{props.oncall.name} - Syncs</DialogTitle>
        <DialogContent>
          <TableContainer component={Paper} sx={{ minWidth: 550 }}>
            <Table aria-label="user group table">
              <TableHead>
                <TableCell>User Group</TableCell>
                <TableCell>Handle</TableCell>
                <TableCell>Add?</TableCell>
              </TableHead>
              {currentSyncs.map((sync) => (
                <TableRow>
                  <TableCell>{sync.user_group_name}</TableCell>
                  <TableCell>{sync.user_group_handle}</TableCell>
                  <TableCell></TableCell>
                </TableRow>
              ))}
              <TableRow>
                <TableCell>
                  <Autocomplete
                    id="user-group-map-field"
                    options={userGroupOptions}
                    fullWidth
                    disabled={submitting}
                    onChange={(event, userGroupOption) => {
                      if (
                        userGroupOption === null ||
                        userGroupOption === undefined
                      ) {
                        setSelectedId(null);
                      } else {
                        setSelectedId(userGroupOption.key);
                      }
                    }}
                    renderInput={(params) => (
                      <TextField {...params} label="Opsgenie User" />
                    )}
                  />
                </TableCell>
                <TableCell></TableCell>
                <TableCell>
                  <Fab
                    color="primary"
                    aria-label="add"
                    size="small"
                    disabled={submitting}
                    onClick={handleSubmit}
                  >
                    <AddIcon />
                  </Fab>
                </TableCell>
              </TableRow>
            </Table>
          </TableContainer>
        </DialogContent>
        <DialogActions>
          <Button onClick={handleClose} disabled={submitting}>
            Close
          </Button>
        </DialogActions>
      </Dialog>
    </div>
  );
}
