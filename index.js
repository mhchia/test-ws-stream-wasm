import init, {greet} from "./pkg/ws_wasm";


window.addEventListener('load', async () => {
    await init();
    await greet();
});
