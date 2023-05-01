<script lang="ts">
    import { page } from "$app/stores";

    let status = $page.url.searchParams.get("status") ?? "";
</script>

Payment status: {status}
<a href="/billing">Go back</a>
