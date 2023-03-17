<script lang="ts">
    import Gmail from "$lib/icons/gmail.svelte";
    import { signIn } from "@auth/sveltekit/client";

    let email = "";
    let error = "";

    async function onSignInUsingGoogle() {
        await signIn("google");
    }

    async function onSignInUsingEmail() {
        error = "";
        if (RegExp(/^\S+@\S+$/).test(email) === false) {
            error = "Please enter an email";
            return;
        }
        await signIn("email", { email: email });
    }
</script>

<section
    class="flex flex-col gap-6 justify-center bg-slate-600 shadow-lg p-8 rounded-md m-8"
>
    <button
        on:click={onSignInUsingGoogle}
        class="flex flex-row gap-4 justify-center items-center rounded px-4 py-2 shadow-md font-bold bg-gray-800 text-gray-200 hover:bg-gray-700 transition"
    >
        <div class="h-8">
            <Gmail />
        </div>
        <div>Sign in with Google</div>
    </button>
    <div class="flex flex-row gap-4 items-center">
        <div class="border-b w-full border-gray-200" />
        <div class="text-gray-200">or</div>
        <div class="border-b w-full border-gray-200" />
    </div>
    <input
        type="email"
        name="email"
        bind:value={email}
        placeholder="Email"
        class={`rounded px-4 py-2 bg-slate-700 text-gray-200 shadow-inner w-full transition ${
            error ? "ring-1 ring-red-500" : ""
        }`}
    />
    <button
        on:click={onSignInUsingEmail}
        class="flex flex-row gap-4 justify-center items-center rounded px-4 py-2 shadow-md bg-gray-800 hover:bg-gray-700 transition text-gray-200 font-bold"
    >
        <div>Sign in with Email</div>
    </button>
</section>
