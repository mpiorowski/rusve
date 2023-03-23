<script lang="ts">
    import { FileType } from "$lib/proto/proto/FileType";
    import type { ActionData, PageData } from "./$types";

    export let data: PageData;
    export let form: ActionData;
</script>

{#if form?.error}
    <h2 class="text-center text-red-700">{form.error}</h2>
{/if}

<h2 class="text-center">
    Files loaded in {data.duration.toFixed(4)}ms
</h2>
{#if form}
    <h2 class="text-center">
        Files created or deleted in {form.duration?.toFixed(4)}ms
    </h2>
{/if}

<form
    action="?/createFile"
    method="post"
    enctype="multipart/form-data"
    class="flex flex-col gap-2 p-4"
>
    <input class="bg-gray-800 p-3 rounded" type="file" name="file" />
    <input type="hidden" name="type" value={FileType.DOCUMENT} />
    <input type="hidden" name="targetId" value={data.userId} />
    <button
        type="submit"
        class="bg-teal-700 p-3 rounded hover:bg-teal-600 transition"
    >
        Upload
    </button>
</form>

{#if data.files}
    <ul class="flex flex-col gap-2 my-2">
        {#each data.files as file}
            <li class="flex flex-row gap-2 items-center">
                <div>{file.name}</div>
                <form
                    action="/api"
                    method="post"
                    class="flex flex-row gap-2"
                >
                    <input type="hidden" name="base64" value={file.data} />
                    <input type="hidden" name="name" value={file.name} />
                    <button
                        type="submit"
                        class="bg-green-700 p-3 rounded hover:bg-green-600 transition"
                    >
                        Download
                    </button>
                </form>
                <form
                    action="?/deleteFile"
                    method="post"
                    class="flex flex-row gap-2"
                >
                    <input type="hidden" name="fileId" value={file.id} />
                    <input type="hidden" name="targetId" value={data.userId} />
                    <button
                        type="submit"
                        class="bg-red-700 p-3 rounded hover:bg-red-600 transition"
                    >
                        Delete
                    </button>
                </form>
            </li>
        {/each}
    </ul>
{/if}
