use js_sys::Function;
use stdweb::web::TypedArray;
use stdweb::unstable::TryInto;

#[allow(dead_code)]
pub struct FileStorageZomeClient {
    call_zome: Function,
}

impl FileStorageZomeClient {
    pub fn new(call_zome: Function) -> Self {
        js! { console.log("FileStorageZomeClient initialized") };
        Self { call_zome }
    }

    pub fn store_file(&self, file: stdweb::Value) -> String {
        js! { console.log("TODO: Store file " + JSON.stringify(@{file})) };
        "ab8629cd989324829f9aa98".into()
    }

    pub fn get_file(&self, manifest_address: &str) -> TypedArray<u8> {
        js! { console.log("TODO: Get file " + JSON.stringify(@{manifest_address})) };

        // We'll just return some mock data here. A red dot png.
        let byte_characters = base64::decode(
            "iVBORw0KGgoAAAANSUhEUgAAAAUAAAAFCAYAAACNbyblAAAAHElEQVQI12P4//8/w38GIAXDIBKE0DHxgljNBAAO9TXL0Y4OHwAAAABJRU5ErkJggg=="
        ).unwrap();

        byte_characters[..].into()
    }
}
