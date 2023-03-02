<script lang="ts">
    import type { PageData } from "./$types";

    export let data: PageData;
</script>

<h1>Home</h1>

<h2>Current user:</h2>
<pre>
    {JSON.stringify(data, null, 2)}
</pre>
