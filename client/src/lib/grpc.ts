import protoLoader from "@grpc/proto-loader";
import { credentials, loadPackageDefinition } from "@grpc/grpc-js";
import type { ProtoGrpcType } from "$lib/proto/main";
import { URI_USERS, URI_NOTES_RUST, URI_UTILS, ENV, URI_NOTES_GO } from "$env/static/private";

export const packageDefinition = protoLoader.loadSync("./src/lib/proto/main.proto");
export const proto = loadPackageDefinition(
    packageDefinition,
) as unknown as ProtoGrpcType;

export const usersClient = new proto.proto.UsersService(
    URI_USERS,
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

export const notesRustClient = new proto.proto.NotesService(
    URI_NOTES_RUST,
    ENV === "production"
        ? credentials.createSsl()
        : credentials.createInsecure(),
);

export const notesGoClient = new proto.proto.NotesService(
    URI_NOTES_GO,
    ENV === "production"
        ? credentials.createSsl()
        : credentials.createInsecure(),
);
