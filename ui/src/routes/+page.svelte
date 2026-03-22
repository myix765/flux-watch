<script>
    import AppleWatch from '$lib/assets/apple-watch.png';
    import Clock from './Clock.svelte';
</script>

<div class="flex h-screen gap-x-24 *:flex *:items-center-safe *:justify-center-safe">
    <div class="w-1/4 p-24">
        <h1 class="font-anta text-[5rem] leading-[1.2]">Apple Watch Flux Face Recreation</h1>
    </div>
    <div class="h-full flex-1">
        <img src={AppleWatch} alt="Apple Watch" class="h-2/3" />
        <Clock />
    </div>
</div>
