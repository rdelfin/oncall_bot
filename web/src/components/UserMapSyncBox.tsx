/**
 * @prettier
 */

import React, { useState } from "react";

import Autocomplete from "@mui/material/Autocomplete";
import Box from "@mui/material/Box";
import Fab from "@mui/material/Fab";
import Paper from "@mui/material/Paper";
import Table from "@mui/material/Table";
import TableCell from "@mui/material/TableCell";
import TableContainer from "@mui/material/TableContainer";
import TableHead from "@mui/material/TableHead";
import TableRow from "@mui/material/TableRow";
import TextField from "@mui/material/TextField";
import Typography from "@mui/material/Typography";
import AddIcon from "@mui/icons-material/Add";
import RemoveIcon from "@mui/icons-material/Remove";

import { useRecoilState } from "recoil";

import { useSnackbar } from "notistack";

import {
  Oncall,
  ListUserGroups,
  UserGroup,
  OncallSync,
  SyncedWith,
  AddSync,
  RemoveSync,
} from "../Api";
import {
  oncallCardLoadingState,
  oncallCardDeletingState,
  oncallCardAddingState,
} from "../State";

interface UserMapSyncBoxProps {
  oncall: Oncall | null;
}

export default function UserMapSyncBox(props: UserMapSyncBoxProps) {
  const [currentSyncs, setCurrentSyncs] = useState<OncallSync[]>([]);
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [userGroups, setUserGroups] = useState<UserGroup[]>([]);

  const [oncallCardLoading, setOncallCardLoading] = useRecoilState<boolean>(
    oncallCardLoadingState
  );
  const [oncallCardDeleting, setOncallCardDeleting] = useRecoilState<boolean>(
    oncallCardDeletingState
  );
  const [oncallCardAdding, setOncallCardAdding] = useRecoilState<boolean>(
    oncallCardAddingState
  );

  const { enqueueSnackbar } = useSnackbar();

  React.useEffect(() => {
    if (props.oncall !== null) {
      setOncallCardLoading(true);
      let userGroupsPromise = ListUserGroups().then(
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
      let syncedWithPromise = SyncedWith(props.oncall.id).then(
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

      Promise.all([userGroupsPromise, syncedWithPromise]).finally(() => {
        setOncallCardLoading(false);
      });
    }
  }, [props.oncall, enqueueSnackbar, setOncallCardLoading]);

  const handleRemove = (
    event: React.MouseEvent<HTMLElement>,
    oncall_sync_id: number
  ) => {
    if (props.oncall === null) {
      enqueueSnackbar("Error removing oncall group: Oncall is null", {
        variant: "error",
      });
      return;
    }
    let oncall_id = props.oncall.id;

    setOncallCardDeleting(true);
    RemoveSync(oncall_sync_id)
      .then(
        (result) => {
          if (result.error !== undefined && result.error !== null) {
            enqueueSnackbar(`Error removing oncall sync: ${result.error}`, {
              variant: "error",
            });
          }
        },
        (error) => {
          enqueueSnackbar(`Error removing oncall sync: ${error}`, {
            variant: "error",
          });
        }
      )
      .then((result) => SyncedWith(oncall_id))
      .then(
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
      )
      .finally(() => {
        setOncallCardDeleting(false);
      });
  };

  const handleSubmit = () => {
    if (selectedId === null) {
      enqueueSnackbar("No item selected", { variant: "error" });
    } else if (props.oncall === null) {
      enqueueSnackbar("Oncall is unexpectedly null", { variant: "error" });
    } else {
      let oncall_id = props.oncall.id;
      setOncallCardAdding(true);

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
        .then((result) => SyncedWith(oncall_id))
        .then(
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
        )
        .finally(() => {
          setOncallCardAdding(false);
        });
    }
  };

  let disabled = oncallCardLoading || oncallCardDeleting || oncallCardAdding;

  const userGroupOptions = userGroups.map((userGroup) => {
    return { label: userGroup.name, key: userGroup.id };
  });

  return (
    <Box sx={{ my: 4 }}>
      <Typography variant="subtitle1">User Group Syncs</Typography>
      <TableContainer component={Paper} sx={{ minWidth: 550 }}>
        <Table aria-label="user group table">
          <TableHead>
            <TableCell>User Group</TableCell>
            <TableCell>Handle</TableCell>
            <TableCell>Add/Remove</TableCell>
          </TableHead>
          {currentSyncs.map((sync) => (
            <TableRow>
              <TableCell>{sync.user_group_name}</TableCell>
              <TableCell>{sync.user_group_handle}</TableCell>
              <TableCell>
                <Fab
                  color="primary"
                  aria-label="remove"
                  size="small"
                  disabled={disabled}
                  onClick={(e) => {
                    handleRemove(e, sync.id);
                  }}
                >
                  <RemoveIcon />
                </Fab>
              </TableCell>
            </TableRow>
          ))}
          <TableRow>
            <TableCell>
              <Autocomplete
                id="user-group-map-field"
                options={userGroupOptions}
                fullWidth
                disabled={disabled}
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
                disabled={disabled}
                onClick={handleSubmit}
              >
                <AddIcon />
              </Fab>
            </TableCell>
          </TableRow>
        </Table>
      </TableContainer>
    </Box>
  );
}
