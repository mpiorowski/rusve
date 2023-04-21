<script lang="ts">
    import { enhance } from "$app/forms";
    import Note from "./Note.svelte";
    import Button from "$lib/form/Button.svelte";
    import Input from "$lib/form/Input.svelte";

    export let data;
    export let form;

    let title = "";
    let content = "";
</script>

<form class="flex flex-col" action="?/createNote" method="post" use:enhance>
    <Input
        name="title"
        bind:value={title}
        label="Title"
        errors={form?.error?.fieldErrors.title ?? []}
    />
    <Input
        name="content"
        bind:value={content}
        label="Content"
        errors={form?.error?.fieldErrors.content ?? []}
    />
    <Button type="submit">Create note</Button>
</form>

<h3 class="text-center">
    Loaded in {data.duration.toFixed(4)}ms ({data.notes.length})
</h3>

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
{/each}
