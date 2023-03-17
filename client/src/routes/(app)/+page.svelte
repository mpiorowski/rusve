<script lang="ts">
    import type { PageData } from "./$types";

    export let data: PageData;
    let title = "This is my title";
    let content = "This is my content";
</script>

<section class="p-4">
    <h2>Current user: {data.email}</h2>
    <form class="flex flex-col gap-4 mt-4" action="?/createNote" method="post">
        <input
            class="bg-slate-600 p-3 rounded shadow-inner ring-gray-800 hover:ring-teal-700 hover:ring-1 transition"
            type="text"
            name="title"
            placeholder="Title"
            bind:value={title}
        />
        <input
            class="bg-slate-600 p-3 rounded shadow-inner ring-gray-800 hover:ring-teal-700 hover:ring-1 transition"
            type="text"
            name="content"
            placeholder="Content"
            bind:value={content}
        />
        <button
            class="bg-teal-700 p-3 rounded hover:bg-teal-600 transition"
            type="submit"
        >
            Create 100x Notes
        </button>
    </form>
</section>
