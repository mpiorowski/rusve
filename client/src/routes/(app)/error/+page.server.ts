import { error } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";

export const load = (() => {
    console.error("page.server.ts:load: %s", "Error Test Data - Not found");
    throw error(404, "Error Test Data - Not found");
}) satisfies PageServerLoad;
