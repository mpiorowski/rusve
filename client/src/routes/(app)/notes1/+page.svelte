<script lang="ts">
    import { enhance } from "$app/forms";
    import type { PageData } from "./$types";

    export let data: PageData;
</script>

<h2 class="text-center">
    Notes loaded in {data.duration.toFixed(4)}ms
    Count: {data.notes.length}
</h2>

{#each data.notes as note}
    <form
        class="flex flex-col max-w-xl p-4"
        action="?/deleteNote"
        method="post"
        use:enhance
    >
        <pre>Note: {JSON.stringify(note, null, 2)}</pre>
        <input type="hidden" name="id" value={note.id} />
        <button
            type="submit"
            class="bg-gray-800 p-3 rounded hover:bg-gray-900 transition"
        >
            Delete Note
        </button>
    </form>
{/each}
