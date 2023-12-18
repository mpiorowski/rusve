<script>
    /** @type {string} */
    export let name;
    /** @type {string} */
    export let label;
    /** @type {string | number} */
    export let value;
    /** @type {"text" | "email" | "password"} */
    export let type = "text";
    /** @type {string} */
    export let placeholder = "";
    /** @type {string} */
    export let autocomplete = "off";
    /** @type {number} */
    export let rows = 0;
    /** @type {string} */
    export let error = "";
    /** @type {string} */
    export let helper = "\x80";

    let className = "";
    export { className as class };

    /**
     * @param {HTMLInputElement} node
     * @returns {void}
     */
    function typeAction(node) {
        node.type = type;
    }
</script>

<div class={className}>
    <label for={name} class="block text-sm font-medium leading-6">
        {label}
    </label>
    <div class="mt-2">
        {#if rows === 0}
            <input
                use:typeAction
                bind:value
                id={name}
                {name}
                {placeholder}
                {autocomplete}
                class="inner3 block w-full rounded-md border-0 bg-gray-800 px-3 py-1.5 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6
                {error ? 'ring-2 ring-red-600' : ''}"
                aria-invalid={!!error}
                aria-describedby="{name}-description"
            />
        {:else}
            <textarea
                bind:value
                id={name}
                {name}
                {placeholder}
                {rows}
                class="inner3 block w-full rounded-md border-0 bg-gray-800 px-3 py-1.5 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6
                {error ? 'ring-2 ring-red-600' : ''}"
                aria-invalid={!!error}
                aria-describedby="{name}-description"
            />
        {/if}
    </div>
    <p
        id="{name}-description"
        class="mb-2 mt-0 text-xs leading-6
        {error ? 'text-red-600' : 'text-gray-500'}"
    >
        {error || helper}
    </p>
</div>

<style>
    .inner-shadow {
        box-shadow:
            inset 0px 0px 3px 3px rgba(0, 0, 0, 0.3),
            0 -2px 0 hsla(0, 0%, 100%, 0.1);
    }
    .inner2 {
        box-shadow:
            inset 0 2px 2px 2px hsla(0, 0%, 0%, 0.3),
            0 1px 0 hsla(0, 0%, 100%, 0.1);
    }
    .inner3 {
        box-shadow:
            inset 0 2px 2px 0px hsla(0, 0%, 0%, 0.4),
            inset 0 0px 2px 2px hsla(0, 0%, 0%, 0.3);
    }
</style>
