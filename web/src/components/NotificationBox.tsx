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

import { useRecoilState, useRecoilValue } from "recoil";

import { useSnackbar } from "notistack";

import {
  Oncall,
  ListSlackChannels,
  AddNotification,
  RemoveNotification,
  GetNotificationForOncall,
  Notification,
  SlackChannel,
} from "../Api";
import {
  oncallCardLoadingState,
  oncallCardDeletingState,
  oncallCardAddingState,
  notificationsCardLoadingState,
  notificationsCardDeletingState,
  notificationsCardAddingState,
} from "../State";

interface NotificationBoxProps {
  oncall: Oncall | null;
}

export default function NotificationBox(props: NotificationBoxProps) {
  const [currentNotifications, setCurrentNotifications] = useState<
    Notification[]
  >([]);
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [slackChannels, setSlackChannels] = useState<SlackChannel[]>([]);

  const oncallCardLoading = useRecoilValue<boolean>(oncallCardLoadingState);
  const oncallCardDeleting = useRecoilValue<boolean>(oncallCardDeletingState);
  const oncallCardAdding = useRecoilValue<boolean>(oncallCardAddingState);
  const [notificationsCardLoading, setNotificationsCardLoading] =
    useRecoilState<boolean>(notificationsCardLoadingState);
  const [notificationsCardDeleting, setNotificationsCardDeleting] =
    useRecoilState<boolean>(notificationsCardDeletingState);
  const [notificationsCardAdding, setNotificationsCardAdding] =
    useRecoilState<boolean>(notificationsCardAddingState);

  const { enqueueSnackbar } = useSnackbar();

  React.useEffect(() => {
    if (props.oncall !== null) {
      setNotificationsCardLoading(true);
      let listSlackChannelsPromise = ListSlackChannels().then(
        (result) => {
          if (result.channels !== undefined && result.channels !== null) {
            setSlackChannels(result.channels);
          } else {
            enqueueSnackbar(`Error fetching slack channels: ${result.error}`, {
              variant: "error",
            });
          }
        },
        (error) => {
          enqueueSnackbar(`Error fetching slack channels: ${error}`, {
            variant: "error",
          });
        }
      );
      let getNotificationForOncallPromise = GetNotificationForOncall(
        props.oncall.id
      ).then(
        (result) => {
          if (
            result.notifications !== undefined &&
            result.notifications !== null
          ) {
            setCurrentNotifications(result.notifications);
          } else {
            enqueueSnackbar(`Error fetching notifications: ${result.error}`, {
              variant: "error",
            });
          }
        },
        (error) => {
          enqueueSnackbar(`Error fetching notifications: ${error}`, {
            variant: "error",
          });
        }
      );

      Promise.all([
        listSlackChannelsPromise,
        getNotificationForOncallPromise,
      ]).finally(() => {
        setNotificationsCardLoading(false);
      });
    }
  }, [props.oncall, enqueueSnackbar, setNotificationsCardLoading]);

  const handleRemove = (
    event: React.MouseEvent<HTMLElement>,
    notification_id: number
  ) => {
    if (props.oncall === null) {
      enqueueSnackbar("Error removing slack channel: Oncall is null", {
        variant: "error",
      });
      return;
    }
    let oncall_id = props.oncall.id;

    setNotificationsCardDeleting(true);
    RemoveNotification(notification_id)
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
      .then((result) => GetNotificationForOncall(oncall_id))
      .then(
        (result) => {
          if (
            result.notifications !== undefined &&
            result.notifications !== null
          ) {
            setCurrentNotifications(result.notifications);
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
        setNotificationsCardDeleting(false);
      });
  };

  const handleSubmit = () => {
    if (selectedId === null) {
      enqueueSnackbar("No item selected", { variant: "error" });
    } else if (props.oncall === null) {
      enqueueSnackbar("Oncall is unexpectedly null", { variant: "error" });
    } else {
      let oncall_id = props.oncall.id;
      setNotificationsCardAdding(true);

      AddNotification(props.oncall.id, selectedId)
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
        .then((result) => GetNotificationForOncall(oncall_id))
        .then(
          (result) => {
            if (
              result.notifications !== undefined &&
              result.notifications !== null
            ) {
              setCurrentNotifications(result.notifications);
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
          setNotificationsCardAdding(false);
        });
    }
  };

  let disabled =
    oncallCardLoading ||
    oncallCardDeleting ||
    oncallCardAdding ||
    notificationsCardLoading ||
    notificationsCardDeleting ||
    notificationsCardAdding;

  const slackChannelOptions = slackChannels.map((channel) => {
    return { label: channel.name, key: channel.id };
  });

  return (
    <Box sx={{ my: 4 }}>
      <Typography variant="subtitle1">Slack Channel Notifications</Typography>
      <TableContainer component={Paper} sx={{ minWidth: 550 }}>
        <Table aria-label="slack channel table">
          <TableHead>
            <TableCell>Slack Channel</TableCell>
            <TableCell>Add/Remove</TableCell>
          </TableHead>
          {currentNotifications.map((notification) => (
            <TableRow>
              <TableCell>{notification.slack_channel_name}</TableCell>
              <TableCell>
                <Fab
                  color="primary"
                  aria-label="remove"
                  size="small"
                  disabled={disabled}
                  onClick={(e) => {
                    handleRemove(e, notification.id);
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
                id="slack-channel-field"
                options={slackChannelOptions}
                fullWidth
                disabled={disabled}
                onChange={(event, slackChannelOption) => {
                  if (
                    slackChannelOption === null ||
                    slackChannelOption === undefined
                  ) {
                    setSelectedId(null);
                  } else {
                    setSelectedId(slackChannelOption.key);
                  }
                }}
                renderInput={(params) => (
                  <TextField {...params} label="Slack Channel" />
                )}
              />
            </TableCell>
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
