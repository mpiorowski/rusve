import type { PageServerLoad } from "./$types";

export const load = (({ locals }) => {
    return {
        user: {
            userId: locals.userId,
            email: locals.email,
            role: locals.role,
        },
    };
}) satisfies PageServerLoad;
