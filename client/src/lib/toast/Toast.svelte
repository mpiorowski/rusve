<script lang="ts">
    import { fade } from "svelte/transition";
    import type { Toast } from "./toast";
    import { toastStore } from "./toast";
    export let toast: Toast;

    setTimeout(() => {
        // remove first toast
        toastStore.update((toasts) => {
            return toasts.slice(1);
        });
    }, 3000);
</script>

<div
    transition:fade={{ duration: 200 }}
    class={"bg-primary-500 border-l-4 py-2 px-4 rounded-md shadow-lg z-[1000] " +
        (toast.type === "success" ? "border-green-500" : "border-red-500")}
>
    {toast.message}
</div>
