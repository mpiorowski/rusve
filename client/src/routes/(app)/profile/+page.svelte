<script lang="ts">
    import ProfileEmail from "./ProfileEmail.svelte";
    import { setContext } from "svelte";
    import ProfileAvatar from "./ProfileAvatar.svelte";
    import ProfileData from "./ProfileData.svelte";
    import { toast } from "$lib/toast/toast";
    import { writable } from "svelte/store";
    import type { ProfileContext } from "./profileTypes";
    import type { PageData, ActionData } from "./$types";

    export let data: PageData;
    export let form: ActionData;
    $: if (form?.error) {
        toast.error(form.error);
    }

    /**
     * Component composition pattern
     * Local store, not shared with other components
     */
    const profileStore: ProfileContext = writable();
    $: profileStore.set(data.user);
    setContext<ProfileContext>("profile", profileStore);
</script>

<h3>
    {data.duration.toFixed(4)}ms
</h3>

<div class="flex flex-col border border-primary-600 rounded">
    <div class="p-4">
        <h3>Your email</h3>
        <p>{data.user.email}</p>
    </div>
    <div class="border-b border-primary-600" />
    <ProfileData />
    <div class="border-b border-primary-600" />
    <ProfileAvatar file={data.stream.file} />
    <div class="border-b border-primary-600" />
    <ProfileEmail />
</div>
