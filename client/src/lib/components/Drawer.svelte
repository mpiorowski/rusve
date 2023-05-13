<script lang="ts">
    import Button from "$lib/form/Button.svelte";
    import XIcon from "$lib/icons/XIcon.svelte";
    import { getContext } from "svelte";
    import { fade, slide } from "svelte/transition";
    import type { DrawerContext } from "$lib/types";

    const drawer = getContext<DrawerContext>("drawer");
</script>

{#if $drawer.open}
    <div
        transition:fade={{ duration: 200 }}
        class="absolute top-0 right-0 w-screen h-screen bg-black bg-opacity-50 z-40"
        on:click={() => {
            drawer.set({ ...$drawer, open: false });
        }}
        on:keypress={(e) => {
            if (e.key === "Escape") {
                drawer.set({ ...$drawer, open: false });
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
                on:click={() => {
                    drawer.set({ ...$drawer, open: false });
                }}
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
                    on:click={() => {
                        drawer.set({ ...$drawer, open: false });
                    }}
                >
                    Close
                </Button>
            </div>
            <slot name="footer" />
        </div>
    </div>
{/if}
