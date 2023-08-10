<script lang="ts">
    import { enhance } from "$app/forms";
    import { page } from "$app/stores";
    import DeleteIcon from "$lib/assets/icons/DeleteIcon.svelte";
    import { toast } from "$lib/components/toast/toast";

    export let noteId: string;

    const type = $page.url.searchParams.get("lang") ?? "rust";
</script>

<div class="mb-4 flex flex-col gap-2 rounded bg-slate-600 p-4 shadow-inner">
    <h1>
        <slot name="title" />
    </h1>
    <div class="whitespace-pre-wrap">
        <slot name="content" />
    </div>
    <div class="mt-4 flex justify-between text-xs">
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
                        toast.success("Note deleted");
                    }
                };
            }}
        >
            <input type="hidden" name="id" value={noteId} />
            <input type="hidden" name="type" value={type} />
            <button
                type="submit"
                form={noteId}
                aria-label="Delete note"
                class="h-5 w-5 text-error-500 transition hover:text-error-400"
            >
                <DeleteIcon />
            </button>
        </form>
    </div>
</div>
