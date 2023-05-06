<script lang="ts">
    /** eslint-disable @typescript-eslint/no-unsafe-member-access */

    import LoadingComponent from "$lib/components/LoadingComponent.svelte";

    export let type: "button" | "submit" = "submit";
    export let loading = false;
    export let variant: "primary" | "secondary" | "error" = "primary";
    export let form: string | undefined = undefined;
    export let href = "";

    let className =
        "w-full h-10 flex flex-row gap-3 justify-center items-center rounded px-4 shadow-md font-normal text-base text-white transition";
    // TODO: fix
    // eslint-disable-next-line @typescript-eslint/no-unnecessary-condition
    if (variant === "primary") {
        className += " bg-secondary-700 hover:bg-secondary-800";
    } else if (variant === "secondary") {
        className += " bg-primary-800 hover:bg-primary-900";
    } else if (variant === "error") {
        className += " bg-error-600";
    }
</script>

{#if href}
    <a {href} class={className}>
        <slot />
    </a>
{:else}
    <button {form} on:click {type} disabled={loading} class={className}>
        {#if loading}
            <div class="w-4 h-4 flex justify-center items-center">
                <LoadingComponent />
            </div>
        {:else if // eslint-disable-next-line @typescript-eslint/no-unsafe-member-access
        $$slots.icon}
            <div class="w-6 h-6 flex justify-center items-center">
                <slot name="icon" />
            </div>
        {/if}
        <slot />
    </button>
{/if}
