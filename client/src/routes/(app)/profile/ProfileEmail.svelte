<script lang="ts">
    import { enhance } from "$app/forms";
    import { toast } from "$lib/toast/toast";
    import Input from "$lib/form/Input.svelte";
    import Button from "$lib/form/Button.svelte";
    import SaveIcon from "$lib/icons/SaveIcon.svelte";

    let loading = false;
    let subject = "Email from your profile";
    let message = "Rusve rocks!";
</script>

<form
    action="?/sendEmail"
    method="post"
    use:enhance={() => {
        loading = true;
        return async ({ result, update }) => {
            await update({ reset: false });
            if (result.type === "success") {
                toast({
                    message: "Email sent",
                    type: "success",
                });
            }
            loading = false;
        };
    }}
    class="p-4"
>
    <h3 class="mb-4">Send email to Yourself</h3>
    <Input name="subject" bind:value={subject} label="Subject" />
    <Input name="message" textarea bind:value={message} label="Message" />
    <div class="w-28">
        <Button type="submit" {loading}>
            <span slot="icon">
                <SaveIcon />
            </span>
            Send
        </Button>
    </div>
</form>
