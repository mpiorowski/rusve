import { writable } from "svelte/store";

export type Toast = {
    message: string;
    type: "success" | "error";
};
export const toastStore = writable<Toast[]>([]);

export function toast(t: Toast) {
    toastStore.update((toasts) => [...toasts, t]);
}
