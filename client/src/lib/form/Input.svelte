<script lang="ts">
    export let value: string;
    export let name: string;
    export let type: "email" | "text" | "password" = "text";
    export let placeholder = "";
    export let errors: string[] = [];

    $: haveErrors = errors.length > 0;
    function typeAction(node: HTMLInputElement) {
        node.type = type;
    }
</script>

<div class="w-full">
    <input
        {name}
        use:typeAction
        {placeholder}
        bind:value
        class={`rounded px-4 py-2 bg-slate-600 text-gray-200 shadow-inner2 w-full transition ${
            haveErrors ? "ring-1 ring-red-500" : ""
        }`}
    />
    <p class="text-red-500 text-sm h-4 mt-1">{haveErrors ? errors[0] : ""}</p>
</div>
