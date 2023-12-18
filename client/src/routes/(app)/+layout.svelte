<script>
    import { page } from "$app/stores";
    import Button from "$lib/form/Button.svelte";
    import LogoIcon from "$lib/icons/LogoIcon.svelte";
    import Drawer from "$lib/ui/Drawer.svelte";
    import Breadcrumbs from "./Breadcrumbs.svelte";

    let open = false;
    $: current = $page.url.pathname.split("/")[1];
</script>

{#if open}
    <Drawer {open} close={() => (open = false)} title="Sidebar" position="left">
        <Button>Close</Button>
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
            Dashboard
        </div>
        <a href="#">
            <span class="sr-only">Your profile</span>
            <img
                class="h-8 w-8 rounded-full bg-gray-800"
                src="https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80"
                alt=""
            />
        </a>
    </div>

    <!-- Static sidebar for desktop -->
    <div
        class="hidden lg:fixed lg:inset-y-0 lg:z-50 lg:flex lg:w-72 lg:flex-col"
    >
        <!-- Sidebar component, swap this element with another sidebar if you like -->
        <div
            class="flex grow flex-col gap-y-5 overflow-y-auto bg-black/10 px-6 ring-1 ring-white/5"
        >
            <div class="flex h-16 shrink-0 items-center">
                <a href="/" class="h-8 w-8 text-indigo-600">
                    <LogoIcon />
                </a>
                <span class="ml-2 select-none text-xl">usve</span>
            </div>
            <nav class="flex flex-1 flex-col">
                <ul role="list" class="flex flex-1 flex-col gap-y-7">
                    <li>
                        <ul role="list" class="-mx-2 space-y-1">
                            <li>
                                <!-- Current: "bg-gray-800 text-white", Default: "text-gray-400 hover:text-white hover:bg-gray-800" -->
                                <a
                                    href="/profile"
                                    class="group flex gap-x-3 rounded-md p-2 text-sm font-semibold leading-6
                                    {current === 'profile'
                                        ? 'bg-gray-800 text-white'
                                        : 'text-gray-400 hover:bg-gray-800 hover:text-white'}"
                                >
                                    <svg
                                        class="h-6 w-6 shrink-0"
                                        fill="none"
                                        viewBox="0 0 24 24"
                                        stroke-width="1.5"
                                        stroke="currentColor"
                                        aria-hidden="true"
                                    >
                                        <path
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            d="M2.25 12l8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125 1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504 1.125-1.125V9.75M8.25 21h8.25"
                                        />
                                    </svg>
                                    Profile
                                </a>
                            </li>
                            <li>
                                <a
                                    href="/notes"
                                    class="group flex gap-x-3 rounded-md p-2 text-sm font-semibold leading-6
                                    {current === 'notes'
                                        ? 'bg-gray-800 text-white'
                                        : 'text-gray-400 hover:bg-gray-800 hover:text-white'}"
                                >
                                    <svg
                                        class="h-6 w-6 shrink-0"
                                        fill="none"
                                        viewBox="0 0 24 24"
                                        stroke-width="1.5"
                                        stroke="currentColor"
                                        aria-hidden="true"
                                    >
                                        <path
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            d="M15 19.128a9.38 9.38 0 002.625.372 9.337 9.337 0 004.121-.952 4.125 4.125 0 00-7.533-2.493M15 19.128v-.003c0-1.113-.285-2.16-.786-3.07M15 19.128v.106A12.318 12.318 0 018.624 21c-2.331 0-4.512-.645-6.374-1.766l-.001-.109a6.375 6.375 0 0111.964-3.07M12 6.375a3.375 3.375 0 11-6.75 0 3.375 3.375 0 016.75 0zm8.25 2.25a2.625 2.625 0 11-5.25 0 2.625 2.625 0 015.25 0z"
                                        />
                                    </svg>
                                    Notes
                                </a>
                            </li>
                            <li>
                                <a
                                    href="/subscription"
                                    class="group flex gap-x-3 rounded-md p-2 text-sm font-semibold leading-6
                                    {current === 'subscription'
                                        ? 'bg-gray-800 text-white'
                                        : 'text-gray-400 hover:bg-gray-800 hover:text-white'}"
                                >
                                    <svg
                                        class="h-6 w-6 shrink-0"
                                        fill="none"
                                        viewBox="0 0 24 24"
                                        stroke-width="1.5"
                                        stroke="currentColor"
                                        aria-hidden="true"
                                    >
                                        <path
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            d="M2.25 12.75V12A2.25 2.25 0 014.5 9.75h15A2.25 2.25 0 0121.75 12v.75m-8.69-6.44l-2.12-2.12a1.5 1.5 0 00-1.061-.44H4.5A2.25 2.25 0 002.25 6v12a2.25 2.25 0 002.25 2.25h15A2.25 2.25 0 0021.75 18V9a2.25 2.25 0 00-2.25-2.25h-5.379a1.5 1.5 0 01-1.06-.44z"
                                        />
                                    </svg>
                                    Subscription
                                </a>
                            </li>
                        </ul>
                    </li>
                    <li class="-mx-6 mt-auto">
                        <a
                            href="#"
                            class="flex items-center gap-x-4 px-6 py-3 text-sm font-semibold leading-6 text-white hover:bg-gray-800"
                        >
                            <img
                                class="h-8 w-8 rounded-full bg-gray-800"
                                src="https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80"
                                alt=""
                            />
                            <span class="sr-only">Your profile</span>
                            <span aria-hidden="true">Tom Cook</span>
                        </a>
                    </li>
                </ul>
            </nav>
        </div>
    </div>

    <!-- Your content -->
    <main class="lg:pl-72">
        <header
            class="hidden items-center justify-between border-b border-white/5 px-4 py-2 sm:px-6 sm:py-4 lg:flex lg:px-8"
        >
            <Breadcrumbs />
            <h1 class="text-base font-semibold leading-7 text-white">
                Deployments
            </h1>
        </header>

        <div class="px-4 sm:px-6 lg:px-8">
            <slot />
        </div>
    </main>
</div>
