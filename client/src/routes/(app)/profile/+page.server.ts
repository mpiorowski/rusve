import { URI_USERS, URI_UTILS } from "$env/static/private";
import { usersClient, utilsClient } from "$lib/grpc";
import { createMetadata } from "$lib/metadata";
import type { File, File__Output } from "$lib/proto/proto/File";
import type { FileId } from "$lib/proto/proto/FileId";
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
        let metadata = await createMetadata(URI_USERS);

        const user = await new Promise<User__Output>((resolve, reject) => {
            usersClient.getUser(request, metadata, (err, response) =>
                err || !response ? reject(err) : resolve(response),
            );
        });

        let file: Promise<
            { id: string; name: string; data: string } | undefined
        > = Promise.resolve(undefined);
        if (user.avatar) {
            const fileId: FileId = {
                fileId: user.avatar,
                targetId: userId,
            };
            metadata = await createMetadata(URI_UTILS);
            file = new Promise((resolve, reject) => {
                utilsClient.getFile(fileId, metadata, (err, response) => {
                    if (err) {
                        reject(err);
                    } else if (response) {
                        const file = {
                            id: response.id,
                            name: response.name,
                            data: Buffer.from(response.buffer).toString(
                                "base64",
                            ),
                        };
                        resolve(file);
                    } else {
                        resolve(undefined);
                    }
                });
            });
        }

        const end = performance.now();
        return {
            user: user,
            duration: end - start,
            stream: {
                file: file,
            },
        };
    } catch (err) {
        console.error(err);
        // return fail(500, { error: "Could not load user" });
        throw error(500, "Could not load user");
    }
}) satisfies PageServerLoad;

export const actions = {
    createUser: async ({ request }) => {
        try {
            const form = await request.formData();
            const name = form.get("name");
            const avatar = form.get("avatar");
            const schema = z
                .object({
                    name: z.string().max(1000),
                    avatar: z.string().optional(),
                })
                .safeParse({ name, avatar });

            if (!schema.success) {
                console.error(schema.error);
                return fail(409, { form: schema.error.flatten() });
            }

            const data: User = {
                name: schema.data.name,
                avatar: schema.data.avatar || undefined,
            };
            const metadata = await createMetadata(URI_USERS);
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
            const avatar = form.get("avatar");
            const name = form.get("name");

            if (!(file instanceof File) || file.size === 0) {
                return fail(400, { error: "Invalid file" });
            }

            // max 10MB
            if (file.size > 10 * 1024 * 1024) {
                return fail(400, { error: "File too large. Max 10MB" });
            }

            const fileName = file.name;
            const buffer = Buffer.from(await file.arrayBuffer());

            // Validate
            const schema = z
                .object({
                    targetId: z.string().uuid(),
                    fileName: z.string().min(1),
                    type: z.nativeEnum(FileType),
                    buffer: z.instanceof(Buffer),
                    avatar: z.string().optional(),
                    name: z.string().optional(),
                })
                .safeParse({
                    targetId,
                    fileName,
                    type,
                    buffer,
                    avatar,
                    name,
                });
            if (!schema.success) {
                console.error(schema.error);
                return fail(400, { error: "Invalid request" });
            }

            const metadata = await createMetadata(URI_UTILS);
            // Delete old avatar
            if (schema.data.avatar) {
                const oldFileId: FileId = {
                    fileId: schema.data.avatar,
                    targetId: schema.data.targetId,
                };
                await new Promise((resolve, reject) => {
                    utilsClient.deleteFile(
                        oldFileId,
                        metadata,
                        (err, response) =>
                            err || !response ? reject(err) : resolve(response),
                    );
                });
            }

            // Create file
            const newFileData: File = {
                targetId: schema.data.targetId,
                name: schema.data.fileName,
                type: schema.data.type,
                buffer: schema.data.buffer,
            };
            const newFile = await new Promise<File__Output>(
                (resolve, reject) => {
                    utilsClient.createFile(
                        newFileData,
                        metadata,
                        (err, response) =>
                            err || !response ? reject(err) : resolve(response),
                    );
                },
            );
            if (!newFile.id) {
                return fail(500, { error: "Could not create file" });
            }

            // Create avatar
            const data: User = {
                name: schema.data.name,
                avatar: newFile.id,
            };
            const user = await new Promise<User__Output>((resolve, reject) => {
                usersClient.createUser(data, metadata, (err, response) =>
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
            return fail(500, { error: "Could not create avatar" });
        }
    },
    deleteAvatar: async ({ request, locals }) => {
        const start = performance.now();

        const form = await request.formData();
        const fileId = form.get("fileId");
        const targetId = locals.userId;
        const name = form.get("name");

        const schema = z
            .object({
                fileId: z.string().uuid(),
                targetId: z.string().uuid(),
                name: z.string().optional(),
            })
            .safeParse({
                fileId,
                targetId,
                name,
            });

        if (!schema.success) {
            console.error(schema.error);
            return fail(409, { error: "Invalid request" });
        }

        const metadata = await createMetadata(URI_USERS);
        const metadataUtils = await createMetadata(URI_UTILS);

        // Delete file
        const fileData: FileId = {
            fileId: schema.data.fileId,
            targetId: schema.data.targetId,
        };
        const file = new Promise<File__Output>((resolve, reject) => {
            utilsClient.deleteFile(fileData, metadataUtils, (err, response) =>
                err || !response ? reject(err) : resolve(response),
            );
        });

        // Update user
        const data: User = {
            name: schema.data.name,
        };
        const user = new Promise<User__Output>((resolve, reject) => {
            usersClient.createUser(data, metadata, (err, response) =>
                err || !response ? reject(err) : resolve(response),
            );
        });

        await Promise.all([file, user]);
        const end = performance.now();
        return { duration: end - start };
    },
} satisfies Actions;
