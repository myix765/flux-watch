<script lang="ts">
    import * as PIXI from 'pixi.js';
    import { onMount } from 'svelte';

    let canvas: HTMLCanvasElement;
    let container: HTMLDivElement;

    onMount(async () => {
        const app = new PIXI.Application();

        await app.init({
            canvas,
            backgroundColor: 0x000,
            resizeTo: container,
        });
    })
</script>

<div bind:this={container} class="absolute mr-[0.94rem] w-[14.8%] h-[33%] rounded-[2.5rem] overflow-hidden">
    <canvas bind:this={canvas}></canvas>
</div>