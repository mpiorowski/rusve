<script lang="ts">
    import Button from "$lib/form/Button.svelte";
    import DollarIcon from "$lib/icons/DollarIcon.svelte";
    import { toast } from "$lib/toast/toast.js";

    export let data;
    export let form;

    $: if (form?.error) {
        toast({
            message: form.error,
            type: "error",
        });
    }

    const isPro = data.isSubscribed;
</script>

<div class="flex flex-col border border-primary-600 rounded p-4">
    <h3>Subscription plan</h3>
    <p class="mb-4">You can change your subscription plan at any time.</p>

    {#if data.paymentId}
        <form action="?/portal" method="post">
            <Button variant="secondary">Manage billing</Button>
        </form>
    {/if}

    <h3 class="my-6">
        You are currently on the {isPro ? '"I use Rust"' : '"Noob"'} plan
    </h3>

    {#if !isPro}
        <form action="?/checkout" method="POST">
            <input
                type="hidden"
                name="paymentId"
                value={data.paymentId ?? ""}
            />
            <Button type="submit">
                <svelte:fragment slot="icon">
                    <DollarIcon />
                </svelte:fragment>
                Upgrade to "I use Rust" plan
            </Button>
            <p class="mt-2">(test mode - no real payment will be made)</p>
        </form>
    {/if}
</div>
