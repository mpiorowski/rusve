import { SERVICE_ACCOUNT } from "$env/static/private";
import admin, { type ServiceAccount } from "firebase-admin";
import { safe, type Safe } from "./safe";

export function getFirebaseServer(): Safe<typeof admin> {
    if (!admin.apps.length) {
        const serviceAccount = safe(
            () => JSON.parse(SERVICE_ACCOUNT) as ServiceAccount,
        );
        if (!serviceAccount.success) {
            return { success: false, error: serviceAccount.error };
        }
        const app = safe(() =>
            admin.initializeApp({
                credential: admin.credential.cert(serviceAccount.data),
            }),
        );
        if (!app.success) {
            return { success: false, error: app.error };
        }
    }
    return { success: true, data: admin };
}
