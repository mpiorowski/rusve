import type { User__Output } from "$lib/proto/proto/User";

export type ProfileContext<A> = {
    user: User__Output;
    form: A;
};
