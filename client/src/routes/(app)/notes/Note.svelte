<script lang="ts">
    import { enhance } from "$app/forms";
    import ThrashIcon from "$lib/icons/ThrashIcon.svelte";

    export let noteId: string;
</script>

<form
    role="listitem"
    class="flex flex-col gap-2 p-4 shadow-inner rounded bg-slate-600 mb-4"
    action="?/deleteNote"
    method="post"
    id={noteId}
    use:enhance
>
    <input type="hidden" name="id" value={noteId} />
    <h2>
        <slot name="title" />
    </h2>
    <p>
        <slot name="content" />
    </p>
    <div class="flex text-xs justify-between mt-4">
        <p>
            Created by:
            <br />
            <slot name="user" />
        </p>
        <button type="submit" form={noteId}>
            <div class="h-5 w-5 text-error-500 hover:text-error-400 transition">
                <ThrashIcon />
            </div>
        </button>
    </div>
</form>
