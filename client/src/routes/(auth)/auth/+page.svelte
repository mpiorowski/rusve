<script lang="ts">
    import LoadingComponent from "$lib/components/LoadingComponent.svelte";
    import { getFirebaseClient } from "$lib/firebase/firebase_client";
    import Button from "$lib/form/Button.svelte";
    import Input from "$lib/form/Input.svelte";
    import GithubIcon from "$lib/icons/GithubIcon.svelte";
    import GmailIcon from "$lib/icons/GmailIcon.svelte";
    import MailIcon from "$lib/icons/MailIcon.svelte";
    import { toast } from "$lib/toast/toast";
    import {
        GithubAuthProvider,
        GoogleAuthProvider,
        getRedirectResult,
        isSignInWithEmailLink,
        sendSignInLinkToEmail,
        signInWithEmailLink,
        signInWithPopup,
    } from "firebase/auth";
    import { onMount } from "svelte";

    let errors: string[] = [];
    let loading = false;

    const auth = getFirebaseClient();
    const googleProvider = new GoogleAuthProvider();
    const githubProvider = new GithubAuthProvider();

    async function sendIdToken(idToken: string): Promise<void> {
        try {
            await fetch("/api/auth", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify({
                    idToken,
                }),
            });
        } catch (err) {
            console.error(err);
        }
    }

    async function onSignInWithOAuth(
        provider: GoogleAuthProvider | GithubAuthProvider,
    ): Promise<void> {
        try {
            loading = true;
            window.localStorage.setItem("checkRedirect", "true");

            /** Sign in with popup */
            const cred = await signInWithPopup(auth, provider);
            const idToken = await cred.user.getIdToken();
            await sendIdToken(idToken);
            await auth.signOut();
            window.location.reload();

            /** Sign in with redirect */
            // await signInWithRedirect(auth, provider);
        } catch (err) {
            console.error(err);
            toast({
                message: "Something went wrong",
                type: "error",
            });
        } finally {
            loading = false;
        }
    }

    let email = "";
    async function onSignInWithMagicLink(): Promise<void> {
        try {
            const url = import.meta.env.DEV
                ? "http://localhost:3000/auth"
                : "https://www.rusve.app/auth";
            await sendSignInLinkToEmail(auth, email, {
                url: url,
                handleCodeInApp: true,
            });
            window.localStorage.setItem("emailForSignIn", email);
            toast({
                message: "Check your email for a magic link",
                type: "success",
            });
        } catch (err) {
            console.error(err);
        }
    }

    async function checkRedirect(): Promise<void> {
        try {
            if (!window.localStorage.getItem("checkRedirect")) {
                return;
            }
            loading = true;
            const result = await getRedirectResult(auth);
            if (!result) {
                return;
            }
            const idToken = await result.user.getIdToken();
            await sendIdToken(idToken);
            await auth.signOut();
            window.location.reload();
        } catch (err) {
            console.error(err);
            toast({
                message: "Something went wrong",
                type: "error",
            });
            await auth.signOut();
        } finally {
            loading = false;
            window.localStorage.removeItem("checkRedirect");
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
            await sendIdToken(idToken);
            await auth.signOut();
            window.location.reload();
        } catch (err) {
            await auth.signOut();
            toast({
                message: "Something went wrong",
                type: "error",
            });
            console.error(err);
        } finally {
            loading = false;
        }
    }
    onMount(() => {
        void checkMagicLink();
        void checkRedirect();
    });
</script>

{#if loading}
    <div
        class="absolute top-0 left-0 w-screen h-screen bg-primary-500 z-10 opacity-40 flex justify-center items-center"
    >
        <LoadingComponent size={60} />
    </div>
{/if}

<div class="max-w-md m-auto flex flex-col items-center justify-center h-full">
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
