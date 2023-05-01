<script lang="ts">
    import { toast } from "$lib/toast/toast.js";

    export let data;
    export let form;

    $: if (form?.error) {
        toast({
            message: form.error,
            type: "error",
        });
    }
</script>

<h1>Billing</h1>

<div class="product">
    <div class="description">
        <h3>Starter plan</h3>
        <h5>$20.00 / month</h5>
    </div>
</div>
<form method="POST">
    <input type="hidden" name="paymentId" value={data.paymentId ?? ""} />
    <input type="hidden" name="lookup_key" value="standard_2" />
    <button id="checkout-and-portal-button" type="submit">Checkout</button>
</form>
