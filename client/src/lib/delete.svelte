<script lang="ts">
    import { t } from "svelte-i18n";
    import { Button, Modal } from "@mpiorowski/svelte-init";
    import { enhance } from "$app/forms";

    export let formId: string;
    export let formAction: string;
    export let itemId: string;
    export let parentId: string | null = null;
    export let deleteAction: (() => void) | null = null;

    let isOpen = false;
</script>

<Button type="error" on:click={() => (isOpen = true)}>
    <slot />
</Button>

<Modal onClose={() => (isOpen = false)} {isOpen}>
    <svelte:fragment slot="content">
        <div class="text-center m-4">
            <h2>{$t("common.confirmDelete")}</h2>
        </div>
    </svelte:fragment>
    <svelte:fragment slot="footer">
        <div class="flex gap-2 justify-center">
            <Button type="ghost" on:click={() => (isOpen = false)}>
                {$t("common.close")}
            </Button>
            <form
                action={formAction}
                method="post"
                id={formId}
                use:enhance={() =>
                    async ({ result, update }) => {
                        if (result.type === "success") {
                            isOpen = false;
                            deleteAction?.();
                        }
                        update();
                    }}
            >
                <input type="hidden" name="id" value={itemId} />
                <input type="hidden" name="parentId" value={parentId} />
                <Button type="error" form={formId}>
                    {$t("common.delete")}
                </Button>
            </form>
        </div>
    </svelte:fragment>
</Modal>
