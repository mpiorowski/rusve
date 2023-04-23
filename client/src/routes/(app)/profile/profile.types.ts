import type { User__Output } from "$lib/proto/proto/User";
import { writable, type Writable } from "svelte/store";

export type ProfileStore = {
    user: User__Output;
    file: Promise<
        | {
              id: string;
              name: string;
              data: string;
          }
        | undefined
    >;
};
export type ProfileContext = Writable<ProfileStore>;

export const avatarStore =
    writable<Promise<{ name: string; data: string } | undefined>>(undefined);
