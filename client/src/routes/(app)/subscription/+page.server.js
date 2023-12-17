import { grpcSafe } from "$lib/safe";
import { usersService } from "$lib/server/grpc";
import { createMetadata } from "$lib/server/metadata";
import { fail, redirect } from "@sveltejs/kit";

/** @type {import('./$types').PageServerLoad} */
export function load({ locals }) {
    return {
        subscriptionActive: locals.user.subscriptionActive,
    };
}

/** @type {import('./$types').Actions} */
export const actions = {
    createStripeCheckout: async ({ locals }) => {
        const metadata = createMetadata("", locals.user.id);

        /** @type {import("$lib/safe").Safe<import("$lib/proto/proto/StripeUrlResponse").StripeUrlResponse__Output>} */
        const s = await new Promise((r) =>
            usersService.CreateStripeCheckout({}, metadata, grpcSafe(r)),
        );

        if (s.error) {
            return fail(500, { error: s.msg });
        }

        throw redirect(303, s.data.url ?? "");
    },
    createStripePortal: async ({ locals }) => {
        const metadata = createMetadata("", locals.user.id);

        /** @type {import("$lib/safe").Safe<
         * import("$lib/proto/proto/StripeUrlResponse").StripeUrlResponse__Output
         * >} */
        const s = await new Promise((r) =>
            usersService.CreateStripePortal({}, metadata, grpcSafe(r)),
        );

        if (s.error) {
            return fail(500, { error: s.msg });
        }

        throw redirect(303, s.data.url ?? "");
    },
};
