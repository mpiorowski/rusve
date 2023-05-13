<script lang="ts">
    import Note from "./Note.svelte";
    import Button from "$lib/form/Button.svelte";
    import PlusIcon from "$lib/icons/PlusIcon.svelte";
    import NoteDrawer from "./NoteDrawer.svelte";
    import { writable } from "svelte/store";
    import { setContext } from "svelte";
    import type { ActionData, PageData } from "./$types.js";
    import type { DrawerContext } from "$lib/types";

    export let data: PageData;
    export let form: ActionData;

    /**
    * Over-engineered component composition
    * Local store, not shared with other components
    */
    const drawer: DrawerContext = writable({ open: false, data: "" });
    setContext("drawer", drawer);
</script>

<NoteDrawer {form} />

<div class="mb-6 grid items-center gap-4">
    <h3>
        Rust: {data.timeRust.toFixed(4)}ms
    </h3>
    <h3>
        Go: {data.timeGo.toFixed(4)}ms
    </h3>
    <h3>
        Created in: {(form?.duration ?? 0).toFixed(4)}ms
    </h3>
    <Button
        type="button"
        on:click={() => drawer.set({ open: true, data: "rust" })}
    >
        <span slot="icon"><PlusIcon /></span>
        Re-create 5000 notes using Rust
    </Button>
    <Button
        type="button"
        on:click={() => drawer.set({ open: true, data: "go" })}
    >
        <span slot="icon"><PlusIcon /></span>
        Re-create 5000 notes using Go
    </Button>
</div>

{#each data.notesRust.slice(0, 10) as note}
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
