import { getFirebaseServer } from "$lib/auth/firebase_server";
import { redirect } from "@sveltejs/kit";

/** @type {import('./$types').RequestHandler} */
export async function POST({
    request,
}: {
    request: Request;
}): Promise<Response> {
    const body = (await request.json()) as { idToken: string | undefined };

    if (!body.idToken) {
        console.info("No idToken found");
        throw redirect(303, "/auth");
    }

    const expiresIn = 60 * 60 * 24 * 5 * 1000;
    const admin = getFirebaseServer();

    const sessionCookie = await admin
        .auth()
        .createSessionCookie(body.idToken, { expiresIn });

    const header = new Headers();
    header.append(
        "set-cookie",
        `session=${sessionCookie}; Max-Age=${expiresIn}; SameSite=lax; HttpOnly; Path=/; ${
            !import.meta.env.DEV ? "Secure;" : ""
        }`,
    );

    return new Response("auth", {
        status: 200,
        headers: header,
    });
}

/** @type {import('./$types').RequestHandler} */
export function DELETE(): Response {
    const header = new Headers();
    header.append("set-cookie", `session=; Max-Age=0; Path=/`);

    return new Response("auth", {
        status: 200,
        headers: header,
    });
}
