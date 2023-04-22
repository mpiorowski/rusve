<script lang="ts">
    import Button from "$lib/form/Button.svelte";
    import EmptyAvatar from "$lib/icons/EmptyAvatar.svelte";
    import { getContext } from "svelte";
    import type { ProfileContext } from "./profile.types";
    import type { ActionData } from "./$types";

    const profile = getContext<ProfileContext<ActionData>>("profile");
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
    <input type="file" name="avatar" />
    <div class="w-20">
        <Button type="submit">Upload</Button>
    </div>
</form>
