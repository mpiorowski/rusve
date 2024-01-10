<script>
    import { page } from "$app/stores";
    import Drawer from "$lib/ui/Drawer.svelte";
    import Avatar from "./Avatar.svelte";
    import Breadcrumbs from "./Breadcrumbs.svelte";
    import Nav from "./Nav.svelte";

    /** @type {import("./$types").LayoutData} */
    export let data;

    let open = false;
    $: current = $page.url.pathname.split("/")[1];
</script>

{#if open}
    <Drawer {open} close={() => (open = false)} position="left">
        <Nav close={() => (open = false)} />
    </Drawer>
{/if}

<div class="h-full overflow-auto bg-gray-900 text-gray-50">
    <!-- Static sidebar for mobile -->
    <div
        class="sticky top-0 z-40 flex items-center gap-x-6 border-b border-white/5 bg-gray-900 px-4 py-4 shadow-sm sm:px-6 lg:hidden"
    >
        <button
            type="button"
            class="-m-2.5 p-2.5 text-gray-400 lg:hidden"
            on:click={() => (open = true)}
        >
            <span class="sr-only">Open sidebar</span>
            <svg
                class="h-6 w-6"
                fill="none"
                viewBox="0 0 24 24"
                stroke-width="1.5"
                stroke="currentColor"
                aria-hidden="true"
            >
                <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"
                />
            </svg>
        </button>
        <div class="flex-1 text-sm font-semibold leading-6 text-white">
            {current?.replace(/^\w/, (c) => c.toUpperCase())}
        </div>
        <Avatar email={data.email} avatarUrl={data.avatar} />
    </div>

    <!-- Static sidebar for desktop -->
    <div
        class="hidden lg:fixed lg:inset-y-0 lg:z-40 lg:flex lg:w-72 lg:flex-col"
    >
        <!-- Sidebar component, swap this element with another sidebar if you like -->
        <Nav />
    </div>

    <!-- Your content -->
    <main class="lg:pl-72">
        <header
            class="hidden items-center justify-between border-b border-white/5 px-4 py-2 sm:px-6 sm:py-4 lg:flex lg:px-8"
        >
            <Breadcrumbs />
            <Avatar email={data.email} avatarUrl={data.avatar} />
        </header>

        <div class="p-6 sm:p-8 lg:p-10">
            <slot />
        </div>
    </main>
</div>
