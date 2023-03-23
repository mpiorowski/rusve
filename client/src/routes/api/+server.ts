import type { RequestHandler } from "./$types";

export const POST = (async ({ request }) => {
    // Get base64 encoded string
    const data = await request.formData();
    const base64 = data.get("base64") as string;
    const name = data.get("name") as string;

    // Convert base64 string to buffer
    const buffer = Buffer.from(base64, "base64");
    return new Response(buffer, {
        headers: {
            "content-type": "application/octet-stream",
            "content-disposition": `attachment; filename=${name}`,
        },
    });
}) satisfies RequestHandler;
