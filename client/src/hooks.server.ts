import { redirect, type Handle, type HandleServerError } from "@sveltejs/kit";
import type { AuthRequest } from "$lib/proto/proto/AuthRequest";
import { createMetadata } from "$lib/metadata";
import { usersClient } from "$lib/grpc";
import type { DecodedIdToken } from "firebase-admin/lib/auth/token-verifier";
import { getFirebaseServer } from "$lib/firebase/firebase_server";
import { URI_USERS } from "$env/static/private";

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

export const handle: Handle = async ({ event, resolve }) => {
    const now = performance.now();

    const emptySession = {
        userId: "",
        email: "",
        role: "",
    };

    const session = event.cookies.get("session") ?? "";
    if (!session || session === "") {
        console.info("No session found");
        event.locals = emptySession;
    } else {
        let decodedClaims: DecodedIdToken | undefined = undefined;
        try {
            const admin = getFirebaseServer();
            decodedClaims = await admin
                .auth()
                .verifySessionCookie(session, false);
        } catch (err) {
            console.error("Error verifying session cookie", err);
            event.locals = emptySession;
        }
        if (!decodedClaims) {
            console.error("No decoded claims found");
            event.locals = emptySession;
        } else {
            console.info("User session verified");

            // Authenticate user agains our server
            const { uid, email } = decodedClaims;
            const request: AuthRequest = {
                sub: uid,
                email: email ?? "",
            };
            const metadata = await createMetadata(URI_USERS);
            await new Promise<void>((res) => {
                usersClient.Auth(request, metadata, (err, response) => {
                    if (err || !response?.id) {
                        console.error("Error authenticating user", err);
                        event.locals = emptySession;
                    } else {
                        event.locals = {
                            userId: response.id,
                            email: response.email,
                            role: response.role,
                        };
                    }
                    res();
                });
            });
        }
    }

    const isAuth = event.url.pathname === "/auth";
    const isApiAuth = event.url.pathname === "/api/auth";
    if (!isAuth && !isApiAuth && !event.locals.userId) {
        throw redirect(303, "/auth");
    }
    if (isAuth && event.locals.userId) {
        throw redirect(303, "/");
    }
    console.debug(`Authorization: ${performance.now() - now}ms`);

    const result = await resolve(event, {
        transformPageChunk: ({ html }) => html,
    });
    return result;
};
