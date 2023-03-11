import { SvelteKitAuth } from "@auth/sveltekit";
import { redirect, type Handle, type HandleServerError } from "@sveltejs/kit";
import { createAuthMetadata, usersClient } from "./grpc";
import type { User__Output } from "./proto/proto/User";
import Google from "@auth/core/providers/google";
import { AUTH_SECRET, GOOGLE_ID, GOOGLE_SECRET } from "$env/static/private";
import type { AuthRequest } from "./proto/proto/AuthRequest";
import { sequence } from "@sveltejs/kit/hooks";
import type { Provider } from "@auth/core/providers";

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
        const metadata = await createAuthMetadata(session.user.sub);
        const user = await new Promise<User__Output>((resolve, reject) => {
            usersClient.Auth(request, metadata, (err, response) =>
                err || !response ? reject(err) : resolve(response),
            );
        });
        event.locals.role = user.role;
        event.locals.email = user.email;
        event.locals.userId = user.id;
    } catch (err) {
        console.error("User not authorized: %s", err);
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

export const handle = sequence(
    SvelteKitAuth({
        providers: [
            Google({
                clientId: GOOGLE_ID,
                clientSecret: GOOGLE_SECRET,
            }) as Provider,
        ],
        secret: AUTH_SECRET,
        trustHost: true,
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
) satisfies Handle;
