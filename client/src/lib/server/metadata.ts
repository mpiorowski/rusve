import { ENV } from "$env/static/private";
import { Metadata } from "@grpc/grpc-js";
import jwt from "jsonwebtoken";
import fs from "fs";
import { debug } from "$lib/logging";

/**
 * Cache the tokens for 50 minutes
 * @param serviceUrl The URL of the service to authorize
 * @returns A Metadata object with the correct authorization headers
 */
const cacheToken = new Map<
    string,
    {
        expires: Date;
        gcpToken: string;
        oauthToken: string;
    }
>();

/**
 * Load the private key from the file system
 */
const key = fs.readFileSync("./src/lib/server/private.key");

/**
 * Create a Metadata object with the correct authorization headers
 * @param serviceUrl The URL of the service to authorize
 * @returns A Metadata object with the correct authorization headers
 *
 * The GCP token is only needed when deploying to their cloud, otherwise delete it.
 * X-authorization is the OAuth2 token, which is used to authenticate with the service, always needed.
 */
export async function createMetadata(serviceUrl: string): Promise<Metadata> {
    const metadata = new Metadata();

    // Check cache for token
    const cached = cacheToken.get(serviceUrl);
    if (cached && cached.expires > new Date()) {
        debug("Using cached token");
        metadata.set("authorization", `bearer ${cached.gcpToken}`);
        metadata.set("x-authorization", `bearer ${cached.oauthToken}`);
        return metadata;
    }

    let gcpToken = "";
    let oauthToken = "";

    const tokenPayload = {
        sub: serviceUrl,
    };

    // Generate and sign the OAuth2 token
    oauthToken = jwt.sign(tokenPayload, key, {
        algorithm: "RS256",
        expiresIn: "1h",
    });

    // Fetch the GCP token
    if (ENV === "production") {
        try {
            const tokenFetch = await fetch(
                `http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/identity?audience=https://${serviceUrl}`,
                {
                    method: "GET",
                    headers: {
                        "Metadata-Flavor": "Google",
                    },
                },
            );
            gcpToken = await tokenFetch.text();
        } catch (err) {
            console.error("Failed to fetch GCP token", err);
        }
    }

    // Cache token for 50 minutes
    cacheToken.set(serviceUrl, {
        expires: new Date(Date.now() + 50 * 60 * 1000),
        gcpToken,
        oauthToken,
    });

    metadata.set("authorization", `bearer ${gcpToken}`);
    metadata.set("x-authorization", `bearer ${oauthToken}`);
    return metadata;
}
