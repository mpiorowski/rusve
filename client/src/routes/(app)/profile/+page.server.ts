import {
    URI_USERS_GO,
    URI_USERS_RUST,
    URI_UTILS_GO,
    URI_UTILS_RUST,
} from "$env/static/private";
import {
    usersGoClient,
    usersRustClient,
    utilsGoClient,
    utilsRustClient,
} from "$lib/server/grpc";
import { createMetadata } from "$lib/server/metadata";
import type { File, File__Output } from "$lib/proto/proto/File";
import type { FileId } from "$lib/proto/proto/FileId";
import { FileType } from "$lib/proto/proto/FileType";
import type { User, User__Output } from "$lib/proto/proto/User";
import type { UserId } from "$lib/proto/proto/UserId";
import { PubSub } from "@google-cloud/pubsub";
import { error, fail } from "@sveltejs/kit";
import { z } from "zod";
import type { Actions, PageServerLoad } from "./$types";
import { safe } from "$lib/server/safe";
import { logger, perf } from "$lib/logging";

export const load = (async ({ locals, url }) => {
    try {
        const end = perf("LoadProfile");
        const start = performance.now();

        const isGo = url.searchParams.get("lang") === "go";
        const uri = isGo ? URI_USERS_GO : URI_USERS_RUST;
        const uriUtils = isGo ? URI_UTILS_GO : URI_UTILS_RUST;
        const client = isGo ? usersGoClient : usersRustClient;
        const utilsClient = isGo ? utilsGoClient : utilsRustClient;
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
                targetId: userId,
            };
            metadata = await createMetadata(uriUtils);
            file = new Promise((resolve, reject) => {
                utilsClient.getFile(fileId, metadata, (err, response) => {
                    if (!response || err) {
                        reject(err);
                    } else {
                        resolve({
                            id: response.id,
                            name: response.name,
                            base64: response.buffer.toString("base64"),
                        });
                    }
                });
            });
        }

        end();
        return {
            user: {
                ...user,
                id: user.id,
                avatarId: user.avatarId ? user.avatarId : undefined,
            },
            duration: performance.now() - start,
            stream: {
                file: file,
            },
        };
    } catch (err) {
        logger.error(err);
        throw error(500, "Could not load user");
    }
}) satisfies PageServerLoad;

export const actions = {
    createUser: async ({ request, locals }) => {
        try {
            const end = perf("CreateUser");
            const form = await request.formData();

            const lang = form.get("lang") ?? "rust";
            const isGo = lang === "go";
            const uri = isGo ? URI_USERS_GO : URI_USERS_RUST;
            const client = isGo ? usersGoClient : usersRustClient;

            const name = form.get("name");
            const avatarId = form.get("avatarId") ?? undefined;
            const schema = z
                .object({
                    name: z.string().max(1000),
                    avatar: z.string().optional(),
                })
                .safeParse({ name, avatarId });

            if (!schema.success) {
                logger.error(schema.error);
                return fail(409, { form: schema.error.flatten() });
            }

            const data: User = {
                id: locals.userId,
                name: schema.data.name,
                avatarId:
                    schema.data.avatar !== "" ? schema.data.avatar : undefined,
            };
            const metadata = await createMetadata(uri);
            await new Promise<void>((resolve, reject) => {
                client.updateUser(data, metadata, (err) =>
                    err ? reject(err) : resolve(),
                );
            });

            end();
            return { status: 200 };
        } catch (err) {
            logger.error(err);
            return fail(500, { error: "Could not create user" });
        }
    },
    createAvatar: async ({ request, locals }) => {
        const end = perf("CreateAvatar");
        const start = performance.now();

        const form = await safe(request.formData());
        if (!form.success) {
            return fail(400, { error: "Invalid form data" });
        }

        const lang = form.data.get("lang") ?? "rust";
        const isGo = lang === "go";
        const uri = isGo ? URI_USERS_GO : URI_USERS_RUST;
        const utilsUri = isGo ? URI_UTILS_GO : URI_UTILS_RUST;
        const client = isGo ? usersGoClient : usersRustClient;
        const utilsClient = isGo ? utilsGoClient : utilsRustClient;

        const targetId = locals.userId;
        const type = form.data.get("type");
        const file = form.data.get("file");
        const name = form.data.get("name");
        const avatarId = form.data.get("avatarId");

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
        const arrayBuffer = await safe(file.arrayBuffer());
        if (!arrayBuffer.success) {
            return fail(400, { error: "Invalid buffer" });
        }
        const buffer = Buffer.from(arrayBuffer.data);

        const schema = z
            .object({
                targetId: z.string().uuid(),
                fileName: z.string().min(1),
                type: z.nativeEnum(FileType),
                buffer: z.instanceof(Buffer),
                avatarId: z.string().uuid().or(z.literal("")),
                name: z.string().optional(),
            })
            .safeParse({
                targetId,
                fileName,
                type,
                buffer,
                avatarId,
                name,
            });
        if (!schema.success) {
            logger.error(schema.error);
            return fail(400, { error: "Invalid request" });
        }

        let metadata = await createMetadata(utilsUri);
        // Delete old avatar
        if (schema.data.avatarId) {
            const oldFileId: FileId = {
                fileId: schema.data.avatarId,
                targetId: schema.data.targetId,
            };
            const del = await safe(
                new Promise((resolve, reject) => {
                    utilsClient.deleteFile(
                        oldFileId,
                        metadata,
                        (err, response) =>
                            err || !response ? reject(err) : resolve(response),
                    );
                }),
            );
            if (!del.success) {
                return fail(500, { error: "Could not delete file" });
            }
        }

        // Create file
        const newFileData: File = {
            targetId: schema.data.targetId,
            name: schema.data.fileName,
            type: schema.data.type,
            buffer: schema.data.buffer,
        };
        const newFile = await safe(
            new Promise<File__Output>((resolve, reject) => {
                utilsClient.createFile(newFileData, metadata, (err, response) =>
                    err || !response ? reject(err) : resolve(response),
                );
            }),
        );
        if (!newFile.success || newFile.data.id.length === 0) {
            return fail(500, { error: "Could not create file" });
        }

        // Create avatar
        const data: User = {
            id: locals.userId,
            name: schema.data.name,
            avatarId: newFile.data.id,
        };
        metadata = await createMetadata(uri);
        const user = await safe(
            new Promise<void>((resolve, reject) => {
                client.updateUser(data, metadata, (err) =>
                    err ? reject(err) : resolve(),
                );
            }),
        );
        if (!user.success) {
            return fail(500, { error: "Could not create user" });
        }

        end();
        return {
            user: user.data,
            duration: performance.now() - start,
        };
    },
    deleteAvatar: async ({ request, locals }) => {
        try {
            const end = perf("DeleteAvatar");
            const start = performance.now();

            const form = await request.formData();

            const lang = form.get("lang") ?? "rust";
            const isGo = lang === "go";
            const uri = isGo ? URI_USERS_GO : URI_USERS_RUST;
            const utilsUri = isGo ? URI_UTILS_GO : URI_UTILS_RUST;
            const client = isGo ? usersGoClient : usersRustClient;
            const utilsClient = isGo ? utilsGoClient : utilsRustClient;

            const targetId = locals.userId;
            const fileId = form.get("fileId");
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
                logger.error(schema.error);
                return fail(409, { error: "Invalid request" });
            }

            const metadata = await createMetadata(uri);
            const metadataUtils = await createMetadata(utilsUri);

            // Delete file
            const fileData: FileId = {
                fileId: schema.data.fileId,
                targetId: schema.data.targetId,
            };
            await new Promise<void>((resolve, reject) => {
                utilsClient.deleteFile(fileData, metadataUtils, (err) =>
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

            end();
            return { duration: performance.now() - start };
        } catch (err) {
            logger.error(err);
            return fail(500, { error: "Could not delete avatar" });
        }
    },
    sendEmail: async ({ request, locals }) => {
        try {
            const end = perf("SendEmail");
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
                logger.error(schema.error);
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
                logger.info(`Message ${messageId} published.`);
            } catch (err) {
                logger.error("Received error while publishing: %s", err);
                return fail(500, { error: "Could not send email" });
            }
            end();
            return { duration: performance.now() - start };
        } catch (err) {
            logger.error(err);
            return fail(500, { error: "Could not send email" });
        }
    },
} satisfies Actions;
