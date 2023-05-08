<script lang="ts">
    import { enhance } from "$app/forms";
    import DeleteIcon from "$lib/icons/DeleteIcon.svelte";
    import { toast } from "$lib/toast/toast";

    export let noteId: string;
</script>

<div class="flex flex-col gap-2 p-4 shadow-inner rounded bg-slate-600 mb-4">
    <h1>
        <slot name="title" />
    </h1>
    <div class="whitespace-pre-wrap">
        <slot name="content" />
    </div>
    <div class="flex text-xs justify-between mt-4">
        <p>
            Created by:
            <br />
            <slot name="user" />
        </p>
        <form
            action="?/deleteNote"
            method="post"
            id={noteId}
            use:enhance={() => {
                return async ({ result, update }) => {
                    await update();
                    if (result.type === "success") {
                        toast({
                            message: "Note deleted",
                            type: "success",
                        });
                    }
                };
            }}
        >
            <input type="hidden" name="id" value={noteId} />
            <button
                type="submit"
                form={noteId}
                aria-label="Delete note"
                class="h-5 w-5 text-error-500 hover:text-error-400 transition"
            >
                <DeleteIcon />
            </button>
        </form>
    </div>
</div>
