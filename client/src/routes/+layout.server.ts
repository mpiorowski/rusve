import type { LayoutServerLoad } from "./$types";

export const load = (async ({ locals }) => {
    return {
        userId: locals.userId,
    };
}) satisfies LayoutServerLoad;
