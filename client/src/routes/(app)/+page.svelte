<script lang="ts">
    import LogoIcon from "$lib/assets/icons/LogoIcon.svelte";
    import type { PageData } from "./$types";
    import { Categories } from "$lib/types";

    export let data: PageData;
</script>

<h1 class="text-center text-5xl text-secondary-500">
    <div class="m-auto mb-4 h-32 w-32 fill-black">
        <LogoIcon />
    </div>
    Welcome to the Rusve!
</h1>

<p class="mb-10 mt-4 text-center text-3xl">
    What are we doing here? We are searching for the best way to build
    <b>fast</b>
    and
    <b>scalable</b>
    web applications. And I think we are getting there.
    <br />
    <br />
    I hope You will enjoy playing around!
    <br />
    <br />
    Source code is available on
    <a
        class="text-secondary-500 transition hover:text-secondary-400"
        href="https://www.github.com/mpiorowski/rusve"
        target="_blank"
    >
        Github
    </a>
    .
</p>

<div class="m-auto max-w-xl">
    {#await data.stream.dashboard then dashboard}
        {#each Object.values(Categories) as category}
            {#if dashboard.some((el) => el.category === category)}
                <h1 class="mb-2 mt-6 text-3xl text-secondary-500">
                    {category}
                </h1>
                {#each dashboard.filter((el) => el.category === category) as { title, description }}
                    <div class="mb-2 w-full rounded-xl p-6 shadow-inner">
                        <h2 class="mb-2 text-secondary-500">{title}</h2>
                        <h3>{description}</h3>
                    </div>
                {/each}
            {/if}
        {/each}
    {:catch error}
        <p class="text-center text-3xl text-secondary-500">
            {error.message}
        </p>
    {/await}
</div>
