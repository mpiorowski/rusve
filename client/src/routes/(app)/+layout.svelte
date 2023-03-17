<script lang="ts">
    import { signOut } from "@auth/sveltekit/client";
</script>

<navigation
    class="flex flex-row items-center justify-center gap-4 text-lg text-emerald-400 p-2 border-b border-gray-600"
>
    <a href="/" class="hover:text-emerald-500">Home</a>
    <a href="/files" class="hover:text-emerald-500">Files</a>
    <a href="/notes1" class="hover:text-emerald-500">Notes-1</a>
    <a href="/notes2" class="hover:text-emerald-500">Notes-2</a>
    <a href="/notes3" class="hover:text-emerald-500">Notes-3</a>
    <button on:click={() => signOut()}>Logout</button>
</navigation>
<div class="p-4">
    <slot />
</div>
