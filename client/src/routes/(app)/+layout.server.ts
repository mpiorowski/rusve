import type { LayoutServerLoad } from "./$types";

export const load = (({ locals }) => {
    return {
        email: locals.email,
    };
}) satisfies LayoutServerLoad;
