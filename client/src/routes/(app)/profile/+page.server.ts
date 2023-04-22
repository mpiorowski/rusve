import { usersClient } from "$lib/grpc";
import { createMetadata } from "$lib/metadata";
import type { User, User__Output } from "$lib/proto/proto/User";
import type { UserId } from "$lib/proto/proto/UserId";
import { error, fail } from "@sveltejs/kit";
import { z } from "zod";
import type { Actions, PageServerLoad } from "./$types";

export const load = (async ({ locals }) => {
    try {
        const start = performance.now();
        const userId = locals.userId;
        const request: UserId = { userId: userId };
        const metadata = createMetadata(userId);

        const user = await new Promise<User__Output>((resolve, reject) => {
            usersClient.getUser(request, metadata, (err, response) =>
                err || !response ? reject(err) : resolve(response),
            );
        });

        const end = performance.now();

        return {
            user: user,
            duration: end - start,
        };
    } catch (err) {
        console.error(err);
        throw error(500, "Could not load user");
    }
}) satisfies PageServerLoad;

export const actions = {
    createUser: async ({ request, locals }) => {
        const form = await request.formData();
        const name = form.get("name");
        const schema = z
            .object({
                name: z.string().max(1000),
            })
            .safeParse({ name });

        if (!schema.success) {
            console.error(schema.error);
            return fail(409, { error: schema.error.flatten() });
        }

        const data: User = {
            name: schema.data.name,
        };
        const metadata = createMetadata(locals.userId);
        const user = await new Promise<User__Output>((resolve, reject) => {
            usersClient.createUser(data, metadata, (err, response) =>
                err || !response ? reject(err) : resolve(response),
            );
        });

        return { user: user };
    },
} satisfies Actions;
