import {
    App,
    default as init
} from './pkg/file_storage_zome_client.js';

async function compilePkg() {
    const url = await fetch('./pkg/file_storage_zome_client_bg.wasm');
    const body = await url.arrayBuffer();
    const module = await WebAssembly.compile(body);
    await init(module);
}

compilePkg().catch(err => Promise.resolve(err)).then(() => {
    holochainclient.connect("ws://localhost:8888").then(({
        callZome,
        close
    }) => {
        window.app = App.new(callZome);
    })
});

setInterval(() => window.app.generateFileListTableBody(), 1000);
