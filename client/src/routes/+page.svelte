<script lang="ts">
    import { enhance } from "$app/forms";
import type { PageData } from "./$types";

    export let data: PageData;
</script>

<h1>Welcome to SvelteKit</h1>

<form action="?/createNote" method="post" use:enhance>
    <input type="text" name="title" placeholder="Title" />
    <input type="text" name="content" placeholder="Content" />
    <button type="submit">Create Note</button>
</form>

{#each data.notes as note}
    <div>
        <h2>{note.title}</h2>
        <p>{note.content}</p>
    </div>
{/each}
