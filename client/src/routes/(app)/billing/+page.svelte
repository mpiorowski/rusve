<script lang="ts">
    import Button from "$lib/components/form/Button.svelte";
    import DollarIcon from "$lib/icons/DollarIcon.svelte";
    import { toast } from "$lib/components/toast/toast.js";
    import type { ActionData, PageData } from "./$types.js";

    export let data: PageData;
    export let form: ActionData;

    $: if (form?.error) {
        toast.error(form.error);
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
        You are currently on the {isPro ? '"Pro"' : '"Noob"'} plan
    </h3>

    {#if !isPro}
        <form action="?/checkout" method="POST">
            <input type="hidden" name="paymentId" value={data.paymentId} />
            <Button type="submit">
                <svelte:fragment slot="icon">
                    <DollarIcon />
                </svelte:fragment>
                Upgrade to "Pro" plan
            </Button>
            <p class="mt-2">
                test mode - no real payment will be made <br />
                use (4242 4242 ....) as credit card number
            </p>
        </form>
    {/if}
</div>
