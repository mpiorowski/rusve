<script>
    import { page } from "$app/stores";
    import LogoIcon from "$lib/icons/LogoIcon.svelte";

    /** @type {() => void} */
    export let close = () => {};

    $: current = $page.url.pathname.split("/")[1];
</script>

<div
    class="flex grow flex-col gap-y-5 overflow-y-auto bg-black/10 px-6 pb-4 ring-1 ring-white/5"
>
    <div class="flex h-16 shrink-0 items-center">
        <a on:click={close} href="/" class="h-8 w-8 text-indigo-600">
            <LogoIcon />
        </a>
        <span class="ml-2 select-none text-xl text-gray-50">usve</span>
    </div>
    <nav class="flex flex-1 flex-col">
        <ul role="list" class="flex flex-1 flex-col gap-y-7">
            <li>
                <ul role="list" class="-mx-2 space-y-1">
                    <li>
                        <!-- Current: "bg-gray-800 text-white", Default: "text-gray-400 hover:text-white hover:bg-gray-800" -->

                        <a
                            on:click={close}
                            href="/"
                            class="group flex gap-x-3 rounded-md p-2 text-sm font-semibold leading-6
                                    {current === ''
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
                            Dashboard
                        </a>
                    </li>
                    <li>
                        <!-- Current: "bg-gray-800 text-white", Default: "text-gray-400 hover:text-white hover:bg-gray-800" -->

                        <a
                            on:click={close}
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
                            on:click={close}
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
                            on:click={close}
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
            <li class="mt-auto">
                <a
                    on:click={close}
                    href="/settings"
                    class="group -mx-2 flex gap-x-3 rounded-md p-2 text-sm font-semibold leading-6
                        {current === 'settings'
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
                    Settings
                </a>
            </li>
        </ul>
    </nav>
</div>
