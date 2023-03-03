import { URI_USERS } from "$env/static/private";
import type { Handle, HandleServerError } from "@sveltejs/kit";
import { sequence } from "@sveltejs/kit/hooks";
import { fetchToken, usersClient } from "./grpc";
import type { User__Output } from "./proto/proto/User";
import type { UserId } from "./proto/proto/UserId";

export const handleError: HandleServerError = ({ error }) => {
    console.error("Error: %s", error);
    if (error instanceof Error) {
        return {
            message: error.message,
            code: "UNKNOWN",
        };
    }
    return {
        message: "Unknown error",
        code: "UNKNOWN",
    };
};

export const authorization = (async ({ event, resolve }) => {
    const request: UserId = { userId: "123e4567-e89b-12d3-a456-426655440000" };
    const metadata = await fetchToken(URI_USERS);
    const user = await new Promise<User__Output>((resolve, reject) => {
        usersClient.getUser(request, metadata, (err, response) =>
            err || !response ? reject(err) : resolve(response),
        );
    });

    event.locals.role = user.role;
    event.locals.email = user.email;
    event.locals.userId = user.id;

    // If the request is still here, just proceed as normally
    const result = await resolve(event, {
        transformPageChunk: ({ html }) => html,
    });
    return result;
}) satisfies Handle;

// First handle authentication, then authorization
// Each function acts as a middleware, receiving the request handle
// And returning a handle which gets passed to the next function
export const handle: Handle = sequence(authorization);
