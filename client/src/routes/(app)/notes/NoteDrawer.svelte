<script lang="ts">
    import { enhance } from "$app/forms";
    import Drawer from "$lib/components/Drawer.svelte";
    import Button from "$lib/form/Button.svelte";
    import Input from "$lib/form/Input.svelte";
    import { toast } from "$lib/toast/toast";
    import type { ActionData } from "./$types";

    export let open: boolean;
    export let form: ActionData;

    let title = "";
    let content = "";
    let loading = false;
</script>

<Drawer {open} on:clickOutside>
    <span slot="header">
        <h2>Create note</h2>
    </span>
    <span slot="content">
        <form
            action="?/createNote"
            id="createNote"
            method="post"
            use:enhance={() => {
                return async ({ result, update }) => {
                    loading = true;
                    await update({ reset: false });
                    if (result.type === "success") {
                        toast({
                            message: "Note created",
                            type: "success",
                        });
                    }
                    loading = false;
                };
            }}
        >
            <div class="p-6">
                <Input
                    name="title"
                    bind:value={title}
                    label="Title"
                    errors={form?.error?.fieldErrors.title ?? []}
                />
                <Input
                    textarea
                    name="content"
                    bind:value={content}
                    label="Content"
                    errors={form?.error?.fieldErrors.content ?? []}
                />
            </div>
        </form>
    </span>
    <span slot="footer">
        <div class="w-28">
            <Button form="createNote" {loading}>Submit</Button>
        </div>
    </span>
</Drawer>
