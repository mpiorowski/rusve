<script lang="ts">
    import LoadingComponent from "$lib/components/LoadingComponent.svelte";

    export let type: "button" | "submit" = "submit";
    export let loading = false;
    export let variant: "primary" | "secondary" | "error" = "primary";
    export let form: string | undefined = undefined;
    export let href = "";

    let className =
        "w-full h-10 flex flex-row gap-3 justify-center items-center rounded px-4 shadow-md font-normal text-base text-white transition" +
        (variant === "primary"
            ? " bg-secondary-700 hover:bg-secondary-800"
            : "") +
        (variant === "secondary"
            ? " bg-primary-800 hover:bg-primary-900"
            : "") +
        (variant === "error" ? " bg-error-600" : "");
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
        {:else if $$slots.icon}
            <div class="w-6 h-6 flex justify-center items-center">
                <slot name="icon" />
            </div>
        {/if}
        <slot />
    </button>
{/if}
