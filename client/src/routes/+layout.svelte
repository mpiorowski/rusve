<script lang="ts">
    import "../app.css";
    import Toast from "$lib/toast/Toast.svelte";
    import { toastStore } from "$lib/toast/toast";
</script>

<svelte:head>
    <title>Rusve</title>
    <meta
        name="description"
        content="Open source Rust with SvelteKit application, using gRPC and microservices."
    />
</svelte:head>

<div class="absolute top-10 right-10 flex flex-col gap-2">
    {#each $toastStore as toast}
        <Toast {toast} />
    {/each}
</div>
<slot />
