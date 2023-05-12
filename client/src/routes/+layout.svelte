<script lang="ts">
    import "../app.css";
    import "$lib/form/TipTap.css";
    import Toast from "$lib/toast/Toast.svelte";
    import { toastStore } from "$lib/toast/toast";
    import LogoIcon from "$lib/icons/LogoIcon.svelte";
    import Dropdown from "$lib/components/Dropdown.svelte";
    import AvatarIcon from "$lib/icons/AvatarIcon.svelte";
    import type { LayoutData } from "./$types";

    export let data: LayoutData;

    async function onLogout() {
        await fetch("/api/auth", {
            method: "DELETE",
        });
        window.location.reload();
    }
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
                <a
                    href="/"
                    class="flex flex-row md:text-2xl items-center gap-2 font-bold md:mr-4 hover:text-secondary-500 hover:cursor-pointer transition"
                >
                    <div class="w-12 h-12">
                        <LogoIcon />
                    </div>
                    Rusve
                </a>

                {#if !data.userId}
                    <a href="/auth" class="hover:text-secondary-500 transition">
                        Login
                    </a>
                {/if}
                {#if data.userId}
                    <a
                        href="/notes"
                        class="hover:text-secondary-500 transition"
                    >
                        Notes
                    </a>
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
                                <div class="border-b border-gray-500 w-full" />
                                <a
                                    href="/profile"
                                    class="hover:text-secondary-500 transition px-3 py-3"
                                >
                                    Profile
                                </a>
                                <a
                                    href="/billing"
                                    class="hover:text-secondary-500 transition px-3 py-3"
                                >
                                    Billing
                                </a>
                                <div class="border-b border-gray-500 w-full" />
                                <button
                                    on:click={onLogout}
                                    class="w-full text-left hover:text-secondary-500 transition px-3 py-3"
                                >
                                    Sign out
                                </button>
                            </div>
                        </svelte:fragment>
                    </Dropdown>
                {/if}
            </div>
        </div>
    </nav>
    <section class="h-[calc(100vh-60px)] overflow-auto">
        <div class="max-w-4xl mx-auto px-6 py-8">
            <slot />
        </div>
    </section>
</main>
