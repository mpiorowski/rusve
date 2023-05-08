<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import { Editor } from "@tiptap/core";
    import StarterKit from "@tiptap/starter-kit";

    export let label = "";
    export let content = "";
    export let onChange: (content: string) => void;
    export let errors: string[] = [];

    let editor: Editor | undefined;
    let element: HTMLDivElement;

    onMount(() => {
        editor = new Editor({
            element: element,
            extensions: [StarterKit],
            content: content,
            onTransaction: () => {
                // force re-render so `editor.isActive` works as expected
                editor = editor;
            },
            onUpdate: (el) => {
                onChange(el.editor.getHTML());
            },
        });
    });

    onDestroy(() => {
        if (editor) {
            editor.destroy();
        }
    });
</script>

<label for="tiptap">
    {label}
    <div bind:this={element} role="textbox" id="tiptap" />
</label>
{#if editor}
    <div class="mt-2">
        <button
            type="button"
            class="h-8 w-8 border border-primary-400 rounded hover:bg-primary-600 transition mr-2"
            on:click={() => editor.chain().focus().setParagraph().run()}
            class:active={editor.isActive("paragraph")}
        >
            P
        </button>
        <button
            type="button"
            class="h-8 w-8 border border-primary-400 rounded hover:bg-primary-600 transition mr-2"
            on:click={() =>
                editor.chain().focus().toggleHeading({ level: 2 }).run()}
            class:active={editor.isActive("heading", { level: 2 })}
        >
            H2
        </button>
        <button
            type="button"
            class="h-8 w-8 border border-primary-400 rounded hover:bg-primary-600 transition mr-2"
            on:click={() =>
                editor.chain().focus().toggleHeading({ level: 3 }).run()}
            class:active={editor.isActive("heading", { level: 3 })}
        >
            H3
        </button>
        <button
            type="button"
            class="h-8 w-8 border border-primary-400 rounded hover:bg-primary-600 transition mr-2"
            on:click={() => editor.chain().focus().toggleBold().run()}
            class:active={editor.isActive("bold")}
        >
            B
        </button>
        <button
            type="button"
            class="h-8 w-8 border border-primary-400 rounded hover:bg-primary-600 transition mr-2"
            on:click={() => editor.chain().focus().toggleItalic().run()}
            class:active={editor.isActive("italic")}
        >
            I
        </button>
        <button
            type="button"
            class="h-8 w-8 border border-primary-400 rounded hover:bg-primary-600 transition mr-2"
            on:click={() => editor.chain().focus().toggleStrike().run()}
            class:active={editor.isActive("strike")}
        >
            S
        </button>
        <button
            type="button"
            class="h-8 w-8 border border-primary-400 rounded hover:bg-primary-600 transition mr-2"
            on:click={() => editor?.chain().focus().toggleOrderedList().run()}
            class:active={editor.isActive("bulletList")}
        >
            OL
        </button>
        <button
            type="button"
            class="h-8 w-8 border border-primary-400 rounded hover:bg-primary-600 transition mr-2"
            on:click={() => editor?.chain().focus().toggleBulletList().run()}
            class:active={editor.isActive("orderedList")}
        >
            UL
        </button>
    </div>
{/if}
{#if errors.length > 0}
    <p class="text-red-500">{errors.join(", ")}</p>
{/if}
