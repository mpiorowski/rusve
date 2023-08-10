import type { Writable } from "svelte/store";

export enum Categories {
    Backend = "Backend",
    Frontend = "Frontend",
    Deployment = "Deployment",
    Additional = "Additional",
}
export type DrawerContext<T> = Writable<{
    open: boolean;
    data: T;
}>;
export type NoteContext = Writable<{
    open: boolean;
    data: {
        id: string;
        title: string;
        content: string;
    };
}>;
