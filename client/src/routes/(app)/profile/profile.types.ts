import type { User__Output } from "$lib/proto/proto/User";

export type ProfileContext<F> = {
    user: User__Output;
    form: F;
};
