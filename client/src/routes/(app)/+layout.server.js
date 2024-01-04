import { usersService } from "$lib/server/grpc";
import { createMetadata } from "$lib/server/metadata";
import { grpcSafe } from "$lib/safe";
import { error } from "@sveltejs/kit";

/** @type {import('./$types').LayoutServerLoad} */
export async function load({ locals }) {
    /** @type {import('$lib/safe').Safe<import('$lib/proto/proto/Profile').Profile__Output>} */
    const profile = await new Promise((r) => {
        usersService.GetProfileByUserId(
            {},
            createMetadata(locals.user.id),
            grpcSafe(r),
        );
    });
    if (profile.error) {
        throw error(500, profile.msg);
    }
    return {
        profile: profile.data,
        email: locals.user.email,
        avatar: locals.user.avatar,
    };
}
