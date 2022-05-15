/**
 * @prettier
 */

import { atom } from "recoil";
import { Oncall } from "./Api";

// Atoms
export const userMappingState = atom<{
  [slack_name: string]: string;
}>({
  key: "userMappingState",
  default: {},
});

export const usersLoadedState = atom<boolean>({
  key: "usersLoadedState",
  default: false,
});

export const oncallCardState = atom<Oncall | null>({
  key: "oncallCardState",
  default: null,
});

export const oncallCardLoadingState = atom<boolean>({
  key: "oncallCardLoadingState",
  default: false,
});

export const oncallCardDeletingState = atom<boolean>({
  key: "oncallCardDeletingState",
  default: false,
});

export const oncallCardAddingState = atom<boolean>({
  key: "oncallCardAddingState",
  default: false,
});
