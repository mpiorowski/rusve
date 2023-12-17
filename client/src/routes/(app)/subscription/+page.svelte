<script>
    import { page } from "$app/stores";
    import Button from "$lib/form/Button.svelte";
    import { toast } from "$lib/ui/toast";

    /** @type {import("./$types").PageData} */
    export let data;

    $: if ($page.url.searchParams.has("success")) {
        toast.success("Success", "Your subscription has been activated.");
    } else if ($page.url.searchParams.has("cancel")) {
        toast.info("Cancelled", "Your subscription has been cancelled.");
    }
</script>

<div class="flex flex-col gap-2">
    <h1 class="mb-6">Subscription status: {data.subscriptionActive}</h1>
    <form action="?/createStripeCheckout" method="post">
        <Button>Checkout</Button>
    </form>
    <form action="?/createStripePortal" method="post">
        <Button>Manage subscription</Button>
    </form>
</div>
