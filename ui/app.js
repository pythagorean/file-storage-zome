import { FileStorageZomeClient } from './js-lib/file-storage-zome-client.js';

class App {

    callZome;
    fileClient;
    mockFileList = [{manifestAddress: "abcd776afbcbcffa73231", fileName: "reddot.png"}];

    constructor() {
        holochainclient.connect("ws://localhost:8888").then(({callZome, close}) => {
            this.callZome = callZome;
            this.fileClient = new FileStorageZomeClient(this.callZome);
        });

        setInterval(() => this.generateFileListTableBody(), 1000);
    }

    addFile(fileInfo) {
        console.log("Add file name: " + fileInfo.name + " file: " + fileInfo.file);
        let address = this.fileClient.storeFile(fileInfo.file);
        
        // TODO: Call our app zome function and save the address + filename
        // this.callZome()

        this.mockFileList.push({manifestAddress: address, fileName: fileInfo.name});
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

        let blob = new Blob([data], {type: "octet/stream"});
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
            rows.push('<tr><td>' + file.fileName + '</td><td>' + file.manifestAddress + '</td><td><button onclick="app.downloadFile(\'' + file.manifestAddress + '\',\'' + file.fileName + '\')">Download</button></td></tr>')
        });

        fileListTableBodyEle.innerHTML = rows.join('\n');
    }

}

window.app = new App();
