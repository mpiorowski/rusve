<script>
    /** @type {string} */
    export let name;
    /** @type {string} */
    export let label;
    /** @type {File | undefined} */
    export let file;
    /** @type {string | undefined} */
    export let accept = "*/*";
    /** @type {string[]} */
    export let errors = [];
    /** @type {string} */
    export let helper = "\x80";

    /** @type {FileList} */
    let files;
    $: if (files && files[0]) {
        file = files[0];
    } else {
        file = undefined;
    }
</script>

<div>
    <label for={name} class="block text-sm font-medium leading-6">
        {label}
    </label>
    <div class="mt-2">
        <input
            bind:files
            {name}
            id={name}
            {accept}
            type="file"
            class="block w-full cursor-pointer rounded-lg border-0 bg-gray-800 text-gray-50 shadow-inset
            file:mr-3 file:cursor-pointer file:rounded-l-lg file:border-0 file:bg-indigo-600 file:px-3 file:py-1.5 file:text-white file:transition
            hover:bg-gray-700/25 file:hover:bg-indigo-700
            focus:outline-none focus:ring-2 focus:ring-inset focus:ring-indigo-600
            sm:text-sm sm:leading-6
            {errors.length > 0 ? 'ring-2 ring-red-600' : ''}"
            aria-invalid={errors.length > 0}
            aria-describedby="{name}-description"
        />
    </div>
    <p
        id="{name}-description"
        class="text-sm leading-6
        {errors.length > 0 ? 'text-red-600' : 'text-gray-500'}"
    >
        {errors.length > 0 ? errors.join(", ") : helper}
    </p>
</div>
