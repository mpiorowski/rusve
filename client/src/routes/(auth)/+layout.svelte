<div class="flex flex-col h-screen">
    <nav class="border-b border-gray-600 h-[50px]">
        <div
            class="max-w-7xl h-full mx-auto flex items-center justify-between text-lg px-6"
        >
            <div class="flex items-center gap-6">
                <a
                    href="/rusve"
                    class="flex flex-row items-center gap-2 font-bold mr-4 hover:text-secondary-500 hover:cursor-pointer transition"
                >
                    Rusve
                </a>
                <a href="/features" class="hover:text-secondary-500 transition">
                    Features
                </a>
                <a
                    href="https://github.com/mpiorowski/rusve"
                    target="_blank"
                    class="hover:text-secondary-500 transition"
                >
                    Github
                </a>
            </div>
            <div>
                <a
                    href="/auth"
                    class="bg-primary-800 text-white px-4 py-2 rounded hover:bg-primary-900 transition"
                >
                    Login
                </a>
            </div>
        </div>
    </nav>
    <section class="overflow-auto h-[calc(100vh-50px)]">
        <div class="py-12 px-6 m-auto max-w-4xl h-full">
            <slot />
        </div>
    </section>
</div>
