import { redirect, type Handle, type HandleServerError } from "@sveltejs/kit";
import type { AuthRequest } from "$lib/proto/proto/AuthRequest";
import { createMetadata } from "$lib/metadata";
import { usersClient } from "$lib/grpc";
import type { DecodedIdToken } from "firebase-admin/lib/auth/token-verifier";
import { getFirebaseServer } from "$lib/firebase/firebase_server";
import { URI_USERS } from "$env/static/private";
import type { User__Output } from "$lib/proto/proto/User";
import { getStripe } from "$lib/apps/stripe";
import type Stripe from "stripe";

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
        paymentId: "",
        email: "",
        role: "",
        isSubscribed: false,
    };

    /**
     * Open pages for everyone
     */
    const isMain = event.url.pathname === "/rusve";
    const isFeatures = event.url.pathname === "/features";
    if (isMain || isFeatures) {
        event.locals = emptySession;
        return await resolve(event);
    }

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
                const metadata = await createMetadata(URI_USERS);
                const user = await new Promise<User__Output>((res, rej) => {
                    usersClient.Auth(request, metadata, (err, response) => {
                        err || !response?.id ? rej(err) : res(response);
                    });
                });
                event.locals = {
                    userId: user.id,
                    email: user.email,
                    role: user.role,
                    paymentId: user.paymentId ?? "",
                    isSubscribed: false,
                };
            } catch (err) {
                console.error("Error authenticating user", err);
                event.locals = emptySession;
            }
        }
    }

    /**
     * Check subscription only for certain pages
     * This will reduce the number of requests to Stripe
     */
    const needsSubscription = ["/billing", "/posts"];
    const isSubscriptionPage = needsSubscription.some((path) =>
        event.url.pathname.startsWith(path),
    );
    if (isSubscriptionPage && event.locals.paymentId) {
        const stripe = getStripe();
        const stripeCustomer = (await stripe.customers.retrieve(
            event.locals.paymentId,
            {
                expand: ["subscriptions"],
            },
        )) as {
            subscriptions: {
                data: Stripe.Subscription[];
            };
        };
        const isSubscribed = stripeCustomer.subscriptions.data.some(
            (sub) => sub.status === "active",
        );
        event.locals.isSubscribed = isSubscribed;
    }

    const isApiAuth = event.url.pathname === "/api/auth";
    const isAuth = event.url.pathname === "/auth";
    if (
        !isMain &&
        !isFeatures &&
        !isAuth &&
        !isApiAuth &&
        !event.locals.userId
    ) {
        throw redirect(303, "/rusve");
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
