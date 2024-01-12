import { getFormValue } from "$lib/utils";
import { grpcSafe, safe } from "$lib/safe";
import { notesService } from "$lib/server/grpc";
import { createMetadata } from "$lib/server/metadata";
import { fail } from "@sveltejs/kit";
import { perf } from "$lib/server/logger";

/** @type {import('./$types').PageServerLoad} */
export async function load({ locals, url }) {
    const end = perf("load_notes");
    const metadata = createMetadata(locals.user.id);

    /**
     * Count notes
     * @type {Promise<import("$lib/safe").Safe<import("$lib/proto/proto/Count").Count__Output>>}
     */
    const s1 = new Promise((r) => {
        notesService.CountNotesByUserId({}, metadata, grpcSafe(r));
    });

    /**
     * Get notes
     */
    const offset = Number(url.searchParams.get("p") ?? 1) - 1;
    const limit = 10;

    /** @typedef {import("$lib/proto/proto/NoteResponse").NoteResponse__Output} Note */
    /** @type {import("@grpc/grpc-js").ClientReadableStream<Note>} */
    const notesStream = notesService.GetNotesByUserId(
        {
            offset: offset * limit,
            limit,
        },
        metadata,
    );
    /** @type {Promise<Note[]>} */
    const p2 = new Promise((res, rej) => {
        /** @type {Note[]} */
        const notes = [];
        notesStream.on("data", (note) => notes.push(note));
        notesStream.on("error", (err) => rej(err));
        notesStream.on("end", () => res(notes));
    });
    const s2 = safe(p2);

    // Wait for both
    const [d1, d2] = await Promise.all([s1, s2]);

    if (d1.error) {
        return {
            error: d1.msg,
            notes: [],
            total: 0,
            pageSize: limit,
        };
    }
    if (d2.error) {
        return {
            error: d2.msg,
            notes: [],
            total: 0,
            pageSize: limit,
        };
    }

    end();
    return {
        notes: d2.data.sort(
            (a, b) =>
                new Date(b.note?.created ?? 0).getTime() -
                new Date(a.note?.created ?? 0).getTime(),
        ),
        total: Number(d1.data.count),
        pageSize: limit,
    };
}

/** @type {import('./$types').Actions} */
export const actions = {
    insert: async ({ locals, request }) => {
        const end = perf("insert_note");
        const form = await request.formData();

        /** @type {import("$lib/proto/proto/Note").Note} */
        const data = {
            title: getFormValue(form, "title"),
            content: getFormValue(form, "content"),
        };
        const metadata = createMetadata(locals.user.id);
        /** @type {import("$lib/safe").Safe<import("$lib/proto/proto/Note").Note__Output>} */
        const req = await new Promise((r) => {
            notesService.CreateNote(data, metadata, grpcSafe(r));
        });

        if (req.error) {
            if (req.fields) {
                return fail(400, { fields: req.fields });
            }
            return fail(500, { error: req.msg });
        }

        end();
        return { note: req.data };
    },
};
