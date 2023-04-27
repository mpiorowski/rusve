<script lang="ts">
    import Post from "./Post.svelte";
    import Button from "$lib/form/Button.svelte";
    import PlusIcon from "$lib/icons/PlusIcon.svelte";
    import PostDrawer from "./PostDrawer.svelte";
    import { writable } from "svelte/store";
    import { setContext } from "svelte";

    export let data;
    export let form;

    const drawer = writable(false);
    setContext("drawer", drawer);
</script>

<PostDrawer {form} />

<h3 class="text-right">
    Go: {data.duration.toFixed(4)}ms / {data.posts.length} posts
</h3>

<div class="mb-6 grid grid-cols-2 gap-4">
    <h3>
        Posts are visible to everyone. <br />
        Please be respectful.
    </h3>
    <Button type="button" on:click={() => drawer.set(true)}>
        <span slot="icon"><PlusIcon /></span>
        Create post
    </Button>
</div>

{#each data.posts.splice(0, 10) as post}
    <Post postId={post.id} canDelete={post.userId === data.userId}>
        <span slot="title">{post.title}</span>
        <span slot="content">
            <p class="whitespace-pre-wrap">
                {@html post.content}
            </p>
        </span>
        <span slot="user">
            {#await data.stream.users}
                <span class="block h-4" />
            {:then users}
                {#if users.find((u) => u.id === post.userId)}
                    {users.find((u) => u.id === post.userId)?.email}
                {:else}
                    User not found
                {/if}
            {/await}
        </span>
    </Post>
{/each}
