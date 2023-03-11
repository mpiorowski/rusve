<script lang="ts">
    import { signOut } from "@auth/sveltekit/client";
</script>

<navigation
    class="flex flex-row items-center justify-center gap-2 text-xl mb-10 text-emerald-400"
>
    <a
        href="/"
        class="border rounded p-2 border-emerald-400 hover:border-emerald-500"
    >
        Home
    </a>

    <a
        href="/notes"
        class="border rounded p-2 border-emerald-400 hover:border-emerald-500"
    >
        Notes
    </a>
    <a
        href="/files"
        class="border rounded p-2 border-emerald-400 hover:border-emerald-500"
    >
        Files
    </a>
    <button on:click={() => signOut()}>Logout</button>
</navigation>
<slot />
