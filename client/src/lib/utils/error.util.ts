import { toast, ToastType } from "@mpiorowski/svelte-init";
import { fail } from "@sveltejs/kit";
import { t } from "svelte-i18n";
import type { ZodError } from "zod";

export type GrpcError = {
    details: string;
};

const isGrpcError = (error: unknown): error is GrpcError => {
    if (error && typeof error === "object" && "details" in error) {
        return true;
    }
    return false;
};

export const handleGrpcError = (error: unknown) => {
    if (isGrpcError(error)) {
        return fail(400, { error: error.details });
    } else {
        return fail(400, { error: "errors.somethingWentWrong" });
    }
};

export const handleError = (error: unknown) => {
    console.error(error);
    if (isGrpcError(error)) {
        t.subscribe((value) => {
            toast(value(error.details), ToastType.ERROR);
        });
    } else {
        t.subscribe((value) => {
            toast(value("errors.somethingWentWrong"), ToastType.ERROR);
        });
    }
};

export const handleZodError = <T>(
    error: ZodError<T>,
    id: string | null = null,
) => {
    const errors = error.flatten().fieldErrors;
    console.error(errors);
    return fail(400, { errors: errors, id: id || "" });
};

export const zodError = (
    field: string,
    errors?: { [key: string]: string[] | undefined },
) => {
    const val = errors?.[field];
    if (val) {
        return val.join(", ");
    }
    return "";
};
