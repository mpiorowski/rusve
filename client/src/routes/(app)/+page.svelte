<script lang="ts">
    import { enhance } from "$app/forms";
    import { signOut } from "@auth/sveltekit/client";
    import type { PageData } from "./$types";

    export let data: PageData;

    async function onSignOut() {
        await signOut();
    }
</script>

<h1>Welcome to SvelteKit</h1>
<button on:click={onSignOut}>Sign Out</button>

<form action="?/createNote" method="post" use:enhance>
    <input type="text" name="title" placeholder="Title" />
    <input type="text" name="content" placeholder="Content" />
    <button type="submit">Create Note</button>
</form>

{#each data.notes as note}
    <form class="content" action="?/deleteNote" method="post" use:enhance>
        <h2>{note.title}</h2>
        <p>{note.content}</p>
        <input type="hidden" name="id" value={note.id} />
        <button type="submit">Delete Note</button>
    </form>
{/each}

<style>
    .content {
        display: flex;
        flex-direction: row;
        gap: 1rem;
        align-items: center;
        border: 1px solid black;
        padding: 0.25rem;
    }
</style>
