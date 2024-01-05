<script>
    import { enhance } from "$app/forms";
    import { extractError } from "$lib/errors";
    import Button from "$lib/form/Button.svelte";
    import FileInput from "$lib/form/FileInput.svelte";
    import Pagination from "$lib/ui/Pagination.svelte";
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
    const newFile = new File([], "");
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
            <p class="mt-1 text-sm leading-6 text-gray-200">Upload a new file.</p>
        </div>
        <FileInput
            name="file"
            label="File"
            accept="image/*"
            bind:file={newFile}
         />
        <Button {loading}>Send</Button>
    </div>
</form>

<div class="mt-10 sm:flex sm:items-center">
    <div class="sm:flex-auto">
        <h1 class="text-base font-semibold leading-6 text-gray-50">Emails</h1>
        <p class="mt-2 text-sm leading-6 text-gray-200">
            List of emails You have sent.
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
                            To
                        </th>
                        <th
                            scope="col"
                            class="px-3 py-3 text-left text-xs uppercase tracking-wide text-gray-500"
                        >
                            From
                        </th>
                        <th
                            scope="col"
                            class="px-3 py-3 text-left text-xs uppercase tracking-wide text-gray-500"
                        >
                            From name
                        </th>
                        <th
                            scope="col"
                            class="px-3 py-3 text-left text-xs uppercase tracking-wide text-gray-500"
                        >
                            Subject
                        </th>
                        <th
                            scope="col"
                            class="px-3 py-3 text-left text-xs uppercase tracking-wide text-gray-500"
                        >
                            Body
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
                    </tr>
                </thead>
                <tbody class="divide-y divide-gray-600 bg-gray-900">
                    {#each data.files as file}
                        <tr>
                            <td
                                class="whitespace-nowrap py-4 pl-4 pr-3 text-sm font-medium text-gray-50 sm:pl-0"
                            >
                                {file.fileName}
                            </td>
                            <td
                                class="whitespace-nowrap px-3 py-4 text-sm text-gray-200"
                            >
                                {file.fileSize}
                            </td>
                            <td
                                class="whitespace-nowrap px-3 py-4 text-sm text-gray-200"
                            >
                                {file.fileType}
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
                        </tr>
                    {/each}
                </tbody>
            </table>
        </div>
    </div>

    <!-- Pagination -->
    <Pagination total={data.total} pageSize={data.pageSize} />
</div>
