<script lang="ts">
    import { signOut } from "@auth/sveltekit/client";
</script>

<navigation
    class="flex flex-col items-center justify-center gap-2 text-xl mb-10 text-emerald-400"
>
    <div class="flex flex-row gap-2">
        <a
            href="/"
            class="border rounded p-2 border-emerald-400 hover:border-emerald-500"
        >
            Home
        </a>
        <a
            href="/files"
            class="border rounded p-2 border-emerald-400 hover:border-emerald-500"
        >
            Files
        </a>
        <button on:click={() => signOut()}>Logout</button>
    </div>
    <div class="flex flex-row gap-2">
        <a
            href="/notes"
            class="border rounded p-2 border-emerald-400 hover:border-emerald-500"
        >
            Notes with Users
        </a>
        <a
            href="/notes2"
            class="border rounded p-2 border-emerald-400 hover:border-emerald-500"
        >
            Only Notes + Streamed Users
        </a>
        <a
            href="/notes3"
            class="border rounded p-2 border-emerald-400 hover:border-emerald-500"
        >
            Only Notes + Streamed Users loaded with one request
        </a>
    </div>
</navigation>
<slot />
