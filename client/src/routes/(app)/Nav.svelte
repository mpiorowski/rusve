<script>
    import { page } from "$app/stores";
    import CreditCardIcon from "$lib/icons/CreditCardIcon.svelte";
    import FileTextIcon from "$lib/icons/FileTextIcon.svelte";
    import HomeIcon from "$lib/icons/HomeIcon.svelte";
    import LogoIcon from "$lib/icons/LogoIcon.svelte";
    import SettingsIcon from "$lib/icons/SettingsIcon.svelte";
    import UserIcon from "$lib/icons/UserIcon.svelte";

    /** @type {() => void} */
    export let close = () => {};

    $: current = $page.url.pathname.split("/")[1];
</script>

<div
    class="flex grow flex-col gap-y-5 overflow-y-auto bg-black/10 px-6 pb-4 ring-1 ring-white/5"
>
    <div class="flex h-16 shrink-0 items-center">
        <a on:click={close} href="/dashboard" class="h-8 w-8 text-indigo-600">
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
                            href="/dashboard"
                            class="group flex gap-x-3 rounded-md p-2 text-sm font-semibold leading-6
                                    {current === 'dashboard'
                                ? 'bg-gray-800 text-white'
                                : 'text-gray-400 hover:bg-gray-800 hover:text-white'}"
                        >
                            <HomeIcon />
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
                            <UserIcon />
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
                            <FileTextIcon />
                            Notes
                        </a>
                    </li>
                    <li>
                        <a
                            on:click={close}
                            href="/emails"
                            class="group flex gap-x-3 rounded-md p-2 text-sm font-semibold leading-6
                                    {current === 'emails'
                                ? 'bg-gray-800 text-white'
                                : 'text-gray-400 hover:bg-gray-800 hover:text-white'}"
                        >
                            <FileTextIcon />
                            Emails
                        </a>
                    </li>
                    <li>
                        <a
                            on:click={close}
                            href="/files"
                            class="group flex gap-x-3 rounded-md p-2 text-sm font-semibold leading-6
                                    {current === 'files'
                                ? 'bg-gray-800 text-white'
                                : 'text-gray-400 hover:bg-gray-800 hover:text-white'}"
                        >
                            <FileTextIcon />
                            Files
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
                            <CreditCardIcon />
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
                    <SettingsIcon />
                    Settings
                </a>
            </li>
        </ul>
    </nav>
</div>
