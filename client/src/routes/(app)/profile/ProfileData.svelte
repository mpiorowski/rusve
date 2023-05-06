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
        loading = true;
        return async ({ result, update }) => {
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
    <input type="hidden" name="avatar" value={$profile.user.avatar ?? ""} />
    <label class="text-xl font-bold" for="name">Your name</label>
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
