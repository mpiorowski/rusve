<script lang="ts">
    import { enhance } from "$app/forms";
    import type { PageData } from "./$types";
    import Note from "../notes/Note.svelte";

    export let data: PageData;
</script>

<section class="p-4">
    <h1 class="text-center mb-4">Notes with Streamed Users one by one</h1>
    <h2 class="text-center">
        Notes loaded in {data.duration.toFixed(4)}ms
        <br />
        Count: {data.notes.length}
    </h2>

    {#each data.notes as note}
        <Note noteId={note.id}>
            <span slot="title">{note.title}</span>
            <span slot="content">{note.content}</span>
            <span slot="user">
                {#await data.streamed.users}
                    Loading...
                {:then users}
                    {#if users.find((u) => u.id === note.userId)}
                        {users.find((u) => u.id === note.userId)?.email}
                    {:else}
                        User not found
                    {/if}
                {/await}
            </span>
        </Note>
        <form
            class="flex flex-col"
            action="?/deleteNote"
            method="post"
            id={note.id}
            use:enhance
        >
            <input type="hidden" name="id" value={note.id} />
        </form>
    {/each}
</section>
