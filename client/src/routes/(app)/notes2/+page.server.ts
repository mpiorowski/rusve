import { URI_NOTES } from "$env/static/private";
import { error } from "@sveltejs/kit";
import { createAuthMetadata, notesClient, usersClient } from "../../../grpc";
import type { Note__Output } from "../../../proto/proto/Note";
import type { NoteId } from "../../../proto/proto/NoteId";
import type { User__Output } from "../../../proto/proto/User";
import type { UserId } from "../../../proto/proto/UserId";
import type { PageServerLoad, Actions } from "./$types";

export const load = (async ({ locals }) => {
    try {
        const start = performance.now();
        const userId = locals.userId;
        const request: UserId = { userId: userId };

        const metadata = await createAuthMetadata(userId);
        const stream = notesClient.getOnlyNotes(request, metadata);
        const notes: Note__Output[] = [];

        const users: Promise<User__Output>[] = [];

        await new Promise<Note__Output[]>((resolve, reject) => {
            stream.on("data", (note: Note__Output) => {
                notes.push(note);
                // get user
                const user = new Promise<User__Output>((resolve, reject) => {
                    usersClient.getUser(
                        { userId: note.userId },
                        metadata,
                        (err, response) =>
                            err || !response ? reject(err) : resolve(response),
                    );
                });
                users.push(user);
            });

            stream.on("end", () => {
                resolve(notes);
            });

            stream.on("error", (err: unknown) => {
                reject(err);
            });
        });

        const end = performance.now();
        const allUsers = Promise.all(users);
        return {
            notes: notes,
            duration: end - start,
            streamed: {
                users: allUsers,
            },
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
