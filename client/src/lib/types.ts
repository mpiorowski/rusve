import type { Writable } from "svelte/store";

export enum Categories {
    Backend = "Backend",
    Frontend = "Frontend",
    Deployment = "Deployment",
    Additional = "Additional",
}
export type DrawerContext = Writable<boolean>;
