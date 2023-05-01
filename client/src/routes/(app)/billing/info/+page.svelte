<script lang="ts">
    import { page } from "$app/stores";
    import Button from "$lib/form/Button.svelte";

    let status = $page.url.searchParams.get("status") ?? "";
</script>

<h1>Payment status: {status}</h1>

<div class="w-fit mt-4">
    <Button href="/billing" variant="secondary">Back to billing</Button>
</div>
