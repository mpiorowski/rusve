<script>
    import Toast from "$lib/ui/Toast.svelte";
    import { toastStore } from "$lib/ui/toast";
    import { navigating } from "$app/stores";
    import "../app.css";
    import LoaderIcon from "$lib/icons/LoaderIcon.svelte";

    /** @type {boolean} */
    let isNavigating = false;
    /** @type {NodeJS.Timeout} */
    let t;
    $: if ($navigating) {
        t = setTimeout(() => {
            isNavigating = true;
        }, 1000);
    } else {
        clearTimeout(t);
        isNavigating = false;
    }
</script>

{#if isNavigating}
    <div
        class="fixed inset-0 z-50 flex items-center justify-center bg-black opacity-50"
    >
        <span class="sr-only">Loading...</span>
        <LoaderIcon />
    </div>
{/if}

<slot />

<div
    aria-live="assertive"
    class="pointer-events-none fixed inset-0 z-20 flex items-end px-4 py-6 sm:items-start sm:p-6"
>
    <div class="flex w-full flex-col items-center space-y-4 sm:items-end">
        {#each $toastStore as toast}
            <Toast {toast} />
        {/each}
    </div>
</div>
