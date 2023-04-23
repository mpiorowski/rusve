<script lang="ts">
    import { fade } from "svelte/transition";
    let open = false;

    const transitionConfig = {
        duration: 100,
        delay: 0,
        easing: cubicInOut,
    };

    // Thank yoo ChatGPT :)
    function cubicInOut(t: number) {
        if (t < 0.5) return 4 * t * t * t;
        return (t - 1) * (2 * t - 2) * (2 * t - 2) + 1;
    }

    function useClickOutisde(node: HTMLElement) {
        const handleClick = (event: MouseEvent) => {
            if (node && !node.contains(event.target as Node)) {
                open = false;
            }
        };
        document.addEventListener("click", handleClick);
        return {
            destroy() {
                document.removeEventListener("click", handleClick);
            },
        };
    }
</script>

<div use:useClickOutisde>
    <button
        class="flex self-center"
        on:click={() => {
            open = !open;
        }}
    >
        <slot name="button" />
    </button>
    <div
        class="relative"
        on:keypress={() => {
            open = false;
        }}
        on:click={() => {
            open = false;
        }}
    >
        {#if open}
            <div
                class="absolute right-0 top-1 whitespace-nowrap shadow-lg"
                transition:fade={transitionConfig}
            >
                <slot name="dropdown" />
            </div>
        {/if}
    </div>
</div>
