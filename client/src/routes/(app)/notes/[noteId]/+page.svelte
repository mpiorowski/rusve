<script>
    import { enhance } from "$app/forms";
    import { goto } from "$app/navigation";
    import { extractError } from "$lib/errors";
    import Button from "$lib/form/Button.svelte";
    import Input from "$lib/form/Input.svelte";
    import { toast } from "$lib/ui/toast";

    /** @type {import("./$types").PageData} */
    export let data;

    /** @type {import("./$types").ActionData} */
    export let form;
    $: if (form?.error) {
        toast.error("Error", form.error);
    }
    /** @type {boolean} */
    export let isModal = false;
</script>

<form
    class="max-w-2xl"
    action="?/update"
    method="post"
    use:enhance={() => {
        return async ({ result, update }) => {
            if (result.type === "success") {
                toast.success("Success", "Note updated");
            }
            await update({ reset: false });
            if (isModal) {
                await goto("/notes");
            }
        };
    }}
>
    <div class="space-y-12">
        <div>
            {#if !isModal}
                <h2
                    class="flex items-center gap-2 text-base font-semibold leading-7 text-gray-900"
                >
                    Note details
                </h2>
            {/if}
            <p class="mt-1 text-sm leading-6 text-gray-600">
                {data.note.id}
            </p>
        </div>

        <div class="mt-10 grid grid-cols-1 gap-x-6 gap-y-2 sm:grid-cols-6">
            <input type="hidden" name="id" bind:value={data.note.id} />
            <div class="sm:col-span-4">
                <input type="hidden" name="id" value={data.note.id} />
                <Input
                    name="title"
                    label="Title"
                    bind:value={data.note.title}
                    error={extractError(form?.fields, "title")}
                />
            </div>

            <div class="col-span-full">
                <Input
                    name="content"
                    label="Content"
                    bind:value={data.note.content}
                    rows={3}
                    error={extractError(form?.fields, "content")}
                />
            </div>
            <div class="col-span-full flex justify-end gap-2">
                <Button class="w-20">Update</Button>
                <form
                    action="?/delete"
                    method="post"
                    use:enhance={() => {
                        return async ({ result, update }) => {
                            if (result.type === "success") {
                                toast.warning("Success", "Note deleted");
                                await goto("/notes");
                            }
                            await update();
                        };
                    }}
                >
                    <input type="hidden" name="id" value={data.note.id} />
                    <Button class="w-20" variant="danger">Delete</Button>
                </form>
            </div>
        </div>
    </div>
</form>
