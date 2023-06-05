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

    const drawer: DrawerContext = writable(false);
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
    <Button type="button" on:click={() => drawer.set(true)}>
        <span slot="icon"><PlusIcon /></span>
        Re-create 5000 notes
    </Button>
</div>

<h3>{data.length}x</h3>
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
            {:catch error}
                <p class="text-3xl text-center text-secondary-500">
                    {error.message}
                </p>
            {/await}
        </span>
    </Note>
{/each}
