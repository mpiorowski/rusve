// See https://kit.svelte.dev/docs/types#app

import type { User__Output } from "$lib/proto/proto/User";
import type { Profile__Output } from "$lib/proto/proto/Profile";
import type { Note__Output } from "$lib/proto/proto/Note";

// for information about these interfaces
declare global {
    namespace App {
        // interface Error {}
        interface Locals {
            token: string;
            user: User__Output;
        }
        interface PageState {
            open: boolean;
            noteDrawer: { email: string; avatar: string; profile: Profile__Output, note: Note__Output };
        }
        // interface PageData {}
        // interface Platform {}
    }
}

export {};
