import { ENV } from "$env/static/private";
import { Metadata } from "@grpc/grpc-js";

const cacheToken = new Map<
    string,
    {
        expires: Date;
        token: string;
    }
>();

export async function createMetadata(serviceUrl: string) {
    const metadata = new Metadata();
    if (ENV === "development") {
        return metadata;
    }

    // check cache for token
    const cached = cacheToken.get(serviceUrl);
    if (cached && cached.expires > new Date()) {
        console.info("Using cached token");
        metadata.set("authorization", `Bearer ${cached.token}`);
        return metadata;
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
    metadata.set("authorization", `Bearer ${token}`);

    // cache token for 1 hour
    cacheToken.set(serviceUrl, {
        expires: new Date(Date.now() + 3600000),
        token,
    });

    return metadata;
}
