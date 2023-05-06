import type { LayoutServerLoad } from "./$types";

export const load = (({ locals }) => {
    return {
        userId: locals.userId,
        paymentId: locals.paymentId,
        email: locals.email,
        role: locals.role,
    };
}) satisfies LayoutServerLoad;
