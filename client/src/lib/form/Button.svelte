<script lang="ts">
    import LoadingComponent from "$lib/components/LoadingComponent.svelte";

    export let type: "button" | "submit" = "submit";
    export let loading = false;
</script>

<button
    on:click
    {type}
    disabled={loading}
    class="w-full h-10 flex flex-row gap-3 justify-center items-center rounded px-4 py-2 shadow-md font-semibold bg-primary-800 hover:opacity-80 transition text-gray-200"
>
    {#if loading}
        <div class="w-4 h-4 flex justify-center items-center">
            <LoadingComponent />
        </div>
    {:else if $$slots.icon}
        <div class="w-4 h-4 flex justify-center items-center">
            <slot name="icon" />
        </div>
    {/if}
    <slot />
</button>
