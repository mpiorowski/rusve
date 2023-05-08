<script lang="ts">
    import Note from "./Note.svelte";
    import Button from "$lib/form/Button.svelte";
    import PlusIcon from "$lib/icons/PlusIcon.svelte";
    import NoteDrawer from "./NoteDrawer.svelte";
    import { writable } from "svelte/store";
    import { setContext } from "svelte";
    import type { ActionData, PageData } from "./$types.js";

    export let data: PageData;
    export let form: ActionData;

    const drawer = writable(false);
    setContext("drawer", drawer);
</script>

<NoteDrawer {form} />

<div class="mb-6 grid grid-cols-2 items-center gap-4">
    <div>
        <h3>Notes are only visible to you.</h3>
        <h3>
            Rust: {data.duration.toFixed(4)}ms / {data.notes.length} notes
        </h3>
    </div>
    <Button type="button" on:click={() => drawer.set(true)}>
        <span slot="icon"><PlusIcon /></span>
        Create note
    </Button>
</div>

{#each data.notes as note}
    <Note noteId={note.id}>
        <span slot="title">{note.title}</span>
        <!-- TODO -->
        <!-- eslint-disable-next-line svelte/no-at-html-tags -->
        <span slot="content">{@html note.content}</span>
        <span slot="user">
            {#await data.stream.users}
                <span class="block h-4" />
            {:then users}
                {#if users.find((u) => u.id === note.userId)}
                    {users.find((u) => u.id === note.userId)?.email}
                {:else}
                    User not found
                {/if}
            {/await}
        </span>
    </Note>
{/each}
