<script lang="ts">
    import LogoIcon from "$lib/assets/icons/LogoIcon.svelte";
    import type { PageData } from "./$types";
    import { Categories } from "$lib/types";

    export let data: PageData;
</script>

<h1 class="text-secondary-500 text-5xl text-center">
    <div class="w-32 h-32 mb-4 m-auto">
        <LogoIcon />
    </div>
    Welcome to the Rusve!
</h1>

<p class="text-3xl mt-4 text-center mb-10">
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
        class="text-secondary-500 hover:text-secondary-400 transition"
        href="https://www.github.com/mpiorowski/rusve"
        target="_blank"
    >
        Github
    </a>
    .
</p>

<div class="max-w-xl m-auto">
    {#await data.stream.dashboard then dashboard}
        {#each Object.values(Categories) as category}
            {#if dashboard.some((el) => el.category === category)}
                <h1 class="text-3xl text-secondary-500 mb-2 mt-6">
                    {category}
                </h1>
                {#each dashboard.filter((el) => el.category === category) as { title, description }}
                    <div class="w-full rounded-xl shadow-inner p-6 mb-2">
                        <h2 class="mb-2 text-secondary-500">{title}</h2>
                        <h3>{description}</h3>
                    </div>
                {/each}
            {/if}
        {/each}
    {:catch error}
        <p class="text-3xl text-center text-secondary-500">
            {error.message}
        </p>
    {/await}
</div>
