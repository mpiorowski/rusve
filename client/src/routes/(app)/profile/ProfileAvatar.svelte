<script lang="ts">
    import Button from "$lib/form/Button.svelte";
    import EmptyAvatar from "$lib/icons/EmptyAvatar.svelte";
    import { getContext } from "svelte";
    import type { ProfileContext } from "./profile.types";
    import SaveIcon from "$lib/icons/SaveIcon.svelte";
    import { FileType } from "$lib/proto/proto/FileType";
    import { enhance } from "$app/forms";
    import { toast } from "$lib/toast/toast";
    import FileInput from "$lib/form/FileInput.svelte";
    import DeleteIcon from "$lib/icons/DeleteIcon.svelte";

    const profile = getContext<ProfileContext>("profile");
    let loading = false;
    let deleteLoading = false;
</script>

<div class="p-4 flex flex-col gap-4">
    <h3>Your avatar</h3>
    {#await $profile.file}
        <div class="h-16 w-16">
            <EmptyAvatar />
        </div>
    {:then file}
        {#if file}
            <form
                action="?/deleteAvatar"
                method="post"
                class="flex flex-row gap-4 items-center"
                use:enhance={() => {
                    return async ({ result, update }) => {
                        deleteLoading = true;
                        await update();
                        if (result.type === "success") {
                            toast({
                                message: "Avatar deleted",
                                type: "success",
                            });
                        }
                        deleteLoading = false;
                    };
                }}
            >
                <div class="h-16 w-16">
                    <img
                        src={`data:image;base64,${file.data}`}
                        alt="Avatar"
                        class="rounded-full object-cover h-full w-full"
                    />
                </div>
                <input type="hidden" name="fileId" value={file.id} />
                <input type="hidden" name="name" value={$profile.user.name || ""} />
                <div>
                    <Button variant="error" {loading}>
                        <span slot="icon">
                            <DeleteIcon />
                        </span>
                        Delete
                    </Button>
                </div>
            </form>
        {:else}
            <div class="h-16 w-16">
                <EmptyAvatar />
            </div>
        {/if}
    {/await}
    <form
        action="?/createAvatar"
        method="post"
        enctype="multipart/form-data"
        use:enhance={() => {
            return async ({ result, update }) => {
                loading = true;
                await update();
                if (result.type === "success") {
                    toast({
                        message: "Avatar uploaded",
                        type: "success",
                    });
                }
                loading = false;
            };
        }}
    >
        <input
            type="hidden"
            name="avatar"
            value={$profile.user.avatar ? $profile.user.avatar : ""}
        />
        <input type="hidden" name="type" value={FileType.AVATAR} />
        <input type="hidden" name="name" value={$profile.user.name || ""} />

        <div class="flex flex-row items-center gap-4">
            <div class="w-28">
                <Button {loading}>
                    <span slot="icon">
                        <SaveIcon />
                    </span>
                    Upload
                </Button>
            </div>
            <FileInput label="Choose new avatar" name="file" />
        </div>
    </form>
</div>
