<script lang="ts">
    export let noteId: string;
</script>

<section class="p-4 shadow-inner rounded bg-slate-600 mb-4"> 
    <h1 class="text-center">
        <slot name="title" />
    </h1>
    <p class="text-center">
        <slot name="content" />
    </p>
    <p class="text-center">
        <slot name="user" />
    </p>
    <button
        type="submit"
        class="bg-gray-800 p-3 rounded hover:bg-gray-900 transition"
        form={noteId}
    >
        Delete Note
    </button>
</section>
