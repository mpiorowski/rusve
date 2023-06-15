import { URI_USERS_GO, URI_USERS_RUST } from "$env/static/private";
import { usersGoClient, usersRustClient } from "$lib/grpc";
import { createMetadata } from "$lib/metadata";
import type { File, File__Output } from "$lib/proto/proto/File";
import type { FileId } from "$lib/proto/proto/FileId";
import { FileType } from "$lib/proto/proto/FileType";
import type { User, User__Output } from "$lib/proto/proto/User";
import type { UserId } from "$lib/proto/proto/UserId";
import { PubSub } from "@google-cloud/pubsub";
import { error, fail } from "@sveltejs/kit";
import { z } from "zod";
import type { Actions, PageServerLoad } from "./$types";

export const load = (async ({ locals, url }) => {
    try {
        const start = performance.now();

        const isGo = url.searchParams.get("lang") === "go";
        const uri = isGo ? URI_USERS_GO : URI_USERS_RUST;
        const client = isGo ? usersGoClient : usersRustClient;
        const userId = locals.userId;
        const request: UserId = { userId: userId };
        let metadata = await createMetadata(uri);
        const user = await new Promise<User__Output>((resolve, reject) => {
            client.getUser(request, metadata, (err, response) =>
                err || !response ? reject(err) : resolve(response),
            );
        });

        let file:
            | Promise<{ id: string; name: string; base64: string }>
            | undefined = undefined;
        if (user.avatarId) {
            const fileId: FileId = {
                fileId: user.avatarId,
                targetId: String(userId),
            };
            metadata = await createMetadata(uri);
            file = new Promise((resolve, reject) => {
                client.getFile(fileId, metadata, (err, response) => {
                    if (!response || err) {
                        reject(err);
                    } else {
                        resolve({
                            id: response.id,
                            name: response.name,
                            base64: Buffer.from(response.buffer).toString(
                                "base64",
                            ),
                        });
                    }
                });
            });
        }

        const end = performance.now();
        return {
            user: {
                ...user,
                id: String(user.id),
            },
            duration: end - start,
            stream: {
                file: file,
            },
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

            const lang = form.get("lang") ?? "rust";
            const isGo = lang === "go";
            const uri = isGo ? URI_USERS_GO : URI_USERS_RUST;
            const client = isGo ? usersGoClient : usersRustClient;

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
                id: locals.userId,
                name: schema.data.name,
                avatarId:
                    schema.data.avatar !== "" ? schema.data.avatar : undefined,
            };
            console.log(data);
            const metadata = await createMetadata(uri);
            await new Promise<void>((resolve, reject) => {
                client.updateUser(data, metadata, (err) =>
                    err ? reject(err) : resolve(),
                );
            });

            return { status: 200 };
        } catch (err) {
            console.error(err);
            return fail(500, { error: "Could not create user" });
        }
    },
    createAvatar: async ({ request, locals }) => {
        try {
            const start = performance.now();

            const form = await request.formData();

            const lang = form.get("lang") ?? "rust";
            const isGo = lang === "go";
            const uri = isGo ? URI_USERS_GO : URI_USERS_RUST;
            const client = isGo ? usersGoClient : usersRustClient;

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

            // supported file types (jpeg, jpg, png, gif, webp)
            if (
                !file.type.startsWith("image/jpeg") &&
                !file.type.startsWith("image/jpg") &&
                !file.type.startsWith("image/png") &&
                !file.type.startsWith("image/gif") &&
                !file.type.startsWith("image/webp")
            ) {
                return fail(400, { error: "Invalid file type" });
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

            let metadata = await createMetadata(uri);
            // Delete old avatar
            if (schema.data.avatar) {
                const oldFileId: FileId = {
                    fileId: schema.data.avatar,
                    targetId: schema.data.targetId,
                };
                await new Promise((resolve, reject) => {
                    client.deleteFile(oldFileId, metadata, (err, response) =>
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
                    client.createFile(newFileData, metadata, (err, response) =>
                        err || !response ? reject(err) : resolve(response),
                    );
                },
            );
            if (!newFile.id) {
                return fail(500, { error: "Could not create file" });
            }

            // Create avatar
            const data: User = {
                id: locals.userId,
                name: schema.data.name,
                avatarId: newFile.id,
            };
            metadata = await createMetadata(uri);
            const user = await new Promise<void>((resolve, reject) => {
                client.updateUser(data, metadata, (err) =>
                    err ? reject(err) : resolve(),
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
        try {
            const start = performance.now();

            const form = await request.formData();

            const lang = form.get("lang") ?? "rust";
            const isGo = lang === "go";
            const uri = isGo ? URI_USERS_GO : URI_USERS_RUST;
            const client = isGo ? usersGoClient : usersRustClient;

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

            const metadata = await createMetadata(uri);
            const metadataUtils = await createMetadata(uri);

            // Delete file
            const fileData: FileId = {
                fileId: schema.data.fileId,
                targetId: schema.data.targetId,
            };
            await new Promise<void>((resolve, reject) => {
                client.deleteFile(fileData, metadataUtils, (err) =>
                    err ? reject(err) : resolve(),
                );
            });

            // Update user
            const data: User = {
                id: locals.userId,
                name: schema.data.name,
            };
            await new Promise<void>((resolve, reject) => {
                client.updateUser(data, metadata, (err) =>
                    err ? reject(err) : resolve(),
                );
            });

            const end = performance.now();
            return { duration: end - start };
        } catch (err) {
            console.error(err);
            return fail(500, { error: "Could not delete avatar" });
        }
    },
    sendEmail: async ({ request, locals }) => {
        try {
            const start = performance.now();

            const form = await request.formData();
            const email = locals.email;
            const subject = form.get("subject");
            const message = form.get("message");

            const schema = z
                .object({
                    email: z.string().email(),
                    subject: z.string().min(1),
                    message: z.string().min(1),
                })
                .safeParse({
                    email,
                    subject,
                    message,
                });

            if (!schema.success) {
                console.error(schema.error);
                return fail(409, { form: schema.error.flatten().fieldErrors });
            }

            const data = {
                email: schema.data.email,
                subject: schema.data.subject,
                message: schema.data.message,
            };

            try {
                const dataBuffer = Buffer.from(JSON.stringify(data));
                const pubSubClient = new PubSub();
                const messageId = await pubSubClient
                    .topic("email")
                    .publishMessage({ data: dataBuffer });
                console.log(`Message ${messageId} published.`);
            } catch (err) {
                console.error("Received error while publishing: %s", err);
                return fail(500, { error: "Could not send email" });
            }

            const end = performance.now();
            return { duration: end - start };
        } catch (err) {
            console.error(err);
            return fail(500, { error: "Could not send email" });
        }
    },
} satisfies Actions;
