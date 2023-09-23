<script lang="ts">
    export let prefix = "$ ";
    export let lines = ["console text > "];
    export let prefixClass = "text-secondary";
    export let linesClass = "text-base-content";
</script>

<div class="bg-base-200 rounded-box p-4">
    <div class="font-mono">
        {#each lines as line}
            <p class={linesClass}>
                <span class={prefixClass}>{prefix}</span>
                {line}
            </p>
        {/each}
    </div>
</div>
