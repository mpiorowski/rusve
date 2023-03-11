<script lang="ts">
    import Gmail from "$lib/icons/gmail.svelte";
    import { signIn } from "@auth/sveltekit/client";

    const onSignInUsingGoogle = async () => {
        await signIn("google");
    };
</script>

<div
    class="h-screen w-screen flex flex-col items-center justify-center max-w-lg m-auto px-6"
>
    <button
        on:click={onSignInUsingGoogle}
        class="flex flex-row gap-4 justify-center items-center border rounded p-2 border-gray-300 bg-gray-300 text-gray-900"
    >
        <div class="h-8">
            <Gmail />
        </div>
        <div>Sign in with Google</div>
    </button>
</div>
