<script lang="ts">
    import { enhance } from "$app/forms";
    import Button from "$lib/form/Button.svelte";
    import Input from "$lib/form/Input.svelte";
    import { getContext } from "svelte";
    import { toast } from "$lib/toast/toast";
    import SaveIcon from "$lib/icons/SaveIcon.svelte";
    import { page } from "$app/stores";
    import type { ProfileContext } from "./profileTypes";

    const lang = $page.url.searchParams.get("lang") ?? "rust";
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
                toast.success("Profile updated");
            }
            loading = false;
        };
    }}
    class="p-4"
>
    <input type="hidden" name="lang" value={lang} />
    <input type="hidden" name="avatarId" value={$profile.avatarId ?? ""} />
    <label class="text-xl font-bold" for="name">Your name</label>
    <Input bind:value={$profile.name} name="name" />
    <div class="w-28">
        <Button type="submit" {loading}>
            <span slot="icon">
                <SaveIcon />
            </span>
            Save
        </Button>
    </div>
</form>
