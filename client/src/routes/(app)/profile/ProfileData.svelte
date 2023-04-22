<script lang="ts">
    import { enhance } from "$app/forms";
    import Button from "$lib/form/Button.svelte";
    import Input from "$lib/form/Input.svelte";
    import { getContext } from "svelte";
    import { toast } from "$lib/toast/toast";
    import SaveIcon from "$lib/icons/SaveIcon.svelte";
    import type { ProfileContext } from "./profile.types";

    const profile = getContext<ProfileContext>("profile");
    let loading = false;
</script>

<form
    action="?/createUser"
    method="post"
    use:enhance={() => {
        return async ({ result, update }) => {
            loading = true;
            await update({ reset: false });
            if (result.type === "success") {
                toast({
                    message: "Profile updated",
                    type: "success",
                });
            }
            loading = false;
        };
    }}
    class="p-4"
>
    <h3 class="mb-4">Your name</h3>
    <input type="hidden" name="avatar" value={$profile.user.avatar ?? ""} />
    <Input bind:value={$profile.user.name} name="name" />
    <div class="w-28">
        <Button type="submit" {loading}>
            <span slot="icon">
                <SaveIcon />
            </span>
            Save
        </Button>
    </div>
</form>
