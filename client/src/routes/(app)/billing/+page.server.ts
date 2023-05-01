import { DOMAIN, URI_USERS } from "$env/static/private";
import { getStripe } from "$lib/apps/stripe";
import { usersClient } from "$lib/grpc";
import { createMetadata } from "$lib/metadata";
import type { PaymentId } from "$lib/proto/proto/PaymentId";
import { fail, redirect } from "@sveltejs/kit";
import { z } from "zod";
import type { Actions, PageServerLoad } from "./$types";

export const load = (({ locals }) => {
    return {};
}) satisfies PageServerLoad;

export const actions = {
    default: async ({ request, locals }) => {
        const form = await request.formData();
        const stripe = getStripe();

        const lookup_key = form.get("lookup_key");
        let paymentId = form.get("paymentId");
        const userId = locals.userId;
        const email = locals.email;

        const schema = z
            .object({
                lookup_key: z.string().min(1),
                email: z.string().email(),
            })
            .safeParse({ lookup_key, email });
        if (!schema.success) {
            throw new Error("Invalid input");
        }

        if (!paymentId) {
            const customer = await stripe.customers.create({
                email: schema.data.email,
            });
            const data: PaymentId = {
                userId: userId,
                paymentId: customer.id,
            };
            const metadata = await createMetadata(URI_USERS);
            await new Promise((resolve, reject) => {
                usersClient.updatePaymentId(data, metadata, (err, res) => {
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

        const price = await stripe.prices.retrieve("price_1N2sqAHRNcZGXxTiPeuR5y02");
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
            success_url: `${DOMAIN}/billing/info?success=true&session_id={CHECKOUT_SESSION_ID}`,
            cancel_url: `${DOMAIN}/billing/info?canceled=true`,
        });
        if (!session.url) {
            return fail(500, { error: "Session URL not found" });
        }
        throw redirect(303, session.url);
    },
} satisfies Actions;
