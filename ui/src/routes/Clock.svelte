<script lang="ts">
    import * as PIXI from 'pixi.js';
    import { onMount } from 'svelte';

    let canvas: HTMLCanvasElement;
    let container: HTMLDivElement;

    onMount(async () => {
        const { default: init, get_glyph_outline } = await import('$lib/wasmc/flux_core.js');
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

        const commands = get_glyph_outline('4', 200.0);

        const g = new PIXI.Graphics();

        for (const cmd of commands) {
            switch (cmd.kind) {
                case 'MoveTo':
                    g.moveTo(cmd.x, cmd.y);
                    break;
                case 'LineTo':
                    g.lineTo(cmd.x, cmd.y);
                    break;
                case 'QuadTo':
                    g.quadraticCurveTo(cmd.cx1, cmd.cy1, cmd.x, cmd.y);
                    break;
                case 'CubicTo':
                    g.bezierCurveTo(cmd.cx1, cmd.cy1, cmd.cx2, cmd.cy2, cmd.x, cmd.y);
                    break;
                case 'Close':
                    g.closePath();
                    break;
            }
        }
        g.stroke({ width: 2, color: 0xff00ff });
        g.y = 200.0;

        app.stage.addChild(g);
    });
</script>

<div
    bind:this={container}
    class="absolute top-[25.3%] right-[17%] bottom-[25.3%] left-[12.4%] overflow-hidden rounded-[15.75%]"
>
    <canvas bind:this={canvas}></canvas>
</div>
