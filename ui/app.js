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

class MyApp {
    app;
    constructor() {
        compilePkg().catch(err => Promise.resolve(err)).then(() => {
            holochainclient.connect("ws://localhost:8888").then(({
                callZome,
                close
            }) => {
                this.app = App.new(callZome);
            })
        });

        setInterval(() => this.generateFileListTableBody(), 1000);
    }

    addFile() {
        this.app.addFile();
    }

    getFiles() {
        return this.app.getFiles();
    }

    downloadFile(manifestAddress, fileName) {
        return this.app.downloadFile(manifestAddress, fileName);
    }

    generateFileListTableBody() {
        this.app.generateFileListTableBody();
    }
}

window.app = new MyApp();
