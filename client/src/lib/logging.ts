import { ENV } from "$env/static/private";
import { pino } from "pino";

export const logger = pino({
    transport: {
        target: "pino-pretty",
        options: {
            colorize: true,
        },
    },
});

export function perf(name: string): () => void | void {
    if (ENV === "production") {
        return () => {
            // do nothing
        };
    }
    const start = performance.now();

    function end(): void {
        const duration = performance.now() - start;
        logger.info(`${name}: ${duration.toFixed(4)}ms`);
    }

    return end;
}

export function debug(msg: string): void {
    if (ENV !== "production") {
        logger.debug(msg);
    }
}
