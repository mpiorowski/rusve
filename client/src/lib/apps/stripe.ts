import { STRIPE_KEY } from "$env/static/private";
import Stripe from "stripe";

export function getStripe() {
    const stripe = new Stripe(STRIPE_KEY, {
        apiVersion: "2022-11-15",
        typescript: true,
    });
    return stripe;
}
