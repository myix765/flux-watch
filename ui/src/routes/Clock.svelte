<script lang="ts">
    import * as PIXI from 'pixi.js';
    import { onMount } from 'svelte';

    let canvas: HTMLCanvasElement;
    let container: HTMLDivElement;

    onMount(async () => {
        const PADDING = 16;
        const app = new PIXI.Application();

        await app.init({
            canvas,
            backgroundColor: 0xed857e,
            resizeTo: container
        });

        app.stage.x = PADDING;
        app.stage.y = PADDING;
    });
</script>

<div
    bind:this={container}
    class="absolute mr-[0.94rem] h-[33%] w-[14.8%] overflow-hidden rounded-[2.5rem]"
>
    <canvas bind:this={canvas}></canvas>
</div>
