<script lang="ts">
    import Button from "$lib/form/Button.svelte";
    import Input from "$lib/form/Input.svelte";
    import Gmail from "$lib/icons/gmail.svelte";
    import { signIn } from "@auth/sveltekit/client";

    let email = "";
    let errors: string[] = [];

    async function onSignInUsingGoogle() {
        await signIn("google");
    }

    async function onSignInUsingEmail() {
        errors = [];
        if (RegExp(/^\S+@\S+$/).test(email) === false) {
            errors = ["Please enter a valid email address"];
            return;
        }
        await signIn("email", { email: email });
    }
</script>

<section
    class="max-w-md h-screen m-auto flex flex-col justify-center items-center p-4"
>
    <h2 class="text-primary-200">Welcome back</h2>
    <p class="text-primary-300 mb-4 mt-2">
        Log in and explore! Also now You can say "I use Rust".
    </p>
    <Button on:click={onSignInUsingGoogle}>
        <div class="h-6">
            <Gmail />
        </div>
        <div>Google</div>
    </Button>
    <div class="w-full flex flex-row gap-4 items-center my-6">
        <div class="border-b w-full border-primary-300" />
        <div class="text-primary-300 text-sm whitespace-nowrap">
            or sign in with email
        </div>
        <div class="border-b w-full border-primary-300" />
    </div>
    <Input
        type="email"
        name="email"
        placeholder="Email"
        {errors}
        bind:value={email}
    />
    <Button on:click={onSignInUsingEmail}>
        <div>Email</div>
    </Button>
</section>
