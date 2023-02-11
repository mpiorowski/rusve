import type { HandleServerError } from "@sveltejs/kit";
import { SvelteKitAuth } from "@auth/sveltekit";
import { GOOGLE_ID, GOOGLE_SECRET } from "$env/static/private";
import { redirect, type Handle } from "@sveltejs/kit";
import { sequence } from "@sveltejs/kit/hooks";
import Google from "@auth/core/providers/google";
import type { Provider } from "@auth/core/providers";
import { metadata, usersClient } from "./grpc";
import type { AuthRequest } from "./proto/proto/AuthRequest";

export const handleError: HandleServerError = ({ error }) => {
    console.error('Error: %s', error);
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
    try {
        const session = (await event.locals.getSession()) as {
            user: { sub: string; role: string; email: string };
            expires: string;
        };
        if (!session?.user?.email || !session?.user?.sub) {
            throw new Error("No user session");
        }
        const request: AuthRequest = {
            sub: session.user.sub,
            email: session.user.email,
        };

        const promise = new Promise<void>((resolve, reject) => {
            usersClient.Auth(request, metadata, (err, response) => {
                if (err || !response?.id || !response?.role) {
                    return reject(err);
                }
                event.locals.userId = response.id;
                event.locals.role = response.role;
                event.locals.email = response.email;
                resolve();
            });
        });
        await promise;
    } catch (err) {
        console.error("User is not authorized: %s", err);
        event.locals.userId = "";
        event.locals.role = "";
        event.locals.email = "";
    }

    if (!event.locals.userId && !event.url.pathname.startsWith("/auth")) {
        throw redirect(303, "/auth");
    } else if (event.url.pathname.startsWith("/auth") && event.locals.userId) {
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
    SvelteKitAuth({
        providers: [
            Google({
                clientId: GOOGLE_ID,
                clientSecret: GOOGLE_SECRET,
            }) as Provider,
        ],
        callbacks: {
            async session({ session, token }) {
                return {
                    user: {
                        ...session.user,
                        sub: token.sub,
                    },
                    expires: session.expires,
                };
            },
        },
    }),
    authorization,
);
