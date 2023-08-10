<script lang="ts">
    import Note from "./Note.svelte";
    import Button from "$lib/components/form/Button.svelte";
    import PlusIcon from "$lib/assets/icons/PlusIcon.svelte";
    import NoteDrawer from "./NoteDrawer.svelte";
    import { writable } from "svelte/store";
    import { setContext } from "svelte";
    import type { ActionData, PageData } from "./$types.js";
    import type { NoteContext } from "$lib/types";
    import EditIcon from "$lib/assets/icons/EditIcon.svelte";

    export let data: PageData;
    export let form: ActionData;

    const emptyNote = {
        id: "",
        title: "",
        content: "",
    };
    const drawer: NoteContext = writable({
        open: false,
        data: emptyNote,
    });
    setContext("drawer", drawer);
</script>

<NoteDrawer {form} />

<div class="mb-6 grid items-center gap-4">
    <h3>
        Query: {data.time.toFixed(4)}ms
    </h3>
    <h3>
        Mutation: {(form?.duration ?? 0).toFixed(4)}ms
    </h3>
    <Button
        type="button"
        on:click={() => drawer.set({ open: true, data: {...emptyNote} })}
    >
        <span slot="icon"><PlusIcon /></span>
        Create note
    </Button>
</div>

<h3>{data.length}x</h3>
{#each data.notes as note}
    <Note noteId={note.id}>
        <span slot="title">
            <div class="flex justify-between">
                {note.title}
                <button
                    class="h-5 w-5 text-primary-200 transition hover:text-primary-400"
                    aria-label="Edit note"
                    type="button"
                    on:click={() =>
                        drawer.set({
                            open: true,
                            data: {
                                id: note.id,
                                title: note.title,
                                content: note.content,
                            },
                        })}
                >
                    <EditIcon />
                </button>
            </div>
        </span>
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
            {:catch error}
                <p class="text-center text-3xl text-secondary-500">
                    {error.message}
                </p>
            {/await}
        </span>
    </Note>
{/each}
