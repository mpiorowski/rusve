import type { Actions, PageServerLoad } from "./$types";

export const load = (({ locals }) => {
    return {};
}) satisfies PageServerLoad;

export const actions = {
    formAction: async ({ request }) => {
        const form = await request.formData();
        const id = form.get("id");

        return { id };
    },
} satisfies Actions;
