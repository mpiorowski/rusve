import { grpcSafe } from "$lib/safe";
import { utilsService } from "$lib/server/grpc";
import { perf } from "$lib/server/logger";
import { createMetadata } from "$lib/server/metadata";
import { fail } from "@sveltejs/kit";
import { safe } from "$lib/safe";
import { FileTarget } from "$lib/proto/proto/FileTarget";

/** @type {import('./$types').PageServerLoad} */
export async function load({ locals, url }) {
    const end = perf("load_files");
    const metadata = createMetadata(locals.user.id);

    /**
     * Count files
     * @type {Promise<import("$lib/safe").Safe<import("$lib/proto/proto/Count").Count__Output>>}
     */
    const s1 = new Promise((r) => {
        utilsService.CountFilesByTargetId({}, metadata, grpcSafe(r));
    });

    /**
     * Get files
     */
    const limit = 10;
    const offset = (Number(url.searchParams.get("p") ?? 1) - 1) * limit;
    const filesStream = utilsService.GetFilesByTargetId(
        { offset, limit },
        metadata,
    );
    /** @type {Promise<import("$lib/proto/proto/File").File__Output[]>} */
    const p2 = new Promise((res, rej) => {
        /** @type {import("$lib/proto/proto/File").File__Output[]} */
        const files = [];
        filesStream.on("data", (data) => files.push(data));
        filesStream.on("error", (err) => rej(err));
        filesStream.on("end", () => res(files));
    });
    const s2 = safe(p2);

    // Wait for both
    const [d1, d2] = await Promise.all([s1, s2]);
    if (d1.error || d2.error) {
        return {
            error: "Failed to load files",
            files: [],
            total: 0,
            pageSize: limit,
        };
    }

    end();
    return {
        // We need to sort the files by created date because thes stream does not guarantee order
        error: "",
        files: d2.data
            .sort(
                (a, b) =>
                    new Date(b.created).getTime() -
                    new Date(a.created).getTime(),
            )
            .map((f) => ({
                ...f,
                file_buffer: Array.from(f.file_buffer),
            })),
        total: Number(d1.data.count),
        pageSize: limit,
    };
}

/** @type {import('./$types').Actions} */
export const actions = {
    uploadFile: async ({ locals, request }) => {
        const end = perf("upload_file");
        const form = await request.formData();

        const file = form.get("file");
        if (!file || !(file instanceof File)) {
            return fail(400, { error: "No file" });
        }
        // 10MB
        if (file.size > 1024 * 1024 * 10) {
            return fail(400, { error: "File too large" });
        }
        const arrayBuffer = await safe(file.arrayBuffer());
        if (arrayBuffer.error) {
            return fail(400, { error: arrayBuffer.msg });
        }
        const buffer = Buffer.from(arrayBuffer.data);

        const metadata = createMetadata(locals.user.id);
        const stream = utilsService.UploadFile(metadata);

        const chunkSize = 1024 * 64;
        let offset = 0;
        while (offset < buffer.length) {
            const chunk = buffer.subarray(offset, offset + chunkSize);
            /** @type {import('$lib/proto/proto/File').File} */
            const message = {
                file_name: file.name,
                file_size: String(file.size),
                file_type: file.type,
                file_target: FileTarget.FILE_DOCUMENT,
                file_buffer: chunk,
            };
            const res = safe(() => stream.write(message));
            if (res.error) {
                return fail(500, { error: res.msg });
            }
            offset += chunkSize;
        }
        stream.end();

        /** @type {import("$lib/proto/proto/File").File} */
        let newFile;
        /** @type {Promise<import("$lib/proto/proto/File").File>} */
        const p = new Promise((res, rej) => {
            stream.on("error", (err) => rej(err));
            stream.on("data", (data) => (newFile = data));
            stream.on("end", () => res(newFile));
        });
        const s = await safe(p);
        if (s.error) {
            return fail(500, { error: s.msg });
        }

        end();
        return {
            file: {
                ...s.data,
                file_buffer: [],
            },
        };
    },
};
