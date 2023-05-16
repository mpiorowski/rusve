import { fail } from "@sveltejs/kit";

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
        console.error(error.details);
    } else {
        console.error(error);
    }
};
