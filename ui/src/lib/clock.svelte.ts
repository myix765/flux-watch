import { browser } from "$app/environment";

export const clock = () => {
    let now = $state(new Date());

    if (browser) {
        const tick = () => {
            now = new Date();
            requestAnimationFrame(tick);
        };
        requestAnimationFrame(tick);
    }

    const options = { hour12: true };

    return {
        get hours() { return now.getHours() % 12 || 12; },
        get minutes() { return now.getMinutes(); },
        get seconds() { return Math.floor(now.getSeconds() + now.getMilliseconds() / 1000); }
    };
};