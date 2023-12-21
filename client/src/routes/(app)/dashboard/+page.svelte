<script>
    const dl = [
        {
            title: "Scalable and Robust Stack",
            description:
                "Employs Rust for a powerful and memory-safe backend, complemented by SvelteKit for a modern and efficient front end.",
        },
        {
            title: "Modular Backend Architecture:",
            description:
                "Organizes the Rust backend into distinct modules, enhancing code maintainability and separation of concerns, allowing for a scalable and structured development approach.",
        },
        {
            title: "Optimized Communication",
            description:
                "Leverages gRPC for communication between components, ensuring efficient and high-performance interactions, making it suitable for scaling applications.",
        },
        {
            title: "Sharing Types via Protobuf",
            description:
                "Utilizes Protobuf to define and share data structures, ensuring consistency and facilitating efficient serialization/deserialization between the Rust backend and Svelte frontend.",
        },
        {
            title: "Built-in Database Scalability",
            description:
                "Utilizes PostgreSQL, a robust relational database, to handle large-scale data with advanced features for scalability and data integrity.",
        },
        {
            title: "Deploying to Google Cloud Run",
            description:
                "Optimized for GCP deployment, leveraging Docker for container orchestration, integrating with GCP services, and implementing automated deployment pipelines for enhanced scalability and reliability.",
        },
        {
            title: "Scalability and Dockerized Deployment",
            description:
                "Designed for scalability, with the ability to handle increased workloads, and includes a Dockerized setup for seamless deployment and easy scaling in production environments.",
        },
        {
            title: "Stripe Subscription Integration",
            description:
                "Integrates the Stripe API with the Rust backend, enabling seamless subscription management and payment processing.",
        },
    ];
</script>

<div class="flex flex-col justify-center gap-4">
    <h1>Rusve</h1>
    <h2>Rust + SvelteKit + gRPC + PostgreSQL</h2>
    <dl class="max-w-xl">
        {#each dl as { title, description }}
            <div class="relative mt-6 pl-9">
                <dt class="font-semibold text-gray-100">
                    <svg
                        class="absolute left-0 top-1 h-5 w-5 text-indigo-500"
                        viewBox="0 0 20 20"
                        fill="currentColor"
                        aria-hidden="true"
                    >
                        <path
                            fill-rule="evenodd"
                            d="M16.704 4.153a.75.75 0 01.143 1.052l-8 10.5a.75.75 0 01-1.127.075l-4.5-4.5a.75.75 0 011.06-1.06l3.894 3.893 7.48-9.817a.75.75 0 011.05-.143z"
                            clip-rule="evenodd"
                        />
                    </svg>
                    {title}
                </dt>
                <dd class="mt-2 text-sm text-gray-200">
                    {description}
                </dd>
            </div>
        {/each}
    </dl>
</div>
