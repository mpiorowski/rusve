import { fail } from "@sveltejs/kit";

/** @type {import('./$types').PageServerLoad} */
export function load({ locals }) {
    return {
        locals,
    };
}

/** @type {import('./$types').Actions} */
export const actions = {
    default: async ({ locals, request }) => {
        const form = await request.formData();
        const id = form.get("id");

        if (!locals.user) {
            return fail(401, { error: "Unauthorized" });
        }

        return { id };
    },
};
