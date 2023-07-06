<script lang="ts">
    import "$lib/components/form/TipTap.css";
    import Toast from "$lib/components/toast/Toast.svelte";
    import { toastStore } from "$lib/components/toast/toast";
    import LogoIcon from "$lib/assets/icons/LogoIcon.svelte";
    import Dropdown from "$lib/components/Dropdown.svelte";
    import AvatarIcon from "$lib/assets/icons/AvatarIcon.svelte";
    import type { LayoutData } from "./$types";
    import { page } from "$app/stores";
    import Button from "$lib/components/form/Button.svelte";
    import { goto } from "$app/navigation";

    export let data: LayoutData;

    async function onLogout() {
        await goto("/auth");
    }

    function onRust() {
        window.location.href = "?lang=rust";
    }

    function onGo() {
        window.location.href = "?lang=go";
    }

    $: isGo = $page.url.searchParams.get("lang") === "go";
</script>

<svelte:head>
    <title>Rusve</title>
    <meta
        name="description"
        content="Open source Rust with SvelteKit application, using gRPC and microservices."
    />
</svelte:head>

<div class="absolute top-10 right-10 flex flex-col gap-2">
    {#each $toastStore as toast}
        <Toast {toast} />
    {/each}
</div>
<main>
    <nav class="border-b border-gray-600 h-[60px]">
        <div
            class="max-w-7xl mx-auto w-full h-full flex items-center justify-between text-xl px-6"
        >
            <div class="flex w-full items-center justify-between">
                <div class="flex flex-row gap-2 items-center justify-center">
                    <a
                        href="/?lang={isGo ? 'go' : 'rust'}"
                        class="flex flex-row md:text-2xl items-center gap-2 font-bold md:mr-4 hover:text-secondary-500 hover:cursor-pointer transition"
                    >
                        <div class="w-12">
                            <LogoIcon />
                        </div>
                        Rusve
                    </a>
                    <div class={isGo ? "" : "ring-2 ring-teal-300 rounded"}>
                        <Button on:click={onRust}>Rust</Button>
                    </div>
                    <div class={isGo ? "ring-2 ring-teal-300 rounded" : ""}>
                        <Button on:click={onGo}>Go</Button>
                    </div>
                </div>

                {#if !data.email}
                    <a
                        href="/auth?lang={isGo ? 'go' : 'rust'}"
                        class="hover:text-secondary-500 transition"
                    >
                        Login
                    </a>
                {/if}
                {#if data.email}
                    <a
                        href="/profile?lang={isGo ? 'go' : 'rust'}"
                        class="hover:text-secondary-500 transition"
                    >
                        Profile
                    </a>
                    <a
                        href="/notes?lang={isGo ? 'go' : 'rust'}"
                        class="hover:text-secondary-500 transition"
                    >
                        Notes
                    </a>
                    <div class="flex flex-row items-center gap-4">
                        <Dropdown>
                            <svelte:fragment slot="button">
                                <div
                                    class="w-6 hover:cursor-pointer hover:text-secondary-500 transition"
                                >
                                    <AvatarIcon />
                                </div>
                            </svelte:fragment>
                            <svelte:fragment slot="dropdown">
                                <div
                                    class="flex flex-col bg-primary-600 min-w-[120px] rounded"
                                >
                                    <p class="font-semibold px-3 py-3">
                                        {data.email}
                                    </p>
                                    <div
                                        class="border-b border-gray-500 w-full"
                                    />
                                    <a
                                        href="/profile?lang={isGo ? 'go' : 'rust'}"
                                        class="hover:text-secondary-500 transition px-3 py-3"
                                    >
                                        Profile
                                    </a>
                                    <a
                                        href="/billing?lang={isGo ? 'go' : 'rust'}"
                                        class="hover:text-secondary-500 transition px-3 py-3"
                                    >
                                        Billing
                                    </a>
                                    <div
                                        class="border-b border-gray-500 w-full"
                                    />
                                    <button
                                        on:click={onLogout}
                                        class="w-full text-left hover:text-secondary-500 transition px-3 py-3"
                                    >
                                        Sign out
                                    </button>
                                </div>
                            </svelte:fragment>
                        </Dropdown>
                    </div>
                {/if}
            </div>
        </div>
    </nav>
    <section class="h-[calc(100dvh-60px)] overflow-auto">
        <div class="max-w-4xl mx-auto px-6 py-8">
            <slot />
        </div>
    </section>
</main>
