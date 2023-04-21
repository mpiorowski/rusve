<script lang="ts">
    export let type: "button" | "submit" = "button";
</script>

<button
    on:click
    {type}
    class="w-full h-10 flex flex-row gap-4 justify-center items-center rounded px-4 py-2 shadow-md font-semibold bg-primary-800 hover:opacity-80 transition text-gray-200"
>
    <slot />
</button>
