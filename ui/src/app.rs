use js_sys::Function;
use serde::Serialize;
use stdweb::{
    unstable::TryInto,
    web::{alert, document, Blob, INode, INonElementParentNode},
};
use wasm_bindgen::prelude::*;

use crate::file_storage_zome_client::FileStorageZomeClient;

#[derive(Serialize)]
struct MockFile {
    manifest_address: String,
    file_name: String,
}

#[wasm_bindgen]
#[allow(dead_code)]
pub struct App {
    call_zome: Function,
    file_client: FileStorageZomeClient,
    mock_file_list: Vec<MockFile>,
}

#[wasm_bindgen]
#[allow(non_snake_case)]
impl App {
    pub fn new(call_zome: Function) -> Self {
        Self {
            call_zome: call_zome.clone(),
            file_client: FileStorageZomeClient::new(call_zome),
            mock_file_list: vec![MockFile {
                manifest_address: "abcd776afbcbcffa73231".into(),
                file_name: "reddot.png".into(),
            }],
        }
    }

    #[allow(clippy::bool_comparison)]
    pub fn addFile(&mut self, name: String, add_file_field_file: String) {
        let file = document().get_element_by_id(&add_file_field_file).unwrap();
        let file = js!(return @{file}.files[0]);

        console!(log, "Add file name: ", name.clone(), " file: ", file.clone());
        if js!(return @{file.clone()} == null) == true {
            alert("Select a file");
            return;
        }

        let manifest_address = self.file_client.store_file(file.clone());
        let file_name = if name.is_empty() {
            js!(return @{file.clone()}.name).into_string().unwrap()
        } else {
            name
        };

        // TODO: Call our app zome function and save the address + filename
        // self.call_zome()

        self.mock_file_list.push(MockFile {
            manifest_address,
            file_name,
        });
    }

    pub fn getFiles(&mut self) -> JsValue {
        // TODO: Call our app zome function and get the files
        // self.call_zome()

        // Return mock data
        JsValue::from_serde(&self.mock_file_list).unwrap()
    }

    pub fn downloadFile(&mut self, manifest_address: String, file_name: String) {
        let data = self.file_client.get_file(&manifest_address);

        let a = document().create_element("a").unwrap();
        document().body().unwrap().append_child(&a);
        js!(@{a.clone()}.style = "display: none");

        let blob: Blob = js!(return new Blob([@{data}], {type: "octet/stream"}))
            .try_into()
            .unwrap();
        let url = js!(return window.URL.createObjectURL(@{blob}));
        js!(@{a.clone()}.href = @{url.clone()});
        js!(@{a.clone()}.download = @{file_name});
        js!(@{a}.click());
        js!(window.URL.revokeObjectURL(@{url}));
    }

    pub fn generateFileListTableBody(&mut self) {
        let file_list_table_body_ele = document()
            .get_element_by_id("file_list_table_body")
            .unwrap();
        let mut rows: Vec<String> = Vec::new();
        self.mock_file_list.iter().for_each(|file| {
            rows.push(format!(
                r#"<tr><td>{}</td><td>{}</td><td><button class="btn btn-primary" onclick="app.downloadFile('{}', '{}')">Download</button></td></tr>"#,
                &file.file_name, &file.manifest_address, &file.manifest_address, &file.file_name
            ))
        });

        js!(@{file_list_table_body_ele}.innerHTML = @{rows.join("\n")});
    }
}
