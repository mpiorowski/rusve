<script>
    import { toast } from "$lib/ui/toast";
    import Input from "$lib/form/Input.svelte";
    import Button from "$lib/form/Button.svelte";
    import { enhance } from "$app/forms";
    import { extractError } from "$lib/errors";
    import SaveIcon from "$lib/icons/SaveIcon.svelte";
    import Pagination from "$lib/ui/Pagination.svelte";
    import { preloadData, pushState, goto } from "$app/navigation";
    import { page } from "$app/stores";
    import Drawer from "$lib/ui/Drawer.svelte";
    import NotePage from "./[noteId]/+page.svelte";

    /** @type {import("./$types").PageData} */
    export let data;
    /** @type {import("./$types").ActionData} */
    export let form;
    $: if (form?.error || data?.error) {
        toast.error("Error", form?.error || data?.error || "Unknown error");
    }

    /** @type {string} */
    let title = "";
    /** @type {string} */
    let content = "";
    /** @type {boolean} */
    let loading = false;

    /** @param {MouseEvent & { currentTarget: EventTarget & HTMLAnchorElement }} e */
    async function onDetails(e) {
        // bail if opening a new tab, or we're on too small a screen
        if (e.metaKey || innerWidth < 640) return;

        // prevent navigation
        e.preventDefault();

        const { href } = e.currentTarget;

        // run `load` functions (or rather, get the result of the `load` functions
        // that are already running because of `data-sveltekit-preload-data`)
        const result = await preloadData(href);

        if (result["type"] === "loaded" && result["status"] === 200) {
            pushState(href, { noteDrawer: result["data"], open: true });
        } else {
            // something bad happened! try navigating
            goto(href);
        }
    }
</script>

{#if $page.state.open}
    <Drawer
        open={$page.state.open}
        close={() => history.back()}
        title="Note details"
    >
        <NotePage isModal data={$page.state.noteDrawer} {form} />
    </Drawer>
{/if}

<form
    class="max-w-2xl"
    action="?/insert"
    method="post"
    use:enhance={() => {
        const timeout = setTimeout(() => {
            loading = true;
        }, 100);
        return async ({ result, update }) => {
            if (result.type === "success") {
                toast.success("Success", "Note created");
            }
            clearTimeout(timeout);
            loading = false;
            await update();
        };
    }}
>
    <div class="space-y-12">
        <div>
            <h2
                class="flex items-center gap-2 text-base font-semibold leading-7 text-gray-50"
            >
                New Note
            </h2>
            <p class="mt-1 text-sm leading-6 text-gray-200">
                Create a new note.
            </p>
        </div>

        <div class="mt-10 grid grid-cols-1 gap-x-6 sm:grid-cols-6">
            <div class="sm:col-span-4">
                <Input
                    name="title"
                    label="Title"
                    bind:value={title}
                    error={extractError(form?.fields, "title")}
                />
            </div>

            <div class="col-span-full">
                <Input
                    name="content"
                    label="Content"
                    bind:value={content}
                    error={extractError(form?.fields, "content")}
                    rows={3}
                    helper="Max 1000 characters"
                />
            </div>
            <div class="col-span-full flex justify-end">
                <Button type="submit" {loading}>
                    <svelte:fragment slot="icon">
                        <SaveIcon />
                    </svelte:fragment>
                    Save
                </Button>
            </div>
        </div>

        <!--
        {#each data.notes as note}
            <div
                class="mx-auto mt-8 rounded-lg bg-gray-800 p-6 text-white shadow-md"
            >
                <h2 class="mb-2 text-xl font-semibold">{note.title}</h2>
                <p class="mb-4 text-gray-50">
                    {note.content}
                </p>
                <Button class="w-20" href="/notes/{note.id}">Edit</Button>
            </div>
        {/each}
-->
    </div>
</form>

<div class="mt-10 sm:flex sm:items-center">
    <div class="sm:flex-auto">
        <h1 class="text-base font-semibold leading-6 text-gray-50">Notes</h1>
        <p class="mt-2 text-sm leading-6 text-gray-200">
            List of notes you have created.
        </p>
    </div>
</div>
<div class="mt-8 flow-root max-w-7xl">
    <div class="w-full overflow-x-auto">
        <div class="inline-block min-w-full align-middle">
            <table class="min-w-full divide-y divide-gray-600">
                <thead>
                    <tr>
                        <th
                            scope="col"
                            class="py-3 pl-4 pr-3 text-left text-xs uppercase tracking-wide text-gray-500 sm:pl-0"
                        >
                            Title
                        </th>
                        <th
                            scope="col"
                            class="px-3 py-3 text-left text-xs uppercase tracking-wide text-gray-500"
                        >
                            Content
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
                    {#each data.notes as note}
                        <tr>
                            <td
                                class="whitespace-nowrap py-4 pl-4 pr-3 text-sm font-medium text-gray-50 sm:pl-0"
                            >
                                {note.title}
                            </td>
                            <td
                                class="whitespace-nowrap px-3 py-4 text-sm text-gray-200"
                            >
                                {note.content}
                            </td>
                            <td
                                class="whitespace-nowrap px-3 py-4 text-sm text-gray-200"
                            >
                                {note.created}
                            </td>
                            <td
                                class="whitespace-nowrap px-3 py-4 text-sm text-gray-200"
                            >
                                {note.updated}
                            </td>
                            <td
                                class="relative whitespace-nowrap py-4 pl-3 pr-4 text-right text-sm font-medium sm:pr-0"
                            >
                                <a
                                    href="/notes/{note.id}"
                                    class="mr-4 text-indigo-600 hover:text-indigo-900"
                                    on:click={(e) => onDetails(e)}
                                >
                                    Edit
                                    <span class="sr-only">
                                        , {note.title}
                                    </span>
                                </a>
                            </td>
                        </tr>
                    {/each}

                    <!-- More people... -->
                </tbody>
            </table>
        </div>
    </div>

    <!-- Pagination -->
    <Pagination total={data.total} pageSize={data.pageSize} />
</div>
