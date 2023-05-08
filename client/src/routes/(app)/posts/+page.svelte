<script lang="ts">
    import Post from "./Post.svelte";
    import Button from "$lib/form/Button.svelte";
    import PlusIcon from "$lib/icons/PlusIcon.svelte";
    import PostDrawer from "./PostDrawer.svelte";
    import { writable } from "svelte/store";
    import { setContext } from "svelte";
    import type { ActionData, PageData } from "./$types.js";

    export let data: PageData;
    export let form: ActionData;

    const drawer = writable(false);
    setContext("drawer", drawer);
</script>

{#if !data.isSubscribed}
    <h2 class="text-center">
        You are currently on the "Noob" plan. <br />
        Please upgrade to the "I use Rust" plan to create posts.
    </h2>
    <p class="text-center">
        test mode - no real payment will be made <br />
        use (4242 4242 ....) as credit card number
    </p>
    <div class="mt-4 w-fit ml-auto">
        <Button href="/billing" variant="secondary">Go to billing</Button>
    </div>
{:else}
    <PostDrawer {form} />

    <div class="mb-6 grid grid-cols-2 gap-4 items-center">
        <div>
            <h3>
                Posts are visible to everyone. <br />
                Please be respectful.
            </h3>
            <h3>
                Go: {data.duration.toFixed(4)}ms / {data.posts.length} posts
            </h3>
        </div>
        <Button type="button" on:click={() => drawer.set(true)}>
            <span slot="icon"><PlusIcon /></span>
            Create post
        </Button>
    </div>

    {#each data.posts as post}
        <Post postId={post.id} canDelete={post.userId === data.userId}>
            <span slot="title">{post.title}</span>
            <span slot="content">
                <!-- TODO -->
                <!-- eslint-disable-next-line svelte/no-at-html-tags -->
                {@html post.content}
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
{/if}
