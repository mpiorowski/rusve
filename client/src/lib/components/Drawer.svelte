<script lang="ts">
    import Button from "$lib/form/Button.svelte";
    import XIcon from "$lib/icons/XIcon.svelte";
    import { createEventDispatcher } from "svelte";
    import { fade, slide } from "svelte/transition";

    export let open: boolean;

    const dispatch = createEventDispatcher();
    function clickOutside() {
        dispatch("clickOutside");
    }
</script>

{#if open}
    <div
        transition:fade={{ duration: 200 }}
        class="absolute top-0 right-0 w-screen h-screen bg-black bg-opacity-50 z-40"
        on:click={clickOutside}
        on:keypress={(e) => {
            if (e.key === "Escape") {
                clickOutside();
            }
        }}
    />
    <div
        class="absolute grid grid-rows-[60px_1fr_60px] top-0 right-0 h-screen max-w-xl w-full bg-primary-700 z-50"
        transition:slide={{ duration: 200, axis: "x" }}
    >
        <div class="flex flex-row justify-between items-center px-6">
            <div>
                <slot name="header" />
            </div>
            <button
                on:click={clickOutside}
                class="w-8 h-8 flex justify-center items-center hover:text-primary-300"
            >
                <XIcon />
            </button>
        </div>
        <div>
            <slot name="content" />
        </div>
        <div class="flex flex-row justify-end items-center px-6 gap-4">
            <div class="w-28">
                <Button
                    type="button"
                    variant="secondary"
                    on:click={clickOutside}
                >
                    Close
                </Button>
            </div>
            <slot name="footer" />
        </div>
    </div>
{/if}
