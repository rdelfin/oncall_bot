/**
 * @prettier
 */

export interface SlackUser {
  id: string;
  name: string;
  real_name: string;
  is_bot: boolean;
}

export interface SlackChannelTopic {
  value: string;
  creator: string;
  last_set: number;
}

export interface SlackChannel {
  id: string;
  name: string;
  topic: SlackChannelTopic;
}

export interface OpsgenieUser {
  id: string;
  username: string;
  fullName: string;
}

export interface UserMapping {
  id: number;
  opsgenie_user_id: string;
  slack_user_id: string;
}

export interface Oncall {
  id: string;
  name: string;
}

export interface OncallSync {
  id: number;
  oncall_id: string;
  oncall_name: string;
  user_group_id: string;
  user_group_name: string;
  user_group_handle: string;
}

export interface UserGroup {
  id: string;
  name: string;
  handle: string;
}

export interface Notification {
  id: number;
  oncall_id: string;
  oncall_name: string;
  slack_channel_id: string;
  slack_channel_name: string;
}

export interface ListSlackUsersResponse {
  users?: SlackUser[];
  error?: string;
}

export interface ListSlackChannelsResponse {
  channels?: SlackChannel[];
  error?: string;
}

export interface ListOpsgenieUsersResponse {
  users?: OpsgenieUser[];
  error?: string;
}

export interface ListUserMappingsResponse {
  user_mappings?: UserMapping[];
  error?: string;
}

export interface ListOncallsResponse {
  oncalls?: Oncall[];
  error?: string;
}

export interface ListUserGroupsResponse {
  user_groups?: UserGroup[] | null;
  error?: string | null;
}

export interface ListSyncsResponse {
  syncs?: OncallSync[] | null;
  error?: string | null;
}

export interface ListNotificationsResponse {
  notifications?: Notification[];
  error?: string;
}

export interface GetSlackUserMappingResponse {
  user_mapping?: UserMapping | null;
}

export interface GetNotificationForSlackChannelResponse {
  notification?: Notification | null;
  error?: string;
}

export interface GetNotificationForOncallResponse {
  notifications: Notification[];
  error?: string;
}

export interface AddUserMapResponse {
  opsgenie_user_id?: string | null;
  slack_user_id?: string | null;
  error?: string | null;
}

export interface SyncedWithResponse {
  syncs?: OncallSync[] | null;
  error?: string | null;
}

export interface AddSyncResponse {
  id?: number | null;
  oncall_id?: string | null;
  user_group_id?: string | null;
  error?: string | null;
}

export interface AddNotificationResponse {
  notification?: Notification | null;
  error?: string | null;
}

export interface RemoveSyncResponse {
  id?: number | null;
  oncall_id?: string | null;
  user_group_id?: string | null;
  error?: string | null;
}

export interface RemoveUserMapResponse {
  opsgenie_user_id?: string | null;
  slack_user_id?: string | null;
  error?: string | null;
}

export interface RemoveNotificationResponse {
  notification?: Notification | null;
  error?: string | null;
}

export function ListOpsgenieUsers(): Promise<ListOpsgenieUsersResponse> {
  return fetch("/api/list_opsgenie_users").then((res) => res.json());
}

export function ListSlackUsers(): Promise<ListSlackUsersResponse> {
  return fetch("/api/list_slack_users").then((res) => res.json());
}

export function ListSlackChannels(): Promise<ListSlackChannelsResponse> {
  return fetch("/api/list_slack_channels").then((res) => res.json());
}

export function ListUserMappings(): Promise<ListUserMappingsResponse> {
  return fetch("/api/list_user_mappings").then((res) => res.json());
}

export function ListOncalls(): Promise<ListOncallsResponse> {
  return fetch("/api/list_oncalls").then((res) => res.json());
}

export function ListNotifications(): Promise<ListNotificationsResponse> {
  return fetch("/api/notification/list").then((res) => res.json());
}

export function GetSlackUserMapping(
  slack_user_id: string
): Promise<GetSlackUserMappingResponse> {
  return fetch(
    `/api/get_slack_user_mapping?slack_user_id=${encodeURIComponent(
      slack_user_id
    )}`
  ).then((res) => res.json());
}

export function GetNotificationForSlackChannel(
  slack_channel_id: string
): Promise<GetNotificationForSlackChannelResponse> {
  return fetch(
    `/api/notifications/slack?slack_channel_id=${encodeURIComponent(
      slack_channel_id
    )}`
  ).then((res) => res.json());
}

export function GetNotificationForOncall(
  oncall_id: string
): Promise<GetNotificationForOncallResponse> {
  return fetch(
    `/api/notifications/oncall?oncall_id=${encodeURIComponent(oncall_id)}`
  ).then((res) => res.json());
}

export function AddUserMap(
  slack_user_id: string,
  opsgenie_user_id: string
): Promise<AddUserMapResponse> {
  return fetch("/api/add_user_map", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      slack_id: slack_user_id,
      opsgenie_id: opsgenie_user_id,
    }),
  }).then((res) => res.json());
}

export function SyncedWith(oncall_id: string): Promise<SyncedWithResponse> {
  return fetch(
    `/api/synced_with?oncall_id=${encodeURIComponent(oncall_id)}`
  ).then((res) => res.json());
}

export function ListSyncs(): Promise<ListSyncsResponse> {
  return fetch("/api/list_syncs").then((res) => res.json());
}

export function AddSync(
  oncall_id: string,
  user_group_id: string
): Promise<AddSyncResponse> {
  return fetch("/api/add_sync", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      oncall_id,
      user_group_id,
    }),
  }).then((res) => res.json());
}

export function AddNotification(
  oncall_id: string,
  slack_channel_id: string
): Promise<AddNotificationResponse> {
  return fetch("/api/notifications/add", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      oncall_id,
      slack_channel_id,
    }),
  }).then((res) => res.json());
}

export function ListUserGroups(): Promise<ListUserGroupsResponse> {
  return fetch("/api/list_user_groups").then((res) => res.json());
}

export function RemoveUserMap(
  user_mapping_id: number
): Promise<RemoveUserMapResponse> {
  return fetch("/api/remove_user_map", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      user_mapping_id,
    }),
  }).then((res) => res.json());
}

export function RemoveSync(
  oncall_sync_id: number
): Promise<RemoveSyncResponse> {
  return fetch("/api/remove_sync", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      oncall_sync_id,
    }),
  }).then((res) => res.json());
}

export function RemoveNotification(
  notification_id: number
): Promise<RemoveNotificationResponse> {
  return fetch("/api/notifications/remove", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      notification_id,
    }),
  }).then((res) => res.json());
}
