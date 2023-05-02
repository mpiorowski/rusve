import { STRIPE_API_KEY } from "$env/static/private";
import Stripe from "stripe";

export function getStripe() {
    const stripe = new Stripe(STRIPE_API_KEY, {
        apiVersion: "2022-11-15",
        typescript: true,
    });
    return stripe;
}

const cache = new Map<string, Date>();
/**
 * Check subscription only for certain pages
 * Cache it for 1 hour
 */
export async function checkSubscription(paymentId: string) {
    const stripe = getStripe();

    if (cache.has(paymentId)) {
        const date = cache.get(paymentId);
        if (date && date.getTime() + 1000 * 60 * 60 > Date.now()) {
            return true;
        }
    }

    const stripeCustomer = (await stripe.customers.retrieve(paymentId, {
        expand: ["subscriptions"],
    })) as {
        subscriptions: {
            data: Stripe.Subscription[];
        };
    };
    const isSubscribed = stripeCustomer.subscriptions.data.some(
        (sub) => sub.status === "active",
    );
    if (isSubscribed) {
        cache.set(paymentId, new Date());
    }
    return isSubscribed;
}
