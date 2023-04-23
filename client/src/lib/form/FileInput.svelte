<script lang="ts">
    export let name: string;
    export let label: string;
</script>

<label class="block">
    <span class="sr-only">{label}</span>
    <input
        type="file"
        {name}
        class="block w-full text-sm text-gray-300 transition
      file:h-10 file:mr-4 file:py-2 file:px-4
      file:rounded-md file:border-0
      file:font-semibold file:text-base
      file:bg-primary-800 file:text-white
      hover:file:opacity-80 hover:file:cursor-pointer
    "
    />
</label>
