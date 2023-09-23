<script lang="ts">
    export let adminPage: string;

    let overlay: HTMLDialogElement;
    let username = "";
    let password = "";

    function startLogin() {
        overlay.showModal();
    }

    function clickLogin(e: MouseEvent) {
        if (username.length > 0 && password.length > 0) {
            e.stopPropagation();
            e.preventDefault();
            window.location.href = adminPage;
        }
    }
</script>

<button class="btn {$$restProps.class}" on:click={startLogin}>
    Admin Login
</button>
<dialog id="my_modal_1" class="modal" bind:this={overlay}>
    <div class="modal-box">
        <div class="text-center space-y-4">
            <h3 class="font-bold text-lg">Admin Login</h3>
            <form class="space-y-4">
                <input
                    type="text"
                    placeholder="Username"
                    class="input input-bordered w-full"
                    bind:value={username}
                    required
                />
                <input
                    type="password"
                    placeholder="Password"
                    class="input input-bordered w-full"
                    bind:value={password}
                    required
                />
                <button class="btn btn-primary w-full" on:click={clickLogin}>
                    Login
                </button>
            </form>
        </div>
    </div>
    <form method="dialog" class="modal-backdrop">
        <!-- this button automatically closes the modal -->
        <button />
    </form>
</dialog>
