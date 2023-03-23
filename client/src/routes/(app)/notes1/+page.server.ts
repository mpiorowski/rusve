import { error } from "@sveltejs/kit";
import type { PageServerLoad, Actions } from "./$types";
import { URI_NOTES } from "$env/static/private";
import type { UserId } from "$lib/proto/proto/UserId";
import { createAuthMetadata, notesClient } from "../../../grpc";
import type { Note__Output } from "$lib/proto/proto/Note";
import type { NoteId } from "$lib/proto/proto/NoteId";

export const load = (async ({ locals }) => {
    try {
        const start = performance.now();
        const userId = locals.userId;
        const request: UserId = { userId: userId };

        const metadata = await createAuthMetadata(userId);
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

        const end = performance.now();
        return {
            notes: notes,
            duration: end - start,
        };
    } catch (err) {
        console.error(err);
        throw error(500, "Could not load notes");
    }
}) satisfies PageServerLoad;

export const actions = {
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
