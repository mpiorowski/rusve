import { SERVICE_ACCOUNT } from "$env/static/private";
import admin, { type ServiceAccount } from "firebase-admin";

export function getFirebaseServer() {
    if (!admin.apps.length) {
        const serviceAccount = JSON.parse(SERVICE_ACCOUNT) as ServiceAccount;
        admin.initializeApp({
            credential: admin.credential.cert(serviceAccount),
        });
    }
    return admin;
}
