export class FileStorageZomeClient {

    constructor(callZome) {
        this.callZome = callZome;
    }

    storeFile(file) {
        console.log('TODO: Store file ' + JSON.stringify(file));
        return "ab8629cd989324829f9aa98";
    }

    getFile(manifestAddress) {
        console.log('TODO: Get file ' + JSON.stringify(manifestAddress));

        // We'll just return some mock data here. A red dot png.
        const byteCharacters = atob('iVBORw0KGgoAAAANSUhEUgAAAAUAAAAFCAYAAACNbyblAAAAHElEQVQI12P4//8/w38GIAXDIBKE0DHxgljNBAAO9TXL0Y4OHwAAAABJRU5ErkJggg==');
        const byteNumbers = new Array(byteCharacters.length);
        for (let i = 0; i < byteCharacters.length; i++) {
            byteNumbers[i] = byteCharacters.charCodeAt(i);
        }
        const byteArray = new Uint8Array(byteNumbers);

        return byteArray;
    }

}
