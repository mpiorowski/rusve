import { redirect, type Handle, type HandleServerError } from "@sveltejs/kit";
import type { AuthRequest } from "$lib/proto/proto/AuthRequest";
import { createMetadata } from "$lib/server/metadata";
import { usersGoClient, usersRustClient } from "$lib/server/grpc";
import { getFirebaseServer } from "$lib/server/firebase_server";
import { URI_USERS_GO, URI_USERS_RUST } from "$env/static/private";
import type { User__Output } from "$lib/proto/proto/User";
import { perf } from "$lib/logging";
import { grpcSafe, safe, type Safe } from "$lib/server/safe";

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

const emptySession = {
    userId: "",
    paymentId: "",
    email: "",
    role: "",
    isSubscribed: false,
};
export const handle: Handle = async ({ event, resolve }) => {
    const log = perf("Authorization");
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

    /**
     * Verify user session against Firebase
     * @param {string} session - Firebase session cookie
     * @returns {Promise<firebase.auth.DecodedIdToken>} - Firebase decoded token
     */
    const admin = getFirebaseServer();
    if (!admin.success) {
        throw redirect(303, "/auth");
    }
    const decodedClaims = await safe(
        admin.data.auth().verifySessionCookie(session, false),
    );
    if (!decodedClaims.success) {
        throw redirect(303, "/auth");
    }
    console.info("User session verified");

    /**
     * Authenticate user agains our server
     * @param {string} uid - Firebase user id
     * @param {string} email - Firebase user email
     */
    const { uid, email } = decodedClaims.data;
    const request: AuthRequest = {
        sub: uid,
        email: email ?? "",
    };
    const metadata = await createMetadata(uri);
    const user = await new Promise<Safe<User__Output>>((res) => {
        client.Auth(request, metadata, grpcSafe(res));
    });
    if (!user.success) {
        throw redirect(303, "/auth");
    }
    event.locals = {
        userId: user.data.id,
        email: user.data.email,
        role: user.data.role,
        paymentId: user.data.paymentId ?? "",
    };

    log();
    if (!event.locals.userId.length) {
        throw redirect(303, "/auth");
    }
    return await resolve(event);
};
