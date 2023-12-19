import protoLoader from "@grpc/proto-loader";
import {
    ChannelCredentials,
    credentials,
    loadPackageDefinition,
} from "@grpc/grpc-js";
import type { ProtoGrpcType } from "$lib/proto/main";
import {
    ENV,
    URI_NOTES_RUST,
    URI_NOTES_GO,
    URI_UTILS_RUST,
    URI_UTILS_GO,
    URI_USERS_RUST,
    URI_USERS_GO,
} from "$env/static/private";

export const packageDefinition = protoLoader.loadSync(
    "./src/lib/proto/main.proto",
    {
        keepCase: false,
        longs: String,
        enums: String,
        defaults: true,
        oneofs: true,
    },
);
export const proto = loadPackageDefinition(
    packageDefinition,
) as unknown as ProtoGrpcType;

const cr: ChannelCredentials =
    ENV === "production"
        ? credentials.createSsl()
        : credentials.createInsecure();

export const usersRustClient = new proto.proto.UsersService(URI_USERS_RUST, cr);

export const usersGoClient = new proto.proto.UsersService(URI_USERS_GO, cr);

export const utilsRustClient = new proto.proto.UtilsService(URI_UTILS_RUST, cr);

export const utilsGoClient = new proto.proto.UtilsService(URI_UTILS_GO, cr);

export const notesRustClient = new proto.proto.NotesService(URI_NOTES_RUST, cr);

export const notesGoClient = new proto.proto.NotesService(URI_NOTES_GO, cr);