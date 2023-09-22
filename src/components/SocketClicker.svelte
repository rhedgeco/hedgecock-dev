<script lang="ts" onload>
    import { onMount } from "svelte";
    import { writable } from "svelte/store";
    import env from "../lib/env";

    let loaded = false;
    let clickedAmount = 0;

    const socketValue = writable<number>(0);
    socketValue.subscribe((value) => {
        if (clickedAmount < value) {
            clickedAmount = value;
        }
    });

    function handleClick() {
        clickedAmount += 1;
        socketClick();
    }

    let socketClick = () => {};
    onMount(async () => {
        clickedAmount = await (await fetch(`${env.apiOrigin()}/click`)).json();
        let socket = new WebSocket(`${env.wsOrigin()}/click/ws`);
        socket.addEventListener("message", (msg) => {
            socketValue.update(() => Number(msg.data));
        });

        socketClick = () => {
            socket.send("CLICKED!");
        };

        loaded = true;
    });
</script>

<button class="btn btn-primary" on:click={handleClick} disabled={!loaded}>
    Clicked
    {clickedAmount}
    {clickedAmount === 1 ? "time" : "times"}
</button>
