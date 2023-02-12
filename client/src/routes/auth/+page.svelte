<script lang="ts">
    import { signIn } from "@auth/sveltekit/client";
    import Gmail from "$lib/icons/gmail.svelte";

    let isEmail = false;
    let isSend = false;

    const onSignInUsingGoogle = async () => {
        await signIn("google");
    };
</script>

<div
    class="h-screen w-screen flex flex-col items-center justify-center max-w-lg m-auto px-6"
>
    {#if !isSend && !isEmail}
        <button
            on:click={onSignInUsingGoogle}
            class="flex gap-4 justify-center items-center h-9 mb-8 bg-slate-600 w-full p-2 rounded-lg shadow hover:bg-slate-500 focus:ring-slate-500 focus:ring-2 transition"
        >
            <Gmail />
            <span>Sign in with Google</span>
        </button>
    {/if}
</div>
