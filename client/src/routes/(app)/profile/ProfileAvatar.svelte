<script lang="ts">
    import Button from "$lib/form/Button.svelte";
    import EmptyAvatarIcon from "$lib/icons/EmptyAvatarIcon.svelte";
    import { getContext } from "svelte";
    import type { ProfileContext } from "./profile.types";
    import SaveIcon from "$lib/icons/SaveIcon.svelte";
    import { FileType } from "$lib/proto/proto/FileType";
    import { enhance } from "$app/forms";
    import { toast } from "$lib/toast/toast";
    import FileInput from "$lib/form/FileInput.svelte";
    import DeleteIcon from "$lib/icons/DeleteIcon.svelte";
    import DownloadIcon from "$lib/icons/DownloadIcon.svelte";

    const profile = getContext<ProfileContext>("profile");
    let loading = false;
    let deleteLoading = false;

    async function downloadAvatar(base64: string, name: string) {
        try {
            const response = await fetch(
                `data:application/octet-stream;base64,${base64}`,
            );
            const blob = await response.blob();
            const url = URL.createObjectURL(blob);
            const link = document.createElement("a");
            link.href = url;
            link.download = name;
            document.body.appendChild(link);
            link.click();
            document.body.removeChild(link);
        } catch (e) {
            console.error(e);
            toast({
                message: "Failed to download avatar",
                type: "error",
            });
        }
    }
</script>

<div class="p-4 flex flex-col gap-4">
    <h3>Your avatar</h3>
    {#await $profile.file}
        <div class="h-16 w-16">
            <EmptyAvatarIcon />
        </div>
    {:then file}
        {#if file}
            <div class="flex flex-row items-center gap-4">
                <div class="h-16 w-16">
                    <img
                        src={`data:image;base64,${file.base64}`}
                        alt="Avatar"
                        class="rounded-full object-cover h-full w-full"
                    />
                </div>
                <div class="flex flex-row gap-2">
                    <Button
                        type="button"
                        on:click={() => downloadAvatar(file.base64, file.name)}
                        variant="secondary"
                    >
                        <span slot="icon">
                            <DownloadIcon />
                        </span>
                        Download
                    </Button>
                    <form
                        action="?/deleteAvatar"
                        method="post"
                        use:enhance={() => {
                            deleteLoading = true;
                            return async ({ result, update }) => {
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
                        <input type="hidden" name="fileId" value={file.id} />
                        <input
                            type="hidden"
                            name="name"
                            value={$profile.user.name || ""}
                        />
                        <Button variant="error" {loading}>
                            <span slot="icon">
                                <DeleteIcon />
                            </span>
                            Delete
                        </Button>
                    </form>
                </div>
            </div>
        {:else}
            <div class="h-16 w-16">
                <EmptyAvatarIcon />
            </div>
        {/if}
    {/await}
    <form
        action="?/createAvatar"
        method="post"
        enctype="multipart/form-data"
        use:enhance={() => {
            loading = true;
            return async ({ result, update }) => {
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

        <div class="flex flex-row items-center gap-6">
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
        <p class="text-xs mt-2">
            Maximum file size is 1MB. Supported formats are JPEG, PNG, GIF, and
            WebP.
        </p>
    </form>
</div>
