import { PUBLIC_API_KEY, PUBLIC_AUTH_DOMAIN } from "$env/static/public";
import { initializeApp } from "firebase/app";
import { getAuth, setPersistence, type Persistence } from "firebase/auth";

export function getFirebaseClient() {
    const persistance: Persistence = { type: "NONE" };
    const firebaseConfig = {
        apiKey: PUBLIC_API_KEY,
        authDomain: PUBLIC_AUTH_DOMAIN,
    };
    const app = initializeApp(firebaseConfig);
    const auth = getAuth(app);
    void setPersistence(auth, persistance);
    return auth;
}
