<script lang="ts">
    import { enhance } from "$app/forms";
    import DeleteIcon from "$lib/icons/DeleteIcon.svelte";
    import { toast } from "$lib/toast/toast";

    export let postId: string;
    export let canDelete: boolean;
</script>

<div class="flex flex-col gap-2 p-4 shadow-inner rounded bg-slate-600 mb-4">
    <h2>
        <slot name="title" />
    </h2>
    <slot name="content" />
    <div class="flex text-xs justify-between mt-4" role="list">
        <p>
            Created by:
            <br />
            <slot name="user" />
        </p>
        {#if canDelete}
            <form
                role="listitem"
                action="?/deletePost"
                method="post"
                id={postId}
                use:enhance={() => {
                    return async ({ result, update }) => {
                        await update();
                        if (result.type === "success") {
                            toast({
                                message: "Post deleted",
                                type: "success",
                            });
                        }
                    };
                }}
            >
                <input type="hidden" name="id" value={postId} />
                <button
                    type="submit"
                    form={postId}
                    class="h-5 w-5 text-error-500 hover:text-error-400 transition"
                    aria-label="Delete post"
                >
                    <DeleteIcon />
                </button>
            </form>
        {/if}
    </div>
</div>
