<script lang="ts">
    import { setContext } from "svelte";
    import ProfileAvatar from "./ProfileAvatar.svelte";
    import ProfileData from "./ProfileData.svelte";
    import type { ProfileContext } from "./profile.types";
    import type { ActionData } from "./$types";
    import { toast } from "$lib/toast/toast";

    export let data;
    export let form;
    $: if (form?.error) {
        toast({
            message: form.error,
            type: "error",
        });
    }
    setContext<ProfileContext<ActionData>>("profile", {
        user: data.user,
        form: form,
    });
</script>

<div class="flex flex-col border border-primary-600 rounded">
    <div class="p-4">
        <h3>Your email</h3>
        <p>{data.email}</p>
    </div>
    <div class="border-b border-primary-600" />
    <ProfileData />
    <div class="border-b border-primary-600" />
    <ProfileAvatar />
</div>
