<script>
    import AppleWatch from '$lib/assets/apple-watch.png';
    import Clock from './Clock.svelte';
</script>

<div class="flex h-screen flex-col items-center gap-x-12 md:flex-row">
    <div
        class="flex w-full basis-1/5 shrink-0 items-end justify-center px-8 py-6 md:w-1/3 md:items-center md:px-12 md:py-8"
    >
        <h1 class="text-center font-anta text-2xl leading-[1.2] md:text-left md:text-[4vw]">
            Apple Watch Flux Face Recreation
        </h1>
    </div>
    <div class="flex min-h-0 h-full w-full flex-1 items-center justify-center">
        <div class="relative aspect-659/1143 h-3/4 max-h-full">
            <img src={AppleWatch} alt="Apple Watch" class="h-full w-full object-contain" />
            <Clock />
        </div>
    </div>
</div>
