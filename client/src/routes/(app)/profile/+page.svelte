<script lang="ts">
    import { setContext } from "svelte";
    import ProfileAvatar from "./ProfileAvatar.svelte";
    import ProfileData from "./ProfileData.svelte";
    import { toast } from "$lib/toast/toast";
    import { writable } from "svelte/store";
    import type { ProfileContext, ProfileStore } from "./profile.types";

    export let data;
    export let form;
    $: if (form?.error) {
        toast({
            message: form.error,
            type: "error",
        });
    }

    const profileStore = writable<ProfileStore>();
    $: profileStore.set({
        user: data.user,
        file: data.stream.file,
    });
    setContext<ProfileContext>("profile", profileStore);
</script>

<h3 class="text-right">
    {data.duration.toFixed(4)}ms
</h3>

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
