import type { Writable } from "svelte/store";

export type ProfileContext = Writable<{
    id: string;
    created: string;
    updated: string;
    deleted?: string;
    email: string;
    role: string;
    sub: string;
    name: string;
    avatarId?: string;
    paymentId?: string;
}>;
