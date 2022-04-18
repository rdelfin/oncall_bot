/**
 * @prettier
 */

const api_endpoint = "http://localhost:8080";

export interface SlackUser {
  id: string;
  name: string;
  real_name: string;
  is_bot: boolean;
}

export interface OpsgenieUser {
  id: string;
  username: string;
  fullName: string;
}

export interface UserMapping {
  opsgenie_user_id: string;
  slack_user_id: string;
}

export interface ListSlackUsersResponse {
  users?: SlackUser[];
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

export function ListOpsgenieUsers(): Promise<ListOpsgenieUsersResponse> {
  return fetch(`${api_endpoint}/list_opsgenie_users`).then((res) => res.json());
}

export function ListSlackUsers(): Promise<ListSlackUsersResponse> {
  return fetch(`${api_endpoint}/list_slack_users`).then((res) => res.json());
}

export function ListUserMappings(): Promise<UserMapping> {
  return fetch(`${api_endpoint}/list_user_mappings`).then((res) => res.json());
}
