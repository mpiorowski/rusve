import { URI_NOTES } from "$env/static/private";
import { error, type Actions } from "@sveltejs/kit";
import { z } from "zod";
import { createAuthMetadata, notesClient } from "../../grpc";
import type { Note__Output } from "$lib/proto/proto/Note";

export const actions = {
    createNote: async ({ locals, request }) => {
        const start = performance.now();

        const form = await request.formData();
        const title = form.get("title");
        const content = form.get("content");

        const data = {
            title: title,
            content: content,
            userId: locals.userId,
        };

        const schema = z
            .object({
                userId: z.string().uuid(),
                title: z.string().min(1),
                content: z.string().min(1),
            })
            .safeParse(data);

        if (!schema.success) {
            throw error(400, "Invalid request");
        }

        try {
            const metadata = await createAuthMetadata(URI_NOTES);
            await new Promise<Note__Output>((resolve, reject) => {
                notesClient.createNote(schema.data, metadata, (err, response) =>
                    err || !response ? reject(err) : resolve(response),
                );
            });

            const end = performance.now();
            return {
                success: true,
                duration: end - start,
            };
        } catch (err) {
            console.error(err);
            throw error(500, "Could not create note");
        }
    },
} satisfies Actions;
