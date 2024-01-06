<script>
    import { enhance } from "$app/forms";
    import { extractError } from "$lib/errors";
    import Button from "$lib/form/Button.svelte";
    import Input from "$lib/form/Input.svelte";
    import SendIcon from "$lib/icons/SendIcon.svelte";
    import Pagination from "$lib/ui/Pagination.svelte";
    import { toast } from "$lib/ui/toast";

    /** @type {import("./$types").PageData} */
    export let data;

    /** @type {import("./$types").ActionData} */
    export let form;
    $: if (form?.error || data.error) {
        toast.error("Error", form?.error || data.error || "Unknown error");
    }

    let loading = false;

    const newEmail = {
        emailTo: "",
        emailFrom: "email@rusve.app",
        emailFromName: "",
        emailSubject: "",
        emailBody: "",
    };
</script>

<form
    class="max-w-2xl"
    method="post"
    action="?/sendEmail"
    enctype="multipart/form-data"
    use:enhance={() => {
        const timeout = setTimeout(() => {
            loading = true;
        }, 100);
        return async ({ result, update }) => {
            if (result.type === "success") {
                toast.success("Success", "Your message has been sent.");
            }
            clearTimeout(timeout);
            loading = false;
            await update({
                reset: false,
            });
        };
    }}
>
    <div class="space-y-12">
        <div>
            <h2
                class="flex items-center gap-2 text-base font-semibold leading-7 text-gray-50"
            >
                New Email
            </h2>
            <p class="mt-1 text-sm leading-6 text-gray-200">Send an email.</p>
        </div>
        <div class="grid grid-cols-2 gap-x-6">
            <div class="col-span-2">
                <Input
                    name="emailTo"
                    label="To"
                    autocomplete="email"
                    bind:value={newEmail.emailTo}
                    error={extractError(form?.fields, "email_to")}
                />
            </div>
            <Input
                name="emailFrom"
                label="From"
                bind:value={newEmail.emailFrom}
                error={extractError(form?.fields, "email_from")}
            />
            <Input
                name="emailFromName"
                label="From name"
                bind:value={newEmail.emailFromName}
                error={extractError(form?.fields, "email_from_name")}
            />
            <div class="col-span-2">
                <Input
                    name="emailSubject"
                    label="Subject"
                    autocomplete="email"
                    bind:value={newEmail.emailSubject}
                    error={extractError(form?.fields, "email_subject")}
                />
            </div>
            <div class="col-span-2">
                <Input
                    name="emailBody"
                    label="Body"
                    autocomplete="email"
                    rows={5}
                    bind:value={newEmail.emailBody}
                    error={extractError(form?.fields, "email_body")}
                />
            </div>
            <div class="col-span-full flex justify-end">
                <Button {loading}>
                    <SendIcon />
                    Send
                </Button>
            </div>
        </div>
    </div>
</form>

<div class="mt-10 sm:flex sm:items-center">
    <div class="sm:flex-auto">
        <h1 class="text-base font-semibold leading-6 text-gray-50">Emails</h1>
        <p class="mt-2 text-sm leading-6 text-gray-200">
            List of emails You have sent.
        </p>
    </div>
</div>
<div class="mt-8 flow-root max-w-7xl">
    <div class="overflow-x-auto overflow-y-hidden">
        <div class="inline-block min-w-full align-middle">
            <table class="min-w-full divide-y divide-gray-600">
                <thead>
                    <tr>
                        <th
                            scope="col"
                            class="py-3 pl-4 pr-3 text-left text-xs uppercase tracking-wide text-gray-500 sm:pl-0"
                        >
                            To
                        </th>
                        <th
                            scope="col"
                            class="px-3 py-3 text-left text-xs uppercase tracking-wide text-gray-500"
                        >
                            From
                        </th>
                        <th
                            scope="col"
                            class="px-3 py-3 text-left text-xs uppercase tracking-wide text-gray-500"
                        >
                            From name
                        </th>
                        <th
                            scope="col"
                            class="px-3 py-3 text-left text-xs uppercase tracking-wide text-gray-500"
                        >
                            Subject
                        </th>
                        <th
                            scope="col"
                            class="px-3 py-3 text-left text-xs uppercase tracking-wide text-gray-500"
                        >
                            Body
                        </th>
                        <th
                            scope="col"
                            class="px-3 py-3 text-left text-xs uppercase tracking-wide text-gray-500"
                        >
                            Created
                        </th>
                        <th
                            scope="col"
                            class="px-3 py-3 text-left text-xs uppercase tracking-wide text-gray-500"
                        >
                            Updated
                        </th>
                    </tr>
                </thead>
                <tbody class="divide-y divide-gray-600 bg-gray-900">
                    {#each data.emails as email}
                        <tr>
                            <td
                                class="whitespace-nowrap py-4 pl-4 pr-3 text-sm font-medium text-gray-50 sm:pl-0"
                            >
                                {email.emailTo}
                            </td>
                            <td
                                class="whitespace-nowrap px-3 py-4 text-sm text-gray-200"
                            >
                                {email.emailFrom}
                            </td>
                            <td
                                class="whitespace-nowrap px-3 py-4 text-sm text-gray-200"
                            >
                                {email.emailFromName}
                            </td>
                            <td
                                class="whitespace-nowrap px-3 py-4 text-sm text-gray-200"
                            >
                                {email.emailSubject}
                            </td>
                            <td
                                class="whitespace-nowrap px-3 py-4 text-sm text-gray-200"
                            >
                                {email.emailBody}
                            </td>
                            <td
                                class="whitespace-nowrap px-3 py-4 text-sm text-gray-200"
                            >
                                {email.created}
                            </td>
                            <td
                                class="whitespace-nowrap px-3 py-4 text-sm text-gray-200"
                            >
                                {email.updated}
                            </td>
                        </tr>
                    {/each}

                    <!-- More people... -->
                </tbody>
            </table>
        </div>
    </div>

    <!-- Pagination -->
    <Pagination total={data.total} pageSize={data.pageSize} />
</div>
