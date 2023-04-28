<script lang="ts">
    import AvatarIcon from "$lib/icons/AvatarIcon.svelte";
    import Dropdown from "$lib/components/Dropdown.svelte";

    export let data;

    async function onLogout() {
        await fetch("/api/auth", {
            method: "DELETE",
        });
        window.location.reload();
    }
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
                    Rusve
                </a>
                <a href="/profile" class="hover:text-secondary-500 transition">
                    Profile
                </a>
                <a href="/notes" class="hover:text-secondary-500 transition">
                    Notes
                </a>
                <a href="/posts" class="hover:text-secondary-500 transition">
                    Posts
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
                        class="flex flex-col bg-primary-600 min-w-[120px] rounded"
                    >
                        <p class="font-semibold px-3 py-2">
                            {data.email}
                        </p>
                        <div class="border-b border-gray-500 w-full" />
                        <a
                            href="/profile"
                            class="hover:text-secondary-500 transition px-3 py-2"
                        >
                            Profile
                        </a>
                        <a
                            href="/billing"
                            class="hover:text-secondary-500 transition px-3 py-2"
                        >
                            Billing
                        </a>
                        <div class="border-b border-gray-500 w-full" />
                        <button
                            on:click={onLogout}
                            class="w-full text-left hover:text-secondary-500 transition px-3 py-2"
                        >
                            Sign out
                        </button>
                    </div>
                </svelte:fragment>
            </Dropdown>
        </div>
    </nav>
    <section class="flex-1 overflow-auto">
        <div class="max-w-4xl mx-auto px-6 py-8">
            <slot />
        </div>
    </section>
</div>
