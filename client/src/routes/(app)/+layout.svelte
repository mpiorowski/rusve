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

    async function onLogout(): Promise<void> {
        await goto("/auth");
    }

    function onRust(): void {
        window.location.href = "?lang=rust";
    }

    function onGo(): void {
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

<div class="absolute right-10 top-10 flex flex-col gap-2">
    {#each $toastStore as toast}
        <Toast {toast} />
    {/each}
</div>
<main>
    <nav class="h-[60px] border-b border-gray-600">
        <div
            class="mx-auto flex h-full w-full max-w-7xl items-center justify-between px-6 text-xl"
        >
            <div class="flex w-full items-center justify-between">
                <div class="flex flex-row items-center justify-center gap-2">
                    <a
                        href="/?lang={isGo ? 'go' : 'rust'}"
                        class="flex flex-row items-center gap-2 font-bold transition hover:cursor-pointer hover:text-secondary-500 md:mr-4 md:text-2xl"
                    >
                        <div class="w-12">
                            <LogoIcon />
                        </div>
                        Rusve
                    </a>
                    <div class={isGo ? "" : "rounded ring-2 ring-teal-300"}>
                        <Button on:click={onRust}>Rust</Button>
                    </div>
                    <div class={isGo ? "rounded ring-2 ring-teal-300" : ""}>
                        <Button on:click={onGo}>Go</Button>
                    </div>
                </div>

                {#if !data.email}
                    <a
                        href="/auth?lang={isGo ? 'go' : 'rust'}"
                        class="transition hover:text-secondary-500"
                    >
                        Login
                    </a>
                {/if}
                {#if data.email}
                    <a
                        href="/profile?lang={isGo ? 'go' : 'rust'}"
                        class="transition hover:text-secondary-500"
                    >
                        Profile
                    </a>
                    <a
                        href="/notes?lang={isGo ? 'go' : 'rust'}"
                        class="transition hover:text-secondary-500"
                    >
                        Notes
                    </a>
                    <div class="flex flex-row items-center gap-4">
                        <Dropdown>
                            <svelte:fragment slot="button">
                                <div
                                    class="w-6 transition hover:cursor-pointer hover:text-secondary-500"
                                >
                                    <AvatarIcon />
                                </div>
                            </svelte:fragment>
                            <svelte:fragment slot="dropdown">
                                <div
                                    class="flex min-w-[120px] flex-col rounded bg-primary-600"
                                >
                                    <p class="px-3 py-3 font-semibold">
                                        {data.email}
                                    </p>
                                    <div
                                        class="w-full border-b border-gray-500"
                                    />
                                    <a
                                        href="/profile?lang={isGo
                                            ? 'go'
                                            : 'rust'}"
                                        class="px-3 py-3 transition hover:text-secondary-500"
                                    >
                                        Profile
                                    </a>
                                    <a
                                        href="/billing?lang={isGo
                                            ? 'go'
                                            : 'rust'}"
                                        class="px-3 py-3 transition hover:text-secondary-500"
                                    >
                                        Billing
                                    </a>
                                    <div
                                        class="w-full border-b border-gray-500"
                                    />
                                    <button
                                        on:click={onLogout}
                                        class="w-full px-3 py-3 text-left transition hover:text-secondary-500"
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
        <div class="mx-auto max-w-4xl px-6 py-8">
            <slot />
        </div>
    </section>
</main>
