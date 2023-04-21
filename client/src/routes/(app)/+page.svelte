<script lang="ts">
    import type { PageData } from "./$types";

    export let data: PageData;
    let title = "This is my title";
    let content = "This is my content";
</script>

<section class="p-4">
    <h2>Current user: {data.email}</h2>
    
</section>
