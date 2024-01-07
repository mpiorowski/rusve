<script>
    import { enhance } from "$app/forms";
    import Button from "$lib/form/Button.svelte";
    import FileInput from "$lib/form/FileInput.svelte";
    import Pagination from "$lib/ui/Pagination.svelte";
    import UploadIcon from "$lib/icons/UploadIcon.svelte";
    import { toast } from "$lib/ui/toast";

    /** @type {import("./$types").PageData} */
    export let data;

    /** @type {import("./$types").ActionData} */
    export let form;
    $: if (form?.error || data.error) {
        toast.error("Error", form?.error || data.error || "Unknown error");
    }

    let loading = false;

    /** @type {File} */
    let newFile = new File([], "");

    $: if (form?.fileBuffer) {
        download(form.fileBuffer, form.fileName, form.fileType);
    }

    /**
     * Download a base64 encoded file
     * @param {number[]} file_buffer
     * @param {string} file_name
     * @param {string} file_type
     * @returns {Promise<void>}
     */
    async function download(file_buffer, file_name, file_type) {
        try {
            const blob = new Blob([new Uint8Array(file_buffer)], {
                type: file_type,
            });
            const url = URL.createObjectURL(blob);
            const link = document.createElement("a");
            link.href = url;
            link.download = file_name;
            document.body.appendChild(link);
            link.click();
            document.body.removeChild(link);
        } catch (e) {
            console.error(e);
        }
    }
</script>

<form
    class="max-w-2xl"
    method="post"
    action="?/uploadFile"
    enctype="multipart/form-data"
    use:enhance={() => {
        const timeout = setTimeout(() => {
            loading = true;
        }, 100);
        return async ({ result, update }) => {
            if (result.type === "success") {
                toast.success("Success", "Your file has been uploaded.");
            }
            clearTimeout(timeout);
            loading = false;
            await update({
                reset: false,
            });
        };
    }}
>
    <div class="space-y-12">
        <div>
            <h2
                class="flex items-center gap-2 text-base font-semibold leading-7 text-gray-50"
            >
                New file
            </h2>
            <p class="mt-1 text-sm leading-6 text-gray-200">
                Upload a new file.
            </p>
        </div>
        <FileInput
            name="file"
            label="File"
            accept="image/*"
            bind:file={newFile}
        />
        <div class="flex justify-end">
            <Button {loading}>
                {#if !loading}
                    <UploadIcon />
                {/if}
                Upload
            </Button>
        </div>
    </div>
</form>

<div class="mt-10 sm:flex sm:items-center">
    <div class="sm:flex-auto">
        <h1 class="text-base font-semibold leading-6 text-gray-50">Files</h1>
        <p class="mt-2 text-sm leading-6 text-gray-200">
            List of files uploaded to the server.
        </p>
    </div>
</div>
<div class="mt-8 flow-root max-w-7xl">
    <div class="overflow-x-auto overflow-y-hidden">
        <div class="inline-block min-w-full align-middle">
            <table class="min-w-full divide-y divide-gray-600">
                <thead>
                    <tr>
                        <th
                            scope="col"
                            class="py-3 pl-4 pr-3 text-left text-xs uppercase tracking-wide text-gray-500 sm:pl-0"
                        >
                            Name
                        </th>
                        <th
                            scope="col"
                            class="px-3 py-3 text-left text-xs uppercase tracking-wide text-gray-500"
                        >
                            Size
                        </th>
                        <th
                            scope="col"
                            class="px-3 py-3 text-left text-xs uppercase tracking-wide text-gray-500"
                        >
                            Type
                        </th>
                        <th
                            scope="col"
                            class="px-3 py-3 text-left text-xs uppercase tracking-wide text-gray-500"
                        >
                            Created
                        </th>
                        <th
                            scope="col"
                            class="px-3 py-3 text-left text-xs uppercase tracking-wide text-gray-500"
                        >
                            Updated
                        </th>
                        <th scope="col" class="relative py-3 pl-3 pr-4 sm:pr-0">
                            <span class="sr-only">Edit</span>
                        </th>
                    </tr>
                </thead>
                <tbody class="divide-y divide-gray-600 bg-gray-900">
                    {#each data.files as file}
                        <tr>
                            <td
                                class="whitespace-nowrap py-4 pl-4 pr-3 text-sm font-medium text-gray-50 sm:pl-0"
                            >
                                {file.file_name}
                            </td>
                            <td
                                class="whitespace-nowrap px-3 py-4 text-sm text-gray-200"
                            >
                                {file.file_size}
                            </td>
                            <td
                                class="whitespace-nowrap px-3 py-4 text-sm text-gray-200"
                            >
                                {file.file_type}
                            </td>
                            <td
                                class="whitespace-nowrap px-3 py-4 text-sm text-gray-200"
                            >
                                {file.created}
                            </td>
                            <td
                                class="whitespace-nowrap px-3 py-4 text-sm text-gray-200"
                            >
                                {file.updated}
                            </td>
                            <td
                                class="relative whitespace-nowrap py-4 pl-3 pr-4 text-right text-sm font-medium sm:pr-0"
                            >
                                <form action="?/downloadFile" method="post" use:enhance>
                                    <input
                                        type="hidden"
                                        name="id"
                                        value={file.id}
                                    />
                                    <button
                                        class="mr-4 text-indigo-600 hover:text-indigo-900"
                                    >
                                        Download
                                        <span class="sr-only">
                                            , {file.file_name}
                                        </span>
                                    </button>
                                </form>
                            </td>
                        </tr>
                    {/each}
                </tbody>
            </table>
        </div>
    </div>

    <!-- Pagination -->
    <Pagination total={data.total} pageSize={data.pageSize} />
</div>
