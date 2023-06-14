import type { LayoutServerLoad } from "./$types";

export const load = (({ locals }) => {
    return {
        userId: locals.userId.toString("hex"),
        email: locals.email,
        role: locals.role,
    };
}) satisfies LayoutServerLoad;
