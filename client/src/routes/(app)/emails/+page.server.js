import { getFormValue } from "$lib/utils";
import { grpcSafe } from "$lib/safe";
import { utilsService } from "$lib/server/grpc";
import { perf } from "$lib/server/logger";
import { createMetadata } from "$lib/server/metadata";
import { fail } from "@sveltejs/kit";
import { safe } from "$lib/safe";

/** @type {import('./$types').PageServerLoad} */
export async function load({ locals, url }) {
    const end = perf("load_emails");
    const metadata = createMetadata(locals.user.id);

    /**
     * Count emails
     * @type {Promise<import("$lib/safe").Safe<import("$lib/proto/proto/Count").Count__Output>>}
     */
    const s1 = new Promise((r) => {
        utilsService.CountEmailsByTargetId({}, metadata, grpcSafe(r));
    });

    /**
     * Get emails
     */
    const limit = 10;
    const offset = (Number(url.searchParams.get("p") ?? 1) - 1) * limit;
    const emailsStream = utilsService.GetEmailsByTargetId(
        { offset, limit },
        metadata,
    );
    /** @type {Promise<import("$lib/proto/proto/Email").Email__Output[]>} */
    const p2 = new Promise((res, rej) => {
        /** @type {import("$lib/proto/proto/Email").Email__Output[]} */
        const emails = [];
        emailsStream.on("data", (data) => emails.push(data));
        emailsStream.on("error", (err) => rej(err));
        emailsStream.on("end", () => res(emails));
    });
    const s2 = safe(p2);

    // Wait for both
    const [d1, d2] = await Promise.all([s1, s2]);
    if (d1.error || d2.error) {
        return {
            error: "Failed to load emails",
            emails: [],
            total: 0,
            pageSize: limit,
        };
    }

    end();
    return {
        // We need to sort the notes by created date because thes stream does not guarantee order
        error: "",
        emails: d2.data.sort(
            (a, b) =>
                new Date(b.created).getTime() - new Date(a.created).getTime(),
        ),
        total: Number(d1.data.count),
        pageSize: limit,
    };
}

/** @type {import('./$types').Actions} */
export const actions = {
    sendEmail: async ({ locals, request }) => {
        const end = perf("send_email");
        const form = await request.formData();

        /** @type {import('$lib/proto/proto/Email').Email} */
        const data = {
            emailTo: getFormValue(form, "emailTo"),
            emailFrom: getFormValue(form, "emailFrom"),
            emailFromName: getFormValue(form, "emailFromName"),
            emailSubject: getFormValue(form, "emailSubject"),
            emailBody: getFormValue(form, "emailBody"),
        };

        /** @type {import("$lib/safe").Safe<import("$lib/proto/proto/Email").Email__Output>} */
        const res = await new Promise((r) => {
            utilsService.SendEmail(
                data,
                createMetadata(locals.user.id),
                grpcSafe(r),
            );
        });

        if (res.error) {
            if (res.fields) {
                return fail(400, { fields: res.fields });
            }
            return fail(400, { error: res.msg });
        }

        end();
        return {
            profile: res.data,
        };
    },
};
