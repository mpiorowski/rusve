import { URI_USERS_RUST } from "$env/static/private";
import { checkSubscription, getStripe } from "$lib/server/stripe";
import { usersRustClient } from "$lib/server/grpc";
import { createMetadata } from "$lib/server/metadata";
import type { PaymentId } from "$lib/proto/proto/PaymentId";
import { fail, redirect } from "@sveltejs/kit";
import { z } from "zod";
import type { Actions, PageServerLoad } from "./$types";
import { PUBLIC_DOMAIN } from "$env/static/public";

export const load = (async ({ locals }) => {
    const isSub = await checkSubscription(locals.paymentId);
    return {
        isSubscribed: isSub,
        paymentId: locals.paymentId,
    };
}) satisfies PageServerLoad;

export const actions = {
    checkout: async ({ request, locals }) => {
        const form = await request.formData();
        const stripe = getStripe();

        let paymentId = form.get("paymentId");
        const userId = locals.userId;
        const email = locals.email;

        const schema = z.string().email().safeParse(email);
        if (!schema.success) {
            return fail(500, { error: "Invalid email" });
        }

        if (!paymentId) {
            const customer = await stripe.customers.create({
                email: schema.data,
            });
            const data: PaymentId = {
                userId: userId,
                paymentId: customer.id,
            };
            const metadata = await createMetadata(URI_USERS_RUST);
            await new Promise((resolve, reject) => {
                usersRustClient.updatePaymentId(data, metadata, (err, res) => {
                    err ? reject(err) : resolve(res);
                });
            });
            paymentId = customer.id;
        }

        const schemaPaymentId = z.string().min(1);
        const schemaPaymentIdResult = schemaPaymentId.safeParse(paymentId);
        if (!schemaPaymentIdResult.success) {
            return fail(500, { error: "Invalid payment ID" });
        }

        const price = await stripe.prices.retrieve(
            "price_1N2sqAHRNcZGXxTiPeuR5y02",
        );
        const session = await stripe.checkout.sessions.create({
            customer: schemaPaymentIdResult.data,
            billing_address_collection: "auto",
            line_items: [
                {
                    price: price.id,
                    // For metered billing, do not pass quantity
                    quantity: 1,
                },
            ],
            mode: "subscription",
            success_url: `${PUBLIC_DOMAIN}/billing/info?status=success&session_id={CHECKOUT_SESSION_ID}`,
            cancel_url: `${PUBLIC_DOMAIN}/billing/info?status=error`,
        });
        if (!session.url) {
            return fail(500, { error: "Session URL not found" });
        }
        throw redirect(303, session.url);
    },
    portal: async ({ locals }) => {
        const stripe = getStripe();
        const portalSession = await stripe.billingPortal.sessions.create({
            customer: locals.paymentId,
            return_url: `${PUBLIC_DOMAIN}/billing`,
        });

        if (!portalSession.url) {
            return fail(500, { error: "Portal URL not found" });
        }
        throw redirect(303, portalSession.url);
    },
} satisfies Actions;
