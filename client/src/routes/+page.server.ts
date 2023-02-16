import { error } from "@sveltejs/kit";
import type { PageServerLoad, Actions } from "./$types";
import type { UserId } from "../proto/proto/UserId";
import { fetchToken, notesClient } from "../grpc";
import type { Note, Note__Output } from "../proto/proto/Note";
import type { NoteId } from "../proto/proto/NoteId";
import { performanceLogger } from "$lib/utils/logging.util";
import { URI_NOTES } from "$env/static/private";

export const load = (async ({ locals }) => {
    try {
        const start = performance.now();
        const end = performanceLogger("getNotes");
        const userId = locals.userId;
        const request: UserId = { userId: userId };

        const metadata = await fetchToken(URI_NOTES);
        const stream = notesClient.getNotes(request, metadata);
        const notes: Note__Output[] = [];

        await new Promise<Note__Output[]>((resolve, reject) => {
            stream.on("data", (note) => {
                notes.push(note);
            });

            stream.on("end", () => {
                resolve(notes);
            });

            stream.on("error", (err: unknown) => {
                reject(err);
            });
        });

        end();
        const end2 = performance.now();

        return {
            notes: notes,
            duration: end2 - start,
        };
    } catch (err) {
        console.error(err);
        throw error(500, "Could not load notes");
    }
}) satisfies PageServerLoad;

export const actions = {
    createNote: async ({ locals, request }) => {
        const start = performance.now();
        const end = performanceLogger("createNote");

        const form = await request.formData();
        const title = form.get("title");
        const content = form.get("content");

        if (!title || !content) {
            throw error(400, "Missing title or content");
        }

        try {
            const note: Note = {
                title: title as string,
                content: content as string,
                userId: locals.userId,
            };

            const metadata = await fetchToken(URI_NOTES);
            const promise = new Promise<Note__Output>((resolve, reject) => {
                notesClient.createNote(note, metadata, (err, response) =>
                    err || !response ? reject(err) : resolve(response),
                );
            });

            end();
            const end2 = performance.now();
            return {
                note: await promise,
                duration: end2 - start,
            };
        } catch (err) {
            console.error(err);
            throw error(500, "Could not create note");
        }
    },
    deleteNote: async ({ locals, request }) => {
        const start = performance.now();
        const end = performanceLogger("deleteNote");

        const form = await request.formData();
        const id = form.get("id");

        if (!id) {
            throw error(400, "Missing id");
        }
        try {
            const request: NoteId = {
                noteId: id as string,
                userId: locals.userId,
            };

            const metadata = await fetchToken(URI_NOTES);
            const promise = new Promise<Note__Output>((resolve, reject) => {
                notesClient.deleteNote(request, metadata, (err, response) =>
                    err || !response ? reject(err) : resolve(response),
                );
            });

            end();
            const end2 = performance.now();

            return {
                note: await promise,
                duration: end2 - start,
            };
        } catch (err) {
            console.error(err);
            throw error(500, "Failed to delete note");
        }
    },
} satisfies Actions;
