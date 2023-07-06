<script lang="ts">
    import { page } from "$app/stores";
</script>

<h1>Error</h1>
<h3>{$page.error?.message ?? "Unknown error"}</h3>
