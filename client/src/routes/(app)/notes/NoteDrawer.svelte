<script lang="ts">
    import { enhance } from "$app/forms";
    import Drawer from "$lib/components/Drawer.svelte";
    import TipTap from "$lib/components/form/TipTap.svelte";
    import Button from "$lib/components/form/Button.svelte";
    import Input from "$lib/components/form/Input.svelte";
    import { toast } from "$lib/components/toast/toast";
    import type { ActionData } from "./$types";
    import { getContext } from "svelte";
    import type { NoteContext } from "$lib/types";
    import { page } from "$app/stores";

    export let form: ActionData;

    let loading = false;
    const drawer = getContext<NoteContext>("drawer");
    const type = $page.url.searchParams.get("lang") ?? "rust";

    function onChange(val: string): void {
        drawer.set({
            open: true,
            data: {
                ...$drawer.data,
                content: val,
            },
        });
    }
</script>

<Drawer>
    <span slot="header">
        <h2>
            {#if $drawer.data.id}
                Edit note
            {:else}
                Create note
            {/if}
        </h2>
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
                        toast.success("Note created");
                    }
                    drawer.set({
                        open: false,
                        data: { id: "", title: "", content: "" },
                    });
                    loading = false;
                };
            }}
        >
            <div class="p-6">
                <input type="hidden" name="id" value={$drawer.data.id} />
                <input type="hidden" name="type" value={type} />
                <Input
                    name="title"
                    bind:value={$drawer.data.title}
                    label="Title"
                    errors={form?.error?.fieldErrors.title ?? []}
                />
                <input
                    type="hidden"
                    name="content"
                    bind:value={$drawer.data.content}
                />
                <TipTap
                    label="Content"
                    content={$drawer.data.content}
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
