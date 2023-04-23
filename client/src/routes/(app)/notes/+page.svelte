<script lang="ts">
    import Note from "./Note.svelte";
    import Button from "$lib/form/Button.svelte";
    import PlusIcon from "$lib/icons/PlusIcon.svelte";
    import NoteDrawer from "./NoteDrawer.svelte";

    export let data;
    export let form;

    let open = false;
</script>

<NoteDrawer {form} on:clickOutside={() => (open = false)} {open} />

<h3 class="text-right">
    {data.duration.toFixed(4)}ms
</h3>

<div class="mb-6">
    <Button type="button" on:click={() => (open = true)}>
        <span slot="icon"><PlusIcon /></span>
        Create note
    </Button>
</div>

{#each data.notes as note}
    <Note noteId={note.id}>
        <span slot="title">{note.title}</span>
        <span slot="content">
            <p class="whitespace-pre-wrap">
                {@html note.content}
            </p>
        </span>
        <span slot="user">
            {#await data.streamed.users}
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
