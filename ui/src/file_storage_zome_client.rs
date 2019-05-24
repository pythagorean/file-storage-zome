use js_sys::Function;
use stdweb::web::TypedArray;

#[allow(dead_code)]
pub struct FileStorageZomeClient {
    call_zome: Function,
}

impl FileStorageZomeClient {
    pub fn new(call_zome: Function) -> Self {
        console!(log, "FileStorageZomeClient initialized");
        Self { call_zome }
    }

    pub fn store_file(&self, file: stdweb::Value) -> String {
        console!(log, "TODO: Store file ", json!(file).to_string());
        "ab8629cd989324829f9aa98".into()
    }

    pub fn get_file(&self, manifest_address: &str) -> TypedArray<u8> {
        console!(log, "TODO: Get file ", json!(manifest_address).to_string());

        // We'll just return some mock data here. A red dot png.
        let byte_characters = base64::decode(
            "iVBORw0KGgoAAAANSUhEUgAAAAUAAAAFCAYAAACNbyblAAAAHElEQVQI12P4//8/w38GIAXDIBKE0DHxgljNBAAO9TXL0Y4OHwAAAABJRU5ErkJggg=="
        ).unwrap();

        byte_characters[..].into()
    }
}
