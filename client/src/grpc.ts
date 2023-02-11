import protoLoader from "@grpc/proto-loader";
import { credentials, loadPackageDefinition, Metadata } from "@grpc/grpc-js";
import type { ProtoGrpcType } from "./proto/main";
import { URI_USERS, URI_NOTES, NODE_ENV } from "$env/static/private";

export const packageDefinition = protoLoader.loadSync("../proto/main.proto");
export const proto = loadPackageDefinition(
    packageDefinition,
) as unknown as ProtoGrpcType;

export const metadata = new Metadata();

export const usersClient = new proto.proto.UsersService(
    URI_USERS,
    NODE_ENV === "production"
        ? credentials.createSsl()
        : credentials.createInsecure(),
);

export const notesClient = new proto.proto.NotesService(
    URI_NOTES,
    NODE_ENV === "production"
        ? credentials.createSsl()
        : credentials.createInsecure(),
);
