import { URI_UTILS } from "$env/static/private";
import { usersClient, utilsClient } from "$lib/grpc";
import { createMetadata } from "$lib/metadata";
import type { File__Output } from "$lib/proto/proto/File";
import { FileType } from "$lib/proto/proto/FileType";
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
        try {
            const form = await request.formData();
            const name = form.get("name");
            const schema = z
                .object({
                    name: z.string().max(1000),
                })
                .safeParse({ name });

            if (!schema.success) {
                console.error(schema.error);
                return fail(409, { form: schema.error.flatten() });
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
        } catch (err) {
            console.error(err);
            return fail(500, { error: "Could not create user" });
        }
    },
    createAvatar: async ({ request, locals }) => {
        try {
            const start = performance.now();

            const form = await request.formData();
            const targetId = locals.userId;
            const type = form.get("type");
            const file = form.get("file");

            if (!(file instanceof File) || file.size === 0) {
                return fail(400, { error: "Invalid file" });
            }

            // max 10MB
            if (file.size > 10 * 1024 * 1024) {
                return fail(400, { error: "File too large. Max 10MB" });
            }

            const name = file.name;
            const buffer = Buffer.from(await file.arrayBuffer());

            const schema = z
                .object({
                    targetId: z.string().uuid(),
                    name: z.string().min(1),
                    type: z.nativeEnum(FileType),
                    data: z.instanceof(Buffer),
                })
                .safeParse({
                    targetId,
                    name: name,
                    type,
                    data: buffer,
                });

            if (!schema.success) {
                console.error(schema.error);
                throw error(400, "Invalid request");
            }

            const metadata = createMetadata(URI_UTILS);
            const newFile = await new Promise<File__Output>((resolve, reject) => {
                utilsClient.createFile(schema.data, metadata, (err, response) =>
                    err || !response ? reject(err) : resolve(response),
                );
            });

            const end = performance.now();
            return { duration: end - start };
        } catch (err) {
            console.error(err);
            return fail(500, { error: "Could not create avatar" });
        }
    },
    deleteAvatar: async ({ request }) => {
        const start = performance.now();

        const form = await request.formData();
        const fileId = form.get("fileId");
        const targetId = form.get("targetId");

        const schema = z
            .object({
                fileId: z.string().uuid(),
                targetId: z.string().uuid(),
            })
            .safeParse({
                fileId,
                targetId,
            });

        if (!schema.success) {
            console.error(schema.error);
            throw error(400, "Invalid request");
        }

        const metadata = createMetadata(URI_UTILS);
        await new Promise<File__Output>((resolve, reject) => {
            utilsClient.deleteFile(schema.data, metadata, (err, response) =>
                err || !response ? reject(err) : resolve(response),
            );
        });

        const end = performance.now();
        return { duration: end - start };
    },
} satisfies Actions;
