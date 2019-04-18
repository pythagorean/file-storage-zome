# file-storage-zome
A zome that can be mixed in to any DNA to provide basic file storage capabilities. 

This aims to be a community maintained project and will receive support but minimal contributions from the dev team 

## Requirements

There are several considerations for an efficient implementation of file/blob storage:
- There is a limit of 4GB of WASM memory that can be used at any time
- Files must be chunked to provide uniform data distribution in the DHT
- There must be a way to validate that the file you receive is as expected

## Specification

Files should be chunked outside the zome itself. A javascript reference implementation should be included in this repo which performs the chunking and restructuring operations

A manifest should describe a single file and how to retrieve all of the chunks. The hash of the manifest therefore becomes the address for retrieving the file.
The manifest must contain:

- An ordered list of the hashes/addresses of the chunks
- Any additional metadata (e.g. description, file type etc)

validation logic must ensure that a manifest can only be stored in the DHT once all of the chunks are also stored

---

The zome should expose the following interface:

- `store_chunk(data: RawString) -> ZomeApiResult<Address>`
- `store_manifest(manifest: Manifest) -> ZomeApiResult<Address>`
- `get_chunk(address: Address) -> ZomeApiResult<RawString>`
- `get_manifest(address: Address) -> ZomeApiResult<Manifest>`

The javascript interface should expose higher level functions to the end user such as :

- `storeFile(bytes: UInt8Array): string | Error` - returns the address of the manifest as a string
- `getFile(address: string): UInt8Array | Error` - given the manifest address returns the restructured chunks


## Getting Started

Clone the repo (Or better still, fork then clone, so you can work on it too)

To run the holochain portion
```
cd dna-src
hc run --package
```

To run the frontend

Open a new separate terminal (So the backend stays running in the other one)

Install the http-server utility `npm install -g http-server` (or use another lightweight http-server)

Then

```
cd ui
http-server
```

Open your web browser and upload and download some files

## Current Status
File storage zome - There is currently scaffolding. It needs coding, validation and testing
Client library - There is a es6 module that returns mocked data.
Sample UI - This is running and using the lib. Application holochain data (where the list of files is storage) is currently mocked in the frontend.

## Contributing

We follow the "fork-and-pull" Git workflow.

    Fork the repo on GitHub
    Clone the project to your own machine
    Commit changes to your own branch
    Push your work back up to your fork
    Submit a Pull request so that we can review your changes

NOTE: Be sure to merge the latest from "upstream" before making a pull request
