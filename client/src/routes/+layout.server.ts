import type { PageServerLoad } from "./$types";

export const load = (({ locals }) => {
    return {
        email: locals.email,
    };
}) satisfies PageServerLoad;
