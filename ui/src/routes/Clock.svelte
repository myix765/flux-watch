<script lang="ts">
    import * as PIXI from 'pixi.js';
    import { onMount } from 'svelte';

    let canvas: HTMLCanvasElement;
    let container: HTMLDivElement;

    onMount(async () => {
        const { default: init, get_glyph } = await import('$lib/wasmc/flux_core.js');
        await init();

        const PADDING = 16;
        const app = new PIXI.Application();

        await app.init({
            canvas,
            backgroundColor: 0x1099bb,
            resizeTo: container
        });

        app.stage.x = PADDING;
        app.stage.y = PADDING;
    });
</script>

<div
    bind:this={container}
    class="absolute top-[25.3%] right-[17%] bottom-[25.3%] left-[12.4%] overflow-hidden rounded-[15.75%] opacity-55"
>
    <canvas bind:this={canvas}></canvas>
</div>
