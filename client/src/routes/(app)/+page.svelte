<script lang="ts">
    export let data;
</script>

<div class="p-4">
    <h1 class="text-secondary-500 text-5xl">Welcome to the Rusve!</h1>
    <h3>(do You see what i did there?)</h3>

    <p class="text-xl mt-4">
        What are we doing here? We are searching for the best way to build
        <b>fast</b>
        and
        <b>scalable</b>
        web applications. And I think we are getting there.
    </p>
    <p class="text-xl mt-4">
        For <b>frontend</b>
        we are using the amazing SvelteKit framework. The moment You start using
        it, You will never go back. "What is the best way to learn Svelte? Learn
        React first - it'll be a great motivator".
    </p>
    <p class="text-xl mt-4">
        Also, because how easy it is to build with Svelte, this whole project is
        build with minimal amount of external libraries, the only UI library is
        <b>Tailwind</b>
        . And You still have everything You need, toasts, drawers, modals, forms,
        etc.
    </p>
    <p class="text-xl mt-4">
        For <b>backend</b>
        we are mostly using Rust. Not gonna lie, not an easy one to learn, but it
        brings
        <b>speed</b>
        and
        <b>security</b>
        to the table. And You will be able to say
        <b>"I use Rust"</b>
        .
    </p>
    <p class="text-xl mt-4">
        But becouse we are using a simple microservice architecture with gRPC,
        You can use any language You want. Here we also have a simple Go
        service.
    </p>
    <p class="text-xl mt-4">
        For <b>database</b>
        we are using PostgreSQL. It is slowly starting to win (or already won) the
        battle against MySQL.
    </p>
    <p class="text-xl mt-4">
        For <b>deployment</b>
        we are using Docker with Fly.io. Amazing and cheap option for small / medium
        applications. But there is a ready to use configuration for Google Cloud
        Platform. Each of them is ready to scale to the moon.
    </p>
    <p class="text-xl mt-4">
        I hope You will enjoy playing around, <b>{data.email}</b>
        !
    </p>
</div>
