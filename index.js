import init from "./pkg/randaudio.js";

async function main() {
    await init();
    console.log("Random audio stream is playing!");
}

onclick = () => {
    main().catch(console.error);
}