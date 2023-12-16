import { getFormValue } from "$lib/utils";
import { grpcSafe, safe } from "$lib/safe";
import { notesService } from "$lib/server/grpc";
import { createMetadata } from "$lib/server/metadata";
import { pagination } from "$lib/ui/pagination";
import { fail } from "@sveltejs/kit";

/** @type {import('./$types').PageServerLoad} */
export async function load({ locals, url }) {
    /** @type {import("$lib/proto/proto/Note").Note__Output[]} */
    const notes = [];
    const metadata = createMetadata("", locals.user.id);
    const notesStream = notesService.GetNotesByUserId({}, metadata);

    /** @type {Promise<void>} */
    const p = new Promise((res, rej) => {
        notesStream.on("data", (data) => notes.push(data));
        notesStream.on("error", (err) => rej(err));
        notesStream.on("end", () => res());
    });
    const r = await safe(p);

    if (r.error) {
        return {
            error: r.msg,
            pagination: pagination(notes, 1),
        };
    }

    const page = Number(url.searchParams.get("p")) || 1;
    return {
        pagination: pagination(notes, page),
    };
}

/** @type {import('./$types').Actions} */
export const actions = {
    insert: async ({ locals, request }) => {
        const form = await request.formData();

        /** @type {import("$lib/proto/proto/Note").Note} */
        const data = {
            title: getFormValue(form, "title"),
            content: getFormValue(form, "content"),
        };
        const metadata = createMetadata("", locals.user.id);
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

        return { note: req.data };
    },
};
