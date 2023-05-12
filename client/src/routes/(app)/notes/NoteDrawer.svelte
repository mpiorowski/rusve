<script lang="ts">
    import { enhance } from "$app/forms";
    import Drawer from "$lib/components/Drawer.svelte";
    import TipTap from "$lib/form/TipTap.svelte";
    import Button from "$lib/form/Button.svelte";
    import Input from "$lib/form/Input.svelte";
    import { toast } from "$lib/toast/toast";
    import type { ActionData } from "./$types";
    import { getContext } from "svelte";
    import type { DrawerContext } from "$lib/types";

    export let form: ActionData;

    let title = "";
    let content = "";
    let loading = false;

    function onChange(val: string) {
        content = val;
    }

    const drawer = getContext<DrawerContext>("drawer");
</script>

<Drawer>
    <span slot="header">
        <h2>Create note</h2>
    </span>
    <span slot="content">
        <form
            action="?/createNote"
            id="createNote"
            method="post"
            use:enhance={() => {
                loading = true;
                return async ({ result, update }) => {
                    await update({ reset: false });
                    if (result.type === "success") {
                        toast({
                            message: "Note created",
                            type: "success",
                        });
                    }
                    drawer.set({ open: false, data: "" });
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
                <input type="hidden" name="content" bind:value={content} />
                <input type="hidden" name="type" value={$drawer.data} />
                <TipTap
                    label="Content"
                    {onChange}
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
