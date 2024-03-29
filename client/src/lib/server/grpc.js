import protoLoader from "@grpc/proto-loader";
import { credentials, loadPackageDefinition } from "@grpc/grpc-js";
import { ENV, USERS_URI, NOTES_URI, UTILS_URI } from "$env/static/private";

export const packageDefinition = protoLoader.loadSync(
    "./src/lib/proto/main.proto",
    {
        keepCase: true,
        longs: String,
        defaults: true,
        oneofs: true,
    },
);

const proto = /** @type {import("$lib/proto/main").ProtoGrpcType} */ (
    /** @type {unknown} */ (loadPackageDefinition(packageDefinition))
);

/** @type {import("@grpc/grpc-js").ChannelCredentials} */
const cr =
    ENV === "production"
        ? credentials.createSsl()
        : credentials.createInsecure();

export const usersService = new proto.proto.UsersService(USERS_URI, cr);
export const notesService = new proto.proto.NotesService(NOTES_URI, cr);
export const utilsService = new proto.proto.UtilsService(UTILS_URI, cr);
