/**
 * @prettier
 */

import { atom } from "recoil";

// Atoms
export const userMappingState = atom<{
  [slack_name: string]: string;
}>({
  key: "userMappingState", // unique ID (with respect to other atoms/selectors)
  default: {}, // default value (aka initial value)
});

export const usersLoadedState = atom<boolean>({
  key: "usersLoadedState",
  default: false,
});
