import { SvelteKitAuth } from "@auth/sveltekit";
import { redirect, type Handle, type HandleServerError } from "@sveltejs/kit";
import { createAuthMetadata, usersClient } from "./grpc";
import type { User__Output } from "./proto/proto/User";
import Google from "@auth/core/providers/google";
import { AUTH_SECRET, GOOGLE_ID, GOOGLE_SECRET, REDIS_TOKEN, REDIS_URL, SENDGRID_API_KEY } from "$env/static/private";
import type { AuthRequest } from "./proto/proto/AuthRequest";
import { sequence } from "@sveltejs/kit/hooks";
import type { Provider } from "@auth/core/providers";
import { UpstashRedisAdapter } from "@next-auth/upstash-redis-adapter"
import Email from "@auth/core/providers/email";
import { Redis } from "@upstash/redis";

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
            user: { role: string; email: string };
            expires: string;
        };
        console.log("Session: %s", JSON.stringify(session));
        if (!session?.user?.email) {
            throw new Error("No user session");
        }
        const request: AuthRequest = {
            sub: session.user.email,
            email: session.user.email,
        };
        const metadata = await createAuthMetadata(session.user.email);
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

const redis = new Redis({
    url: REDIS_URL,
    token: REDIS_TOKEN
});

export const handle = sequence(
    SvelteKitAuth({
        // eslint-disable-next-line @typescript-eslint/ban-ts-comment
        // @ts-ignore
        adapter: UpstashRedisAdapter(redis),
        providers: [
            // eslint-disable-next-line @typescript-eslint/ban-ts-comment
            // @ts-ignore
            Email({
                server: {
                    host: "smtp.sendgrid.net",
                    port: 587,
                    auth: {
                        user: 'apikey',
                        pass: SENDGRID_API_KEY
                    },
                },
                from: "email@homeit.app",
            }),
            Google({
                clientId: GOOGLE_ID,
                clientSecret: GOOGLE_SECRET,
            }) as Provider,
        ],
        secret: AUTH_SECRET,
        trustHost: true,
    }),
    authorization,
) satisfies Handle;
