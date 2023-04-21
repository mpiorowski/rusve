<script lang="ts">
    import BearIcon from "$lib/icons/BearIcon.svelte";
    import AvatarIcon from "$lib/icons/AvatarIcon.svelte";
    import { signOut } from "@auth/sveltekit/client";
    import Dropdown from "$lib/form/Dropdown.svelte";
</script>

<div class="flex flex-col h-screen">
    <nav class="border-b border-gray-600">
        <div
            class="max-w-7xl mx-auto w-full flex items-center justify-between text-lg py-2 px-6"
        >
            <div class="flex items-center gap-6">
                <a
                    href="/"
                    class="flex flex-row items-center gap-2 font-bold mr-4 hover:text-secondary-500 hover:cursor-pointer transition"
                >
                    <div class="w-10">
                        <BearIcon />
                    </div>
                    Rusve
                </a>
                <a href="/notes" class="hover:text-secondary-500 transition">
                    Notes
                </a>
                <a href="/files" class="hover:text-secondary-500 transition">
                    Files
                </a>
            </div>
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
                        class="flex flex-col text-sm bg-primary-600 w-40 rounded"
                    >
                        <a
                            href="/profile"
                            class="hover:text-secondary-500 transition p-2"
                        >
                            Profile
                        </a>
                        <div class="border-b border-gray-500 w-full" />
                        <button
                            class="w-full text-left hover:text-secondary-500 transition p-2"
                            on:click={() => signOut()}
                        >
                            Sign out
                        </button>
                    </div>
                </svelte:fragment>
            </Dropdown>
        </div>
    </nav>
    <section class="flex-1 overflow-auto">
        <div class="max-w-3xl mx-auto p-6">
            <slot />
        </div>
    </section>
</div>
