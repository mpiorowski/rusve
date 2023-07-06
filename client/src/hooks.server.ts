import { redirect, type Handle, type HandleServerError } from "@sveltejs/kit";
import type { AuthRequest } from "$lib/proto/proto/AuthRequest";
import { createMetadata } from "$lib/server/metadata";
import { usersGoClient, usersRustClient } from "$lib/server/grpc";
import type { DecodedIdToken } from "firebase-admin/lib/auth/token-verifier";
import { getFirebaseServer } from "$lib/server/firebase_server";
import { URI_USERS_GO, URI_USERS_RUST } from "$env/static/private";
import type { User__Output } from "$lib/proto/proto/User";
import { performanceLogger } from "$lib/logging";

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
    const log = performanceLogger("Authorization");
    const emptySession = {
        userId: Buffer.from(""),
        paymentId: "",
        email: "",
        role: "",
        isSubscribed: false,
    };
    if (event.url.pathname === "/auth") {
        event.cookies.set("session", "");
        event.locals = emptySession;
        return await resolve(event);
    }

    const isGo = event.url.searchParams.get("lang") === "go";
    const client = isGo ? usersGoClient : usersRustClient;
    const uri = isGo ? URI_USERS_GO : URI_USERS_RUST;

    const session = event.cookies.get("session") ?? "";
    if (!session || session === "") {
        console.info("No session found");
        throw redirect(303, "/auth");
    }

    let decodedClaims: DecodedIdToken | undefined = undefined;
    try {
        const admin = getFirebaseServer();
        decodedClaims = await admin.auth().verifySessionCookie(session, false);
    } catch (err) {
        console.error("Error verifying session cookie", err);
        throw redirect(303, "/auth");
    }
    console.info("User session verified");
    /**
     * Authenticate user agains our server
     * @param {string} uid - Firebase user id
     * @param {string} email - Firebase user email
     */
    try {
        const { uid, email } = decodedClaims;
        const request: AuthRequest = {
            sub: uid,
            email: email ?? "",
        };
        const metadata = await createMetadata(uri);
        const user = await new Promise<User__Output>((res, rej) => {
            client.Auth(request, metadata, (err, response) => {
                err || !response?.id ? rej(err) : res(response);
            });
        });
        event.locals = {
            userId: user.id,
            email: user.email,
            role: user.role,
            paymentId: user.paymentId ?? "",
        };
    } catch (err) {
        console.error("Error authenticating user", err);
        throw redirect(303, "/auth");
    }
    log();

    const isMain = event.url.pathname === "/";
    if (isMain) {
        const result = await resolve(event, {
            transformPageChunk: ({ html }) => html,
        });
        return result;
    }

    if (!event.locals.userId.length) {
        throw redirect(303, "/auth");
    }

    return await resolve(event);
};
