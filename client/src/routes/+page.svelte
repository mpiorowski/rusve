<script lang="ts">
    import { enhance } from "$app/forms";
    import type { ActionData, PageData } from "./$types";

    export let data: PageData;
    export let form: ActionData;
</script>

<h2 class="text-center">
    Notes loaded in {data.duration.toFixed(4)}ms
</h2>
{#if form}
    <h2 class="text-center">
        Notes created or deleted in {form.duration.toFixed(4)}ms
    </h2>
{/if}

<form
    class="flex flex-col max-w-xl p-4 gap-4"
    action="?/createNote"
    method="post"
    use:enhance
>
    <input
        class="bg-gray-800 p-3 rounded ring-2 ring-gray-800 hover:ring-teal-700 transition"
        type="text"
        name="title"
        placeholder="Title"
    />
    <input
        class="bg-gray-800 p-3 rounded ring-2 ring-gray-800 hover:ring-teal-700 transition"
        type="text"
        name="content"
        placeholder="Content"
    />
    <button
        class="bg-teal-700 p-3 rounded hover:bg-teal-600 transition"
        type="submit"
    >
        Create Note
    </button>
</form>

{#each data.notes as note}
    <form
        class="flex flex-col max-w-xl p-4"
        action="?/deleteNote"
        method="post"
        use:enhance
    >
        <pre>{JSON.stringify(note, null, 2)}</pre>
        <input type="hidden" name="id" value={note.id} />
        <button
            type="submit"
            class="bg-gray-800 p-3 rounded hover:bg-gray-900 transition"
        >
            Delete Note
        </button>
    </form>
{/each}
