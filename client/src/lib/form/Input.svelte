<script lang="ts">
    export let value: string;
    export let name: string;
    export let type: "email" | "text" | "password" = "text";
    export let label = "";
    export let placeholder = "";
    export let errors: string[] = [];
    export let textarea = false;

    $: haveErrors = errors.length > 0;
    function typeAction(node: HTMLInputElement) {
        node.type = type;
    }
</script>

<div class="w-full flex flex-col">
    {#if label}
        <label class="text-gray-200 h-5 flex items-center" for={name}>
            {label}
        </label>
    {/if}
    {#if !textarea}
        <input
            id={name}
            {name}
            use:typeAction
            {placeholder}
            bind:value
            class={`rounded px-4 py-2 bg-slate-600 border-primary-700 focus:ring-1 focus:ring-slate-800 shadow-inner w-full ${
                haveErrors ? "ring-1 ring-error-500" : ""
            }`}
        />
    {:else}
        <textarea
            id={name}
            {name}
            rows="5"
            {placeholder}
            bind:value
            class={`rounded px-4 py-2 bg-slate-600 border-primary-700 focus:ring-1 focus:ring-slate-800 shadow-inner w-full ${
                haveErrors ? "ring-1 ring-error-500" : ""
            }`}
        />
    {/if}

    <p class="text-error-500 text-sm h-5 flex items-baseline">
        {haveErrors ? errors[0] : ""}
    </p>
</div>
