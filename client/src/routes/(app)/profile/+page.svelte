<script lang="ts">
    import type { PageData } from "./$types";

    export let data: PageData;
</script>

<div class="flex gap-4 justify-between items-center w-full bg-primary-600 rounded">
    <h2>Profile email</h2>
    <p>{data.email}</p>
</div>
