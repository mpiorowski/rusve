import protoLoader from "@grpc/proto-loader";
import { credentials, loadPackageDefinition, Metadata } from "@grpc/grpc-js";
import type { ProtoGrpcType } from "./proto/main";
import { URI_USERS, URI_NOTES, ENV } from "$env/static/private";

const cacheToken = new Map<
    string,
    {
        expires: Date;
        metadata: Metadata;
    }
>();

export const fetchToken = async (serviceUrl: string) => {
    if (ENV === "development") {
        return new Metadata();
    }

    // check cache for token
    const cached = cacheToken.get(serviceUrl);
    if (cached && cached.expires > new Date()) {
        console.info("Using cached token");
        return cached.metadata;
    }

    console.info("Fetching token");
    const tokenFetch = await fetch(
        `http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/identity?audience=https://${serviceUrl}`,
        {
            method: "GET",
            headers: {
                "Metadata-Flavor": "Google",
            },
        },
    );
    const token = await tokenFetch.text();
    const metadata = new Metadata();
    metadata.add("authorization", `Bearer ${token}`);

    // cache token for 1 hour
    cacheToken.set(serviceUrl, {
        expires: new Date(Date.now() + 3600000),
        metadata,
    });

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
