import { toast, ToastType } from "@mpiorowski/svelte-init";

export const toastError = (error: string) => {
    toast(error, ToastType.ERROR);
};
