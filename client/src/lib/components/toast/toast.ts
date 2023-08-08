import { writable } from "svelte/store";

export type Toast = {
    message: string;
    type: "success" | "error";
};
export const toastStore = writable<Toast[]>([]);

export function showToast(t: Toast): void {
    toastStore.update((toasts) => [...toasts, t]);
}

export const toast = {
    success: (message: string) => showToast({ message, type: "success" }),
    error: (message: string) => showToast({ message, type: "error" }),
};
