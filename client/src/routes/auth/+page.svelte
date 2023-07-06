<script lang="ts">
    import LoadingComponent from "$lib/components/LoadingComponent.svelte";
    import { getFirebaseClient } from "$lib/firebase_client";
    import Button from "$lib/components/form/Button.svelte";
    import Input from "$lib/components/form/Input.svelte";
    import GithubIcon from "$lib/assets/icons/GithubIcon.svelte";
    import GmailIcon from "$lib/assets/icons/GmailIcon.svelte";
    import MailIcon from "$lib/assets/icons/MailIcon.svelte";
    import { toast } from "$lib/components/toast/toast";
    import {
        GithubAuthProvider,
        GoogleAuthProvider,
        isSignInWithEmailLink,
        sendSignInLinkToEmail,
        signInWithEmailLink,
        signInWithPopup,
    } from "firebase/auth";
    import { onMount } from "svelte";
    import { PUBLIC_DOMAIN } from "$env/static/public";

    let errors: string[] = [];
    let loading = false;

    let form: HTMLFormElement;
    const auth = getFirebaseClient();
    const googleProvider = new GoogleAuthProvider();
    const githubProvider = new GithubAuthProvider();

    function sendIdToken(idToken: string): void {
        try {
            const input = document.createElement("input");
            input.type = "hidden";
            input.name = "idToken";
            input.value = idToken;
            form.appendChild(input);
            form.submit();
        } catch (err) {
            toast.error("Something went wrong");
            console.error(err);
        }
    }

    async function onSignInWithOAuth(
        provider: GoogleAuthProvider | GithubAuthProvider,
    ): Promise<void> {
        try {
            loading = true;
            const cred = await signInWithPopup(auth, provider);
            const idToken = await cred.user.getIdToken();
            await auth.signOut();
            sendIdToken(idToken);
        } catch (err) {
            console.error(err);
            toast.error("Something went wrong");
            loading = false;
        }
    }

    let email = "";
    async function onSignInWithMagicLink(): Promise<void> {
        try {
            const url = PUBLIC_DOMAIN + "/auth";
            await sendSignInLinkToEmail(auth, email, {
                url: url,
                handleCodeInApp: true,
            });
            window.localStorage.setItem("emailForSignIn", email);
            toast.success("Check your email for a magic link");
        } catch (err) {
            console.error(err);
        }
    }

    async function checkMagicLink(): Promise<void> {
        if (!isSignInWithEmailLink(auth, window.location.href)) {
            return;
        }
        try {
            loading = true;
            const emailForSignIn =
                window.localStorage.getItem("emailForSignIn");
            if (!emailForSignIn) {
                throw new Error("No email found");
            }
            const user = await signInWithEmailLink(
                auth,
                emailForSignIn,
                window.location.href,
            );
            const idToken = await user.user.getIdToken();
            await auth.signOut();
            sendIdToken(idToken);
        } catch (err) {
            loading = false;
            toast.error("Something went wrong");
            console.error(err);
        }
    }
    onMount(() => {
        void checkMagicLink();
    });
</script>

{#if loading}
    <div
        class="absolute top-0 left-0 w-screen h-screen bg-primary-500 z-10 opacity-40 flex justify-center items-center"
    >
        <LoadingComponent size={60} />
    </div>
{/if}

<form method="post" bind:this={form} />
<div class="max-w-md m-auto flex flex-col items-center justify-center h-screen">
    <h2 class="text-primary-200">Welcome back</h2>
    <p class="text-primary-300 mb-4 mt-2">
        Sign in so You can say "I use Rust"
    </p>
    <div class="flex flex-col w-full gap-2">
        <Button
            variant="secondary"
            on:click={() => onSignInWithOAuth(googleProvider)}
        >
            <svelte:fragment slot="icon">
                <GmailIcon />
            </svelte:fragment>
            Google
        </Button>
        <Button
            variant="secondary"
            on:click={() => onSignInWithOAuth(githubProvider)}
        >
            <svelte:fragment slot="icon">
                <GithubIcon />
            </svelte:fragment>
            Github
        </Button>
    </div>
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
    <Button variant="secondary" on:click={onSignInWithMagicLink}>
        <svelte:fragment slot="icon">
            <MailIcon />
        </svelte:fragment>
        <div>Email</div>
    </Button>
</div>
