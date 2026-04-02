<script lang="ts">
    import * as PIXI from 'pixi.js';
    import { onMount } from 'svelte';

    let canvas: HTMLCanvasElement;
    let container: HTMLDivElement;

    onMount(async () => {
    const { default: init, get_time_layout } = await import('$lib/wasmc/flux_core.js');
    await init();

    const app = new PIXI.Application();
    await app.init({
        canvas,
        backgroundColor: 0x881155,
        resizeTo: container,
        resolution: window.devicePixelRatio || 2,
        autoDensity: true,
        antialias: false,
    });

    function draw() {
        app.stage.removeChildren();

        const PADDING = 16;
        const targetWidth = app.screen.width - PADDING * 2;
        const targetHeight = app.screen.height - PADDING * 2;

        // padding area
        const g = new PIXI.Graphics()
            .rect(PADDING, PADDING, targetWidth, targetHeight)
            .fill(0xff0000)
    
        app.stage.addChild(g);

        const layout = get_time_layout(10, 42, targetWidth, targetHeight, 3);

        for (const glyph of layout) {
            const g = new PIXI.Graphics();
            // + 1 for half of stroke width
            g.x = glyph.x_offset + PADDING + 1;
            g.y = glyph.y_offset + PADDING + 1;

            for (const cmd of glyph.commands) {
                switch (cmd.kind) {
                    case 'MoveTo': g.moveTo(cmd.x, cmd.y); break;
                    case 'LineTo': g.lineTo(cmd.x, cmd.y); break;
                    case 'QuadTo': g.quadraticCurveTo(cmd.cx1, cmd.cy1, cmd.x, cmd.y); break;
                    case 'CubicTo': g.bezierCurveTo(cmd.cx1, cmd.cy1, cmd.cx2, cmd.cy2, cmd.x, cmd.y); break;
                    case 'Close': g.closePath(); break;
                }
            }

            g.stroke({ width: 2, color: 0xff8888 });
            app.stage.addChild(g);
        }
    }

    draw();

    const observer = new ResizeObserver(() => {
        draw();
    });
    observer.observe(container);
});
</script>

<div
    bind:this={container}
    class="absolute top-[25.3%] right-[17%] bottom-[25.3%] left-[12.4%] overflow-hidden rounded-[15.75%]"
>
    <canvas bind:this={canvas}></canvas>
</div>
