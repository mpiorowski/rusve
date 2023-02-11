<script lang="ts">
    import { signOut } from "@auth/sveltekit/client";
    import { Dropdown } from "@mpiorowski/svelte-init";
    import { t } from "svelte-i18n";
    import Avatar from "$lib/icons/avatar.svelte";

    const onLogout = async () => {
        await signOut();
    };
    let isOpen = false;
    export let email: string;
</script>

<Dropdown
    {isOpen}
    onClickOutside={() => {
        isOpen = false;
    }}
>
    <svelte:fragment slot="button">
        <div
            on:click={() => {
                isOpen = !isOpen;
            }}
            on:keypress={() => {
                isOpen = !isOpen;
            }}
            class="w-8 h-8 hover:cursor-pointer hover:opacity-50 transition-all"
        >
            <Avatar />
        </div>
    </svelte:fragment>
    <svelte:fragment slot="content">
        <div class="flex flex-col">
            <div
                class="text-gray-50 p-2 px-4 mt-2 border-b border-gray-400 text-sm"
            >
                {email}
            </div>
            <a
                href="/settings"
                on:click={() => {
                    isOpen = false;
                }}
                role="button"
                class="text-gray-50 p-2 hover:bg-gray-600 hover:cursor-pointer"
            >
                {$t("settings.settings")}
            </a>
            <a
                href="/settings/contact"
                on:click={() => {
                    isOpen = false;
                }}
                role="button"
                class="text-gray-50 p-2 hover:bg-gray-600 hover:cursor-pointer border-b border-gray-400"
            >
                {$t("settings.contact")}
            </a>
            <div
                role="button"
                class="text-gray-50 p-2 mb-2 border-slate-400 hover:bg-gray-600 hover:cursor-pointer"
                on:click={onLogout}
                on:keypress={onLogout}
            >
                {$t("auth.logout")}
            </div>
        </div>
    </svelte:fragment>
</Dropdown>
