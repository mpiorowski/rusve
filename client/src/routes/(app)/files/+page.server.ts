import { URI_UTILS } from "$env/static/private";
import { error } from "@sveltejs/kit";
import { z } from "zod";
import { fetchToken, utilsClient } from "../../../grpc";
import type { File__Output } from "../../../proto/proto/File";
import { FileType } from "../../../proto/proto/FileType";
import type { TargetId } from "../../../proto/proto/TargetId";
import type { Actions, PageServerLoad } from "./$types";

export const load = (async ({ locals }) => {
    try {
        const start = performance.now();
        const userId = locals.userId;
        const request: TargetId = { targetId: userId };

        const metadata = await fetchToken(URI_UTILS);
        const stream = utilsClient.getFiles(request, metadata);
        const files: File__Output[] = [];

        await new Promise<File__Output[]>((resolve, reject) => {
            stream.on("data", (file) => {
                files.push(file);
            });

            stream.on("end", () => {
                resolve(files);
            });

            stream.on("error", (err: unknown) => {
                reject(err);
            });
        });

        const end = performance.now();
        return {
            files: files,
            duration: end - start,
        };
    } catch (err) {
        console.error(err);
        throw error(500, "Could not load files");
    }
}) satisfies PageServerLoad;

export const actions = {
    createFile: async ({ request }) => {
        const form = await request.formData();
        const targetId = form.get("targetId");
        const type = form.get("type");
        const file = form.get("file");

        if (!(file instanceof File) || file.size === 0) {
            throw error(400, "Invalid request");
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

        const metadata = await fetchToken(URI_UTILS);
        await new Promise<File__Output>((resolve, reject) => {
            utilsClient.createFile(schema.data, metadata, (err, response) =>
                err || !response ? reject(err) : resolve(response),
            );
        });

        return { success: true };
    },
    deleteFile: async ({ request }) => {
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

        const metadata = await fetchToken(URI_UTILS);
        await new Promise<File__Output>((resolve, reject) => {
            utilsClient.deleteFile(schema.data, metadata, (err, response) =>
                err || !response ? reject(err) : resolve(response),
            );
        });

        return { success: true };
    },
} satisfies Actions;
