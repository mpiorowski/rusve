import { JWT_SECRET } from "$env/static/private";
import { Metadata } from "@grpc/grpc-js";
import jwt from "jsonwebtoken";

export function createMetadata(userId: string) {
    const metadata = new Metadata();
    const token = jwt.sign({ user_id: userId }, JWT_SECRET, {
        expiresIn: 3600,
        algorithm: "HS256",
    });
    metadata.set("authorization", `Bearer ${token}`);
    return metadata;
}
