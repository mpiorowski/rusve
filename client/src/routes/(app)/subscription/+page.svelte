<script>
    import Button from "$lib/form/Button.svelte";

    /** @type {import("./$types").PageData} */
    export let data;
</script>

<h1 class="mb-6">Subscription status: {data.subscriptionActive}</h1>

<form action="?/createStripeCheckout" method="post">
    <Button>Checkout</Button>
</form>
