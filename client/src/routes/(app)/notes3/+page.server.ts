import { error } from "@sveltejs/kit";
import type { PageServerLoad, Actions } from "./$types";
import type { UserId } from "../../../proto/proto/UserId";
import { createAuthMetadata, notesClient, usersClient } from "../../../grpc";
import type { Note__Output } from "../../../proto/proto/Note";
import type { NoteId } from "../../../proto/proto/NoteId";
import { URI_NOTES } from "$env/static/private";
import { z } from "zod";
import type { User__Output } from "../../../proto/proto/User";

export const load = (async ({ locals }) => {
    try {
        const start = performance.now();
        const userId = locals.userId;
        const request: UserId = { userId: userId };

        const metadata = await createAuthMetadata(userId);
        const stream = notesClient.getOnlyNotes(request, metadata);
        const notes: Note__Output[] = [];

        const userIds: Set<string> = new Set();

        await new Promise<Note__Output[]>((resolve, reject) => {
            stream.on("data", (note: Note__Output) => {
                notes.push(note);
                userIds.add(note.userId);
            });
            stream.on("end", () => resolve(notes));
            stream.on("error", (err: unknown) => reject(err));
        });

        const end = performance.now();

        const usersStream = usersClient.getUsers(
            { userIds: Array.from(userIds) },
            metadata,
        );
        const users: User__Output[] = [];

        const usersPromise = new Promise<User__Output[]>((resolve, reject) => {
            usersStream.on("data", (user: User__Output) => users.push(user));
            usersStream.on("end", () => resolve(users));
            usersStream.on("error", (err: unknown) => reject(err));
        });

        return {
            notes: notes,
            duration: end - start,
            streamed: {
                users: usersPromise,
            },
        };
    } catch (err) {
        console.error(err);
        throw error(500, "Could not load notes");
    }
}) satisfies PageServerLoad;

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

            // Do this 100 times
            for (let i = 0; i < 100; i++) {
                await new Promise<Note__Output>((resolve, reject) => {
                    notesClient.createNote(schema.data, metadata, (err, response) =>
                        err || !response ? reject(err) : resolve(response),
                    );
                });
            }

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
    deleteNote: async ({ locals, request }) => {
        const start = performance.now();

        const form = await request.formData();
        const id = form.get("id");

        if (!id) {
            throw error(400, "Missing id");
        }
        try {
            const data: NoteId = {
                noteId: id as string,
                userId: locals.userId,
            };

            const metadata = await createAuthMetadata(URI_NOTES);
            const note = await new Promise<Note__Output>((resolve, reject) => {
                notesClient.deleteNote(data, metadata, (err, response) =>
                    err || !response ? reject(err) : resolve(response),
                );
            });

            const end = performance.now();
            return {
                note: note,
                duration: end - start,
            };
        } catch (err) {
            console.error(err);
            throw error(500, "Failed to delete note");
        }
    },
} satisfies Actions;
