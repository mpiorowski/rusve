import protoLoader from "@grpc/proto-loader";
import { credentials, loadPackageDefinition, Metadata } from "@grpc/grpc-js";
import type { ProtoGrpcType } from "$lib/proto/main";
import {
    URI_USERS,
    URI_NOTES,
    URI_UTILS,
    ENV,
    JWT_SECRET,
} from "$env/static/private";
import jwt from "jsonwebtoken";

import { writeFileSync } from "fs";
import mainProto from "$lib/proto/main.proto?raw";
writeFileSync("/tmp/main.proto", mainProto);

export const createAuthMetadata = async (userId: string) => {
    const metadata = new Metadata();
    const token = jwt.sign({ user_id: userId }, JWT_SECRET, {
        expiresIn: 3600,
        algorithm: "HS256",
    });
    metadata.set("authorization", `Bearer ${token}`);
    return metadata;
};

export const packageDefinition = protoLoader.loadSync("/tmp/main.proto");
export const proto = loadPackageDefinition(
    packageDefinition,
) as unknown as ProtoGrpcType;

export const usersClient = new proto.proto.UsersService(
    URI_USERS,
    ENV === "production"
        ? credentials.createSsl()
        : credentials.createInsecure(),
);

export const notesClient = new proto.proto.NotesService(
    URI_NOTES,
    ENV === "production"
        ? credentials.createSsl()
        : credentials.createInsecure(),
);

export const utilsClient = new proto.proto.UtilsService(
    URI_UTILS,
    ENV === "production"
        ? credentials.createSsl()
        : credentials.createInsecure(),
);
