<script>
    import { enhance } from "$app/forms";
    import { extractError } from "$lib/errors";
    import Button from "$lib/form/Button.svelte";
    import Dropzone from "$lib/form/Dropzone.svelte";
    import FileInput from "$lib/form/FileInput.svelte";
    import Input from "$lib/form/Input.svelte";
    import SaveIcon from "$lib/icons/SaveIcon.svelte";
    import { toast } from "$lib/ui/toast";

    /** @type {import("./$types").PageData} */
    export let data;

    /** @type {import("./$types").ActionData} */
    export let form;
    $: if (form?.error) {
        toast.error("Error", form?.error || "Unknown error");
    }

    /** @type {boolean} */
    let loading = false;

    /** @type {File} */
    let resume = new File([""], "resume.pdf", { type: "application/pdf" });

    /** @type {File} */
    let cover = new File([""], "cover.png", { type: "image/png" });

    /**
     * Download a base64 encoded file
     * @param {Buffer} buffer
     * @param {string} name
     * @param {string} mimeType
     * @returns {Promise<void>}
     */
    async function download(buffer, name, mimeType) {
        try {
            const blob = new Blob([new Uint8Array(buffer)], { type: mimeType });
            const url = URL.createObjectURL(blob);
            const link = document.createElement("a");
            link.href = url;
            link.download = name;
            document.body.appendChild(link);
            link.click();
            document.body.removeChild(link);
        } catch (e) {
            console.error(e);
        }
    }
</script>

<form
    class="max-w-2xl"
    method="post"
    action="?/createProfile"
    enctype="multipart/form-data"
    use:enhance={() => {
        const timeout = setTimeout(() => {
            loading = true;
        }, 100);
        return async ({ result, update }) => {
            if (result.type === "success") {
                toast.success("Success", "Your profile has been updated.");
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
                Profile
            </h2>
            <p class="mt-1 text-sm leading-6 text-gray-200">
                Your public profile is how other people will see you.
            </p>
        </div>

        <div class="mt-10 grid grid-cols-1 gap-x-6 sm:grid-cols-6">
            <input type="hidden" name="id" bind:value={data.profile.id} />
            <div class="sm:col-span-4">
                <Input
                    name="name"
                    label="Name"
                    autocomplete="name"
                    bind:value={data.profile.name}
                    error={extractError(form?.fields, "name")}
                />
            </div>

            <div class="col-span-full">
                <Input
                    name="about"
                    label="About"
                    autocomplete="on"
                    bind:value={data.profile.about}
                    rows={3}
                    error={extractError(form?.fields, "about")}
                    helper="Write a few sentences about yourself."
                />
            </div>

            <div class="col-span-full">
                <input
                    type="hidden"
                    name="resume_id"
                    bind:value={data.profile.resume_id}
                />
                <FileInput
                    name="resume"
                    label="Resume"
                    bind:file={resume}
                    helper="PDF up to 5MB"
                />
                {#if data.profile.resume_id}
                    {#await data.stream.resume}
                        <div class="mt-2">
                            <span class="text-sm text-gray-600">
                                Loading...
                            </span>
                        </div>
                    {:then r}
                        {#if !r.error && r.data}
                            <div class="mt-2">
                                <button
                                    class="text-sm text-blue-600 hover:text-blue-500"
                                    type="button"
                                    on:click={() => {
                                        if (r.data) {
                                            download(
                                                r.data.buffer,
                                                r.data.name,
                                                r.data.mime_type,
                                            );
                                        }
                                    }}
                                >
                                    Download {r.data.name}
                                </button>
                            </div>
                        {/if}
                    {:catch error}
                        <div class="mt-2 text-sm text-red-600">
                            {error.message}
                        </div>
                    {/await}
                {/if}
            </div>

            <div class="col-span-full mt-6">
                <input
                    type="hidden"
                    name="cover_id"
                    bind:value={data.profile.cover_id}
                />
                <input
                    type="hidden"
                    name="cover_url"
                    bind:value={data.profile.cover_url}
                />
                <Dropzone
                    name="cover"
                    label="Cover photo"
                    bind:file={cover}
                    description="SVG, PNG, JPG, GIF up to 5MB"
                    url={data.profile.cover_url
                        ? data.profile.cover_url + "h=400"
                        : ""}
                    accept="image/*"
                />
            </div>
        </div>
        <div class="flex justify-end">
            <Button type="submit" {loading}>
                <svelte:fragment slot="icon">
                    <SaveIcon />
                </svelte:fragment>
                Save
            </Button>
        </div>
    </div>
</form>
