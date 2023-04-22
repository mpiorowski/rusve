<script lang="ts">
    import "../app.css";
    import Toast from "$lib/toast/Toast.svelte";
    import { toastStore } from "$lib/toast/toast";
</script>

<div class="absolute top-10 right-10 flex flex-col gap-2">
    {#each $toastStore as toast}
        <Toast {toast} />
    {/each}
</div>
<slot />
