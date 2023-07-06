import { redirect } from "@sveltejs/kit";
import type { Actions } from "./$types";
import { getFirebaseServer } from "$lib/server/firebase_server";

export const actions = {
    default: async ({ request, cookies }) => {
        const form = await request.formData();
        const idToken = form.get("idToken");
        if (!idToken || typeof idToken !== "string") {
            throw redirect(303, "/auth");
        }

        // Cookie expires in 5 days
        const expiresIn = 60 * 60 * 24 * 5 * 1000;
        const admin = getFirebaseServer();

        let sessionCookie = "";
        try {
            sessionCookie = await admin
                .auth()
                .createSessionCookie(idToken, { expiresIn });
        } catch (err) {
            console.error("Error creating session cookie", err);
            throw redirect(303, "/auth");
        }

        cookies.set("session", sessionCookie, {
            maxAge: 60 * 60 * 24 * 30, // 30 days
            path: "/",
            httpOnly: true,
            secure: true,
            sameSite: "lax",
        });

        throw redirect(303, "/");
    },
} satisfies Actions;
