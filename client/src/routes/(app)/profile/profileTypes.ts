import type { User__Output } from "$lib/proto/proto/User";
import type { Writable } from "svelte/store";

export type ProfileStore = {
    user: User__Output;
};
export type ProfileContext = Writable<ProfileStore>;
