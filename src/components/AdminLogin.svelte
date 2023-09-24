<script lang="ts">
    import { onMount } from "svelte";
    export let adminPage: string;

    let loggedIn = false;
    let loginError = false;
    let overlay: HTMLDialogElement;
    let loginForm: HTMLFormElement;
    let username: HTMLInputElement;
    let password: HTMLInputElement;

    async function gotoPanel() {
        if (loggedIn) {
            window.location.href = adminPage;
        } else {
            overlay.showModal();
        }
    }

    async function logout() {
        document.cookie =
            "session_token=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;";
        window.location.href = "/";
    }

    async function login(e: MouseEvent) {
        if (!loginForm.checkValidity()) {
            return;
        }

        const form = new FormData();
        form.append("username", username.value);
        form.append("password", password.value);
        const response = await fetch("/api/admin/login", {
            method: "POST",
            body: form,
        });

        if (response.ok) {
            window.location.href = adminPage;
        } else {
            username.value = "";
            password.value = "";
            loginError = true;
            username.focus();
            e.stopPropagation();
            e.preventDefault();
        }
    }

    onMount(async () => {
        loggedIn = Boolean(await (await fetch("/api/admin/login")).json());
    });
</script>

<div class="join {$$restProps.class}">
    <button class="btn join-item" on:click={gotoPanel}> Admin Panel </button>
    {#if loggedIn}
        <button class="btn join-item" on:click={logout}> Log Out </button>
    {/if}
</div>

<dialog id="my_modal_1" class="modal" bind:this={overlay}>
    <div class="modal-box w-80 overflow-visible">
        <div class="text-center space-y-4 overflow-visible">
            <h3 class="font-bold text-lg">Admin Login</h3>
            <form class="space-y-4 overflow-visible" bind:this={loginForm}>
                <input
                    type="text"
                    placeholder="Username"
                    class="input input-bordered w-full"
                    on:change={() => (loginError = false)}
                    bind:this={username}
                    required
                />
                <input
                    type="password"
                    placeholder="Password"
                    class="input input-bordered w-full"
                    on:change={() => (loginError = false)}
                    bind:this={password}
                    required
                />
                <button class="btn btn-primary w-full" on:click={login}>
                    Login
                </button>
                {#if loginError}
                    <div class="alert alert-error">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="stroke-current shrink-0 h-6 w-6"
                            fill="none"
                            viewBox="0 0 24 24"
                            ><path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
                            /></svg
                        >
                        <span>Invalid Credentials</span>
                    </div>
                {/if}
            </form>
        </div>
    </div>
    <form method="dialog" class="modal-backdrop">
        <button />
    </form>
</dialog>
