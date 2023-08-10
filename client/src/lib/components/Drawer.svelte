<script lang="ts">
    import Button from "$lib/components/form/Button.svelte";
    import XIcon from "$lib/assets/icons/XIcon.svelte";
    import { getContext } from "svelte";
    import { fade, slide } from "svelte/transition";
    import type { DrawerContext } from "$lib/types";

    const drawer = getContext<DrawerContext<unknown>>("drawer");
</script>

{#if $drawer.open}
    <div
        transition:fade={{ duration: 200 }}
        class="absolute right-0 top-0 z-40 h-screen w-screen bg-black bg-opacity-50"
        on:click={() => {
            drawer.set({
                open: false,
                data: "",
            });
        }}
        on:keypress={(e) => {
            if (e.key === "Escape") {
                drawer.set({
                    open: false,
                    data: "",
                });
            }
        }}
    />
    <div
        class="absolute right-0 top-0 z-50 grid h-[100dvh] w-full max-w-xl grid-rows-[60px_1fr_60px] bg-primary-700"
        transition:slide={{ duration: 200, axis: "x" }}
    >
        <div class="flex flex-row items-center justify-between px-6">
            <div>
                <slot name="header" />
            </div>
            <button
                on:click={() => {
                    drawer.set({
                        open: false,
                        data: "",
                    });
                }}
                class="flex h-8 w-8 items-center justify-center hover:text-primary-300"
            >
                <XIcon />
            </button>
        </div>
        <div>
            <slot name="content" />
        </div>
        <div class="flex flex-row items-center justify-end gap-4 px-6">
            <div class="w-28">
                <Button
                    type="button"
                    variant="secondary"
                    on:click={() => {
                        drawer.set({
                            open: false,
                            data: "",
                        });
                    }}
                >
                    Close
                </Button>
            </div>
            <slot name="footer" />
        </div>
    </div>
{/if}
