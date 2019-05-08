import {
    add,
    default as init
} from './pkg/file_storage_zome_client.js';

async function compilePkg() {
    const url = await fetch('./pkg/file_storage_zome_client_bg.wasm');
    const body = await url.arrayBuffer();
    const module = await WebAssembly.compile(body);
    await init(module);

    const result = add(1, 2);
    if (result == 3) {
        return "pkg compiled";
    } else {
        throw new Error("wasm addition doesn't work!");
    }
}

async function ensureResolved(prop) {
    const value = await prop;
    if (value instanceof Error) {
        throw value;
    } else {
        return value;
    }
}

import {
    FileStorageZomeClient
} from './js-lib/file-storage-zome-client.js';

class App {
    pkg;
    callZome;
    fileClient;
    mockFileList = [{
        manifestAddress: "abcd776afbcbcffa73231",
        fileName: "reddot.png"
    }];

    constructor() {
        this.pkg = compilePkg().catch(err => Promise.resolve(err));

        holochainclient.connect("ws://localhost:8888").then(({
            callZome,
            close
        }) => {
            this.callZome = callZome;
            this.fileClient = new FileStorageZomeClient(this.callZome);
        });

        setInterval(() => this.generateFileListTableBody(), 1000);
    }

    async getPkg() {
        return ensureResolved(this.pkg);
    }

    addFile(fileInfo) {
        console.log("Add file name: " + fileInfo.name + " file: " + fileInfo.file);

        if (fileInfo.file == null) {
            alert("Select a file");
            return;
        }

        if (fileInfo.name == null || fileInfo.name.trim().length === 0) {
            fileInfo.name = fileInfo.file.name;
        }

        let address = this.fileClient.storeFile(fileInfo.file);

        // TODO: Call our app zome function and save the address + filename
        // this.callZome()

        this.mockFileList.push({
            manifestAddress: address,
            fileName: fileInfo.name
        });
    }

    getFiles() {
        // TODO: Call our app zome function and get the files
        // this.callZome()

        // Return mock data
        return this.mockFileList;
    }

    downloadFile(manifestAddress, fileName) {
        let data = this.fileClient.getFile(manifestAddress);

        let a = document.createElement("a");
        document.body.appendChild(a);
        a.style = "display: none";

        let blob = new Blob([data], {
            type: "octet/stream"
        });
        let url = window.URL.createObjectURL(blob);
        a.href = url;
        a.download = fileName;
        a.click();
        window.URL.revokeObjectURL(url);
    }

    generateFileListTableBody() {
        let fileListTableBodyEle = document.getElementById('file_list_table_body');
        var rows = [];
        this.getFiles().forEach(file => {
            rows.push('<tr><td>' + file.fileName + '</td><td>' + file.manifestAddress + '</td><td><button class="btn btn-primary" onclick="app.downloadFile(\'' + file.manifestAddress + '\',\'' + file.fileName + '\')">Download</button></td></tr>')
        });

        fileListTableBodyEle.innerHTML = rows.join('\n');
    }
}

const app = new App();
app.getPkg()
    .then(pkg => console.log("succeeded", pkg))
    .catch(err => console.error("failed", err));

window.app = app;
