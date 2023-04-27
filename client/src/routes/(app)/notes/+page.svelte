<script lang="ts">
    import Note from "./Note.svelte";
    import Button from "$lib/form/Button.svelte";
    import PlusIcon from "$lib/icons/PlusIcon.svelte";
    import NoteDrawer from "./NoteDrawer.svelte";
    import { writable } from "svelte/store";
    import { setContext } from "svelte";

    export let data;
    export let form;

    const drawer = writable(false);
    setContext("drawer", drawer);
</script>

<NoteDrawer {form} />

<h3 class="text-right">
    Rust: {data.duration.toFixed(4)}ms / {data.notes.length} notes
</h3>

<div class="mb-6 grid grid-cols-2 gap-4">
    <h3>Notes are only visible to you.</h3>
    <Button type="button" on:click={() => drawer.set(true)}>
        <span slot="icon"><PlusIcon /></span>
        Create note
    </Button>
</div>

{#each data.notes.splice(0, 10) as note}
    <Note noteId={note.id}>
        <span slot="title">{note.title}</span>
        <span slot="content">
            <p class="whitespace-pre-wrap">
                {@html note.content}
            </p>
        </span>
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
