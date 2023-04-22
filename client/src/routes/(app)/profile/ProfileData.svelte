<script lang="ts">
    import { enhance } from "$app/forms";
    import Button from "$lib/form/Button.svelte";
    import Input from "$lib/form/Input.svelte";
    import { getContext } from "svelte";
    import type { ActionData } from "./$types";
    import type { ProfileContext } from "./profile.types";

    const profile = getContext<ProfileContext<ActionData>>("profile");

</script>

<form
    action="?/createUser"
    method="post"
    use:enhance={() => {
        return async ({ result, update }) => {
            await update({ reset: false });
            if (result.type === "error") {
                console.error("success");
            } else {
                console.log("success");
            }
        };
    }}
    class="p-4"
>
    <h3 class="mb-4">Your name</h3>
    <Input bind:value={profile.user.name} name="name" />
    <div class="w-20">
        <Button type="submit">Save</Button>
    </div>
</form>
