import { ENV } from "$env/static/private";

export function performanceLogger(name: string): () => void | void {
    if (ENV === "production") {
        return () => {
            // do nothing
        };
    }
    const start = performance.now();

    function end() {
        const end = performance.now();
        const duration = end - start;
        console.info(`${name} took ${duration}ms`);
    }

    return end;
}
