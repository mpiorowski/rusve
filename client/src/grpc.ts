import protoLoader from "@grpc/proto-loader";
import { credentials, loadPackageDefinition, Metadata } from "@grpc/grpc-js";
import type { ProtoGrpcType } from "./proto/main";
import {
    URI_USERS,
    URI_NOTES,
    ENV,
    URI_UTILS,
    SECRET,
} from "$env/static/private";
import { createHmac } from "crypto";

export const fetchToken = async (serviceUrl: string) => {
    const timestamp = Date.now().toString();
    const hmac = createHmac("sha256", SECRET);
    hmac.update(timestamp);
    const token = `${timestamp}.${hmac.digest("hex")}`;
    const metadata = new Metadata();
    metadata.set("authorization", `Bearer ${token}`);
    return metadata;
};

export const packageDefinition = protoLoader.loadSync("./src/proto/main.proto");
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
