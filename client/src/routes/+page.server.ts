import { error } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";
import type { Categories } from "$lib/types";
import { URI_DIRECTUS } from "$env/static/private";
import { z } from "zod";

type DashboardItem = {
    title: string;
    description: string;
    category: Categories;
    sort: number;
};

async function fetchDashboard(): Promise<DashboardItem[]> {
    try {
        const data = await fetch(URI_DIRECTUS + "/items/dashboard");

        const json = (await data.json()) as { data: DashboardItem[] };
        z.array(
            z.object({
                title: z.string(),
                description: z.string(),
                category: z.string(),
                sort: z.number(),
            }),
        ).parse(json.data);

        return json.data.sort((a, b) => a.sort - b.sort);
    } catch (err) {
        console.error(err);
        return [];
    }
}

export const load = (() => {
    try {
        const dashboard = fetchDashboard();

        return {
            status: 200,
            stream: {
                dashboard: dashboard,
            },
        };
    } catch (err) {
        console.log(err);
        throw error(500, "Failed to fetch dashboard");
    }
}) satisfies PageServerLoad;
