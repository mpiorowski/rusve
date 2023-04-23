<script lang="ts">
    import { browser } from "$app/environment";
    import { goto } from "$app/navigation";
    import { getFirebaseClient } from "$lib/firebase/firebase_client";
    import Button from "$lib/form/Button.svelte";
    import Input from "$lib/form/Input.svelte";
    import GmailIcon from "$lib/icons/GmailIcon.svelte";
    import {
        GoogleAuthProvider,
        isSignInWithEmailLink,
        sendSignInLinkToEmail,
        signInWithEmailLink,
        signInWithPopup,
    } from "firebase/auth";

    let errors: string[] = [];

    const googleProvider = new GoogleAuthProvider();
    const auth = getFirebaseClient();

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

    async function onSignInUsingGoogle() {
        try {
            const user = await signInWithPopup(auth, googleProvider);
            const idToken = await user.user.getIdToken();
            await sendIdToken(idToken);
            await auth.signOut();
            window.location.reload();
        } catch (err) {
            console.error(err);
            await auth.signOut();
        }
    }

    let email = "";
    async function onSignInWithMagicLink() {
        try {
            const url = import.meta.env.DEV
                ? "http://localhost:3000/auth"
                : "https://www.rusve.app/auth";
            await sendSignInLinkToEmail(auth, email, {
                url: url,
                handleCodeInApp: true,
            });
            window.localStorage.setItem("emailForSignIn", email);
        } catch (err) {
            console.error(err);
        }
    }

    async function checkMagicLink(browser: boolean) {
        if (browser && isSignInWithEmailLink(auth, window.location.href)) {
            try {
                const email = window.localStorage.getItem("emailForSignIn");
                if (!email) {
                    console.error("No email found");
                    return;
                }
                const user = await signInWithEmailLink(
                    auth,
                    email,
                    window.location.href,
                );
                const idToken = await user.user.getIdToken();
                await sendIdToken(idToken);
                goto("/");
            } catch (err) {
                console.error(err);
            } finally {
                auth.signOut();
            }
        }
    }
    $: checkMagicLink(browser);
</script>

<section
    class="max-w-md h-screen m-auto flex flex-col justify-center items-center p-4"
>
    <h2 class="text-primary-200">Welcome back</h2>
    <p class="text-primary-300 mb-4 mt-2">
        Sign in so You can say "I use Rust"
    </p>
    <Button variant="secondary" on:click={onSignInUsingGoogle}>
        <div class="h-5">
            <GmailIcon />
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
    <Button variant="secondary" on:click={onSignInWithMagicLink}>
        <div>Email</div>
    </Button>
</section>
