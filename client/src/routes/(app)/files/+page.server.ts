import { error } from "@sveltejs/kit";
import { z } from "zod";
import type { Actions, PageServerLoad } from "./$types";

export const load = (({ locals }) => {
    return {};
}) satisfies PageServerLoad;

export const actions = {
    createFile: async ({ request }) => {
        const form = await request.formData();
        const file = form.get("file");

        const schema = z.instanceof(File).safeParse(file);

        if (!schema.success) {
            return error(400, "Invalid request");
        }

        return { file: schema.data.name };
    },
} satisfies Actions;
