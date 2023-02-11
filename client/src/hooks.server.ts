import type { HandleServerError } from "@sveltejs/kit";
import { redirect, type Handle } from "@sveltejs/kit";
import { sequence } from "@sveltejs/kit/hooks";

export const handleError: HandleServerError = ({ error }) => {
    console.error(error);
    return {
        message: "Whoops!",
        code: "UNKNOWN",
    };
};

export const authorization = (async ({ event, resolve }) => {
    try {
        event.locals.userId = crypto.randomUUID();
    } catch (err) {
        console.error("User is not authorized: %s", err);
        event.locals.userId = "";
        if (!event.url.pathname.startsWith("/auth")) {
            throw redirect(303, "/auth");
        }
    }

    if (event.url.pathname.startsWith("/auth") && event.locals.userId) {
        throw redirect(303, "/");
    }

    // If the request is still here, just proceed as normally
    const result = await resolve(event, {
        transformPageChunk: ({ html }) => html,
    });
    return result;
}) satisfies Handle;

// First handle authentication, then authorization
// Each function acts as a middleware, receiving the request handle
// And returning a handle which gets passed to the next function
export const handle: Handle = sequence(
    authorization,
);
