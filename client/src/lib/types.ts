import type { Writable } from "svelte/store";

export type Categories = ["Backend", "Frontend", "Deployment", "Additional"];
export type DrawerContext = Writable<boolean>;
