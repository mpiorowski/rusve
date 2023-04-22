<script lang="ts">
    import { page } from "$app/stores";
</script>

<h1 class="text-4xl font-bold">Error</h1>
<p class="text-gray-500">{$page.error?.message ?? "Unknown error"}</p>
