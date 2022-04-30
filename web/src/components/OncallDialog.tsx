/**
 * @prettier
 */

import React, { useState } from "react";

import Grid from "@mui/material/Grid";
import Autocomplete from "@mui/material/Autocomplete";
import Button from "@mui/material/Button";
import Dialog from "@mui/material/Dialog";
import DialogActions from "@mui/material/DialogActions";
import DialogContent from "@mui/material/DialogContent";
import DialogContentText from "@mui/material/DialogContentText";
import DialogTitle from "@mui/material/DialogTitle";
import Paper from "@mui/material/Paper";
import TextField from "@mui/material/TextField";
import Table from "@mui/material/Table";
import TableBody from "@mui/material/TableBody";
import TableCell from "@mui/material/TableCell";
import TableContainer from "@mui/material/TableContainer";
import TableHead from "@mui/material/TableHead";
import TableRow from "@mui/material/TableRow";
import Fab from "@mui/material/Fab";
import AddIcon from "@mui/icons-material/Add";

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

const user_groups = [{ label: "a", key: "a" }];

export default function OncallDialog(props: UserMapDialogProps) {
  const [open, setOpen] = useState<boolean>(false);
  const [userGroups, setUserGroups] = useState<UserGroup[]>([]);
  const [currentSyncs, setCurrentSyncs] = useState<OncallSync[]>([]);
  const [submitting, setSubmitting] = useState<boolean>(false);
  const [selectedId, setSelectedId] = useState<string | null>(null);

  const handleClickOpen = () => {
    setOpen(true);
    ListUserGroups().then(
      (result) => {
        if (result.user_groups !== undefined && result.user_groups !== null) {
          setUserGroups(result.user_groups);
        } else {
          console.log("Error fetching user groups: " + result.error);
        }
      },
      (error) => {
        console.log("Error fetching user groups: " + error);
      }
    );
    console.log(`Oncall name: ${props.oncall.name}; ID: ${props.oncall.id}`);
    SyncedWith(props.oncall.id).then(
      (result) => {
        if (result.syncs !== undefined && result.syncs !== null) {
          setCurrentSyncs(result.syncs);
        } else {
          console.log("Error fetching user groups: " + result.error);
        }
      },
      (error) => {
        console.log("Error fetching user groups: " + error);
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
      console.log(
        `Adding sync for oncall: ${props.oncall.id}; ang group ${selectedId}`
      );
      AddSync(props.oncall.id, selectedId)
        .then(
          (result) => {
            if (result.error !== undefined && result.error !== null) {
              console.log("Error adding oncall sync: " + result.error);
            }
          },
          (error) => {
            console.log("Error adding oncall sync: " + error);
          }
        )
        .then((result) => SyncedWith(props.oncall.id))
        .then(
          (result) => {
            if (result.syncs !== undefined && result.syncs !== null) {
              setCurrentSyncs(result.syncs);
            } else {
              console.log("Error fetching user groups: " + result.error);
            }
            setSubmitting(false);
          },
          (error) => {
            console.log("Error fetching user groups: " + error);
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