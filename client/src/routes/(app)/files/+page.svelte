<script lang="ts">
    import type { ActionData, PageData } from "./$types";

    export let data: PageData;
    export let form: ActionData;
</script>

<h1>Files</h1>

{#if form}
    <h2>Uploaded {form.file}</h2>
{/if}

<form
    action="?/createFile"
    method="post"
    enctype="multipart/form-data"
    class="flex flex-col gap-2"
>
    <input class="bg-gray-800 p-3 rounded" type="file" name="file" />
    <button
        type="submit"
        class="bg-teal-700 p-3 rounded hover:bg-teal-600 transition"
    >
        Upload
    </button>
</form>
