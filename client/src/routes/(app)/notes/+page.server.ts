import { error, fail } from "@sveltejs/kit";
import type { PageServerLoad, Actions } from "./$types";
import { URI_NOTES_GO, URI_NOTES_RUST, URI_USERS } from "$env/static/private";
import type { UserId } from "$lib/proto/proto/UserId";
import type { Note__Output } from "$lib/proto/proto/Note";
import type { User__Output } from "$lib/proto/proto/User";
import type { NoteId } from "$lib/proto/proto/NoteId";
import { createMetadata } from "$lib/metadata";
import { notesGoClient, notesRustClient, usersClient } from "$lib/grpc";
import { z } from "zod";
import type { Metadata } from "@grpc/grpc-js";

export const load = (async ({ locals }) => {
    try {
        const userId = locals.userId;
        const request: UserId = { userId: userId };
        const userIds = new Set<string>();

        /**
         * Get notes from Rust server
         */
        const startRust = performance.now();
        let metadata = await createMetadata(URI_NOTES_RUST);
        const streamRust = notesRustClient.getNotes(request, metadata);
        const notesRust: Note__Output[] = [];

        await new Promise<Note__Output[]>((resolve, reject) => {
            streamRust.on("data", (note: Note__Output) => {
                notesRust.push(note);
                userIds.add(note.userId);
            });
            streamRust.on("end", () => resolve(notesRust));
            streamRust.on("error", (err: unknown) => reject(err));
        });
        const timeRust = performance.now() - startRust;

        /**
         * Get users from Go server
         */
        const startGo = performance.now();
        metadata = await createMetadata(URI_NOTES_GO);
        const streamGo = notesGoClient.getNotes(request, metadata);
        const notesGo: Note__Output[] = [];

        await new Promise<Note__Output[]>((resolve, reject) => {
            streamGo.on("data", (note: Note__Output) => {
                notesGo.push(note);
            });
            streamGo.on("end", () => resolve(notesGo));
            streamGo.on("error", (err: unknown) => reject(err));
        });
        const timeGo = performance.now() - startGo;

        /**
         * Get users from Rust server
         */
        metadata = await createMetadata(URI_USERS);
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
            notesRust: notesRust.slice(0, 1),
            timeRust: timeRust,
            notesGo: notesGo.slice(0, 1),
            timeGo: timeGo,
            length: notesRust.length,
            stream: {
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
        const type = form.get("type");

        const data = {
            title: title,
            content: content,
            type: type,
            userId: locals.userId,
        };

        const schema = z
            .object({
                userId: z.string().uuid(),
                title: z.string().min(1).max(100),
                content: z.string().min(1).max(1000),
                type: z.union([z.literal("go"), z.literal("rust")]),
            })
            .safeParse(data);

        if (!schema.success) {
            return fail(409, { error: schema.error.flatten() });
        }

        try {
            let metadata: Metadata;
            if (schema.data.type === "rust") {
                metadata = await createMetadata(URI_NOTES_RUST);
                await new Promise<Note__Output>((resolve, reject) => {
                    notesRustClient.createNote(
                        schema.data,
                        metadata,
                        (err, response) =>
                            err || !response ? reject(err) : resolve(response),
                    );
                });
            } else {
                metadata = await createMetadata(URI_NOTES_GO);
                await new Promise<Note__Output>((resolve, reject) => {
                    notesGoClient.createNote(
                        schema.data,
                        metadata,
                        (err, response) =>
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

        const schema = z
            .object({
                id: z.string().uuid(),
            })
            .safeParse({ id: id });
        if (!schema.success) {
            throw error(400, "Missing id");
        }
        try {
            const data: NoteId = {
                noteId: schema.data.id,
                userId: locals.userId,
            };

            const metadata = await createMetadata(URI_NOTES_RUST);
            const note = await new Promise<Note__Output>((resolve, reject) => {
                notesRustClient.deleteNote(data, metadata, (err, response) =>
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
