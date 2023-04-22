<script lang="ts">
    import Button from "$lib/form/Button.svelte";
    import EmptyAvatar from "$lib/icons/EmptyAvatar.svelte";
    import { getContext } from "svelte";
    import type { ProfileContext } from "./profile.types";
    import type { ActionData } from "./$types";
    import SaveIcon from "$lib/icons/SaveIcon.svelte";
    import { FileType } from "$lib/proto/proto/FileType";
    import { enhance } from "$app/forms";
    import { toast } from "$lib/toast/toast";

    const profile = getContext<ProfileContext<ActionData>>("profile");
    let loading = false;
</script>

<form action="/?createUser" class="p-4 flex flex-col gap-4">
    <h3>Your avatar</h3>
    <div class="h-8 w-8">
        {#if profile.user.avatar}
            <img src={profile.user.avatar} alt="Your avatar" />
        {:else}
            <EmptyAvatar />
        {/if}
    </div>

    <form
        action="?/createAvatar"
        method="post"
        enctype="multipart/form-data"
        class="flex flex-col gap-2 p-4"
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
        <input class="bg-gray-800 p-3 rounded" type="file" name="file" />
        <input type="hidden" name="type" value={FileType.AVATAR} />
        <div class="w-28">
            <Button {loading}>
                <span slot="icon">
                    <SaveIcon />
                </span>
                Upload
            </Button>
        </div>
    </form>
</form>
