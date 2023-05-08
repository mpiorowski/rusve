<script lang="ts">
    import { enhance } from "$app/forms";
    import Drawer from "$lib/components/Drawer.svelte";
    import Button from "$lib/form/Button.svelte";
    import Input from "$lib/form/Input.svelte";
    import { toast } from "$lib/toast/toast";
    import { getContext } from "svelte";
    import type { ActionData } from "./$types";
    import type { Writable } from "svelte/store";
    import TipTap from "$lib/form/TipTap.svelte";

    export let form: ActionData;
    let title = "";
    let content = "";
    let loading = false;
    const drawer = getContext<Writable<boolean>>("drawer");

    function onChange(value: string) {
        content = value;
    }

</script>

<Drawer>
    <span slot="header">
        <h2>Create post</h2>
    </span>
    <span slot="content">
        <form
            action="?/createPost"
            id="createPost"
            method="post"
            use:enhance={() => {
                loading = true;
                return async ({ result, update }) => {
                    await update({ reset: false });
                    if (result.type === "success") {
                        toast({
                            message: "Post created",
                            type: "success",
                        });
                        drawer.set(false);
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
                <input type="hidden" name="content" bind:value={content} />
                <TipTap
                    {onChange}
                    errors={form?.error?.fieldErrors.content ?? []}
                />
            </div>
        </form>
    </span>
    <span slot="footer">
        <div class="w-28">
            <Button form="createPost" {loading}>Submit</Button>
        </div>
    </span>
</Drawer>
