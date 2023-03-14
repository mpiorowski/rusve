<script lang="ts">
    import { enhance } from "$app/forms";
    import type { ActionData, PageData } from "./$types";

    export let data: PageData;
    export let form: ActionData;

    let title = "Notes";
    let content = "This is my note";
</script>

<h2 class="text-center">
    Notes loaded in {data.duration.toFixed(4)}ms
    Count: {data.notes.length}
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
>
    <input
        class="bg-gray-800 p-3 rounded ring-2 ring-gray-800 hover:ring-teal-700 transition"
        type="text"
        name="title"
        placeholder="Title"
        bind:value={title}
    />
    <input
        class="bg-gray-800 p-3 rounded ring-2 ring-gray-800 hover:ring-teal-700 transition"
        type="text"
        name="content"
        placeholder="Content"
        bind:value={content}
    />
    <button
        class="bg-teal-700 p-3 rounded hover:bg-teal-600 transition"
        type="submit"
    >
        Create 100x Notes
    </button>
</form>

{#each data.notes as note}
    <form
        class="flex flex-col max-w-xl p-4"
        action="?/deleteNote"
        method="post"
        use:enhance
    >
        <pre>Note: {JSON.stringify(note, null, 2)}</pre>
        {#await data.streamed.users}
            <p>Loading user...</p>
        {:then users}
            <p>User: {JSON.stringify(users.find((u) => u.id === note.userId), null, 2)}</p>
        {/await}
        <input type="hidden" name="id" value={note.id} />
        <button
            type="submit"
            class="bg-gray-800 p-3 rounded hover:bg-gray-900 transition"
        >
            Delete Note
        </button>
    </form>
{/each}
