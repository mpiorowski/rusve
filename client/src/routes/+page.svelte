<script lang="ts">
    import LogoIcon from "$lib/icons/LogoIcon.svelte";

    const rustData = [
        {
            title: "No unnecessary unwrap / cloning",
            description: "As clean and performant as possible.",
        },
        {
            title: "Tonic",
            description:
                "Amazing gRPC rust crate, with first class support of async/await.",
        },
        {
            title: "Sql Pools and Transactions",
            description:
                "Using the best practice for the best performance and error handling.",
        },
        {
            title: "Streaming",
            description: "Streaming the data directly from postgreSQL.",
        },
    ];
    const svelteData = [
        {
            title: "Typescript",
            description:
                "Fully written in typescript with the strict rules enforced. No any or unknown in code.",
        },
        {
            title: "Form action",
            description:
                "Forms are handled by server, which force You to seperate view and logic.",
        },
        {
            title: "Zod validation",
            description:
                "Every data if validated. Errors are returned and used by client to view them on forms.",
        },
        {
            title: "Streaming",
            description:
                "Important data is loaded and rendered first, after that all the rest are returned as promises and rendered when they resolve.",
        },
        {
            title: "TawilwindCSS",
            description: "Used for styling, no other UI library needed. ",
        },
        {
            title: "Minimal external libraries",
            description:
                "With Svelte animation and stores builing custom components is very easy. Checkout our custom components.",
        },
    ];
    const data = [
        {
            title: "gRPC",
            description:
                "Fast, efficient, and with amazing typesafety, regardless of the language.",
        },
        {
            title: "Microservices",
            description:
                "Scaling? Cost-efficiency? Resilience? Got you covered.",
        },
        {
            title: "GCP",
            description:
                "Using Google Cloud Run. Easy to deploy, easy to scale, easy to maintain.",
        },
        {
            title: "Dockerized",
            description:
                "Local development? CI/CD? Production? All the same. Dockerized.",
        },
        {
            title: "Firebase SSR Auth",
            description:
                "Battle-tested OAuth, Magic Link, Phone Number, and all done on the server.",
        },
        {
            title: "Multi-language",
            description:
                "Mutli-language for backend. Using mainly Rust, with one Go service.",
        },
    ];
</script>

<h1 class="text-secondary-500 text-5xl text-center">
    <div class="w-32 h-32 mb-4 m-auto">
        <LogoIcon />
    </div>
    Welcome to the Rusve!
    <br />
    Rust + Svelte = ❤️
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

<div class="hidden grid-cols-1 md:grid-cols-2 text-primary-200 gap-6 pb-10">
    <div class="w-full rounded-xl shadow-inner p-6 md:col-span-2">
        <h1 class="mb-2 text-secondary-500">Rust</h1>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            {#each rustData as { title, description }}
                <div class="w-full rounded-xl shadow-inner p-6">
                    <h1 class="mb-2 text-secondary-500">{title}</h1>
                    <h3>{description}</h3>
                </div>
            {/each}
        </div>
    </div>
    <div class="w-full rounded-xl shadow-inner p-6 md:col-span-2">
        <h1 class="mb-2 text-secondary-500">SvelteKit</h1>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            {#each svelteData as { title, description }}
                <div class="w-full rounded-xl shadow-inner p-6">
                    <h1 class="mb-2 text-secondary-500">{title}</h1>
                    <h3>{description}</h3>
                </div>
            {/each}
        </div>
    </div>

    {#each data as { title, description }}
        <div class="w-full rounded-xl shadow-inner p-6">
            <h1 class="mb-2 text-secondary-500">{title}</h1>
            <h3>{description}</h3>
        </div>
    {/each}
</div>
