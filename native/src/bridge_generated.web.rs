use super::*;
// Section: wire functions

#[wasm_bindgen]
pub fn wire_platform(port_: MessagePort) {
    wire_platform_impl(port_)
}

#[wasm_bindgen]
pub fn wire_rust_release_mode(port_: MessagePort) {
    wire_rust_release_mode_impl(port_)
}

#[wasm_bindgen]
pub fn wire_tick(port_: MessagePort) {
    wire_tick_impl(port_)
}

#[wasm_bindgen]
pub fn wire_test_api_key(port_: MessagePort, api: String) {
    wire_test_api_key_impl(port_, api)
}

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<String> for String {
    fn wire2api(self) -> String {
        self
    }
}

impl Wire2Api<Vec<u8>> for Box<[u8]> {
    fn wire2api(self) -> Vec<u8> {
        self.into_vec()
    }
}
// Section: impl Wire2Api for JsValue

impl Wire2Api<String> for JsValue {
    fn wire2api(self) -> String {
        self.as_string().expect("non-UTF-8 string, or not a string")
    }
}
impl Wire2Api<u8> for JsValue {
    fn wire2api(self) -> u8 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<Vec<u8>> for JsValue {
    fn wire2api(self) -> Vec<u8> {
        self.unchecked_into::<js_sys::Uint8Array>().to_vec().into()
    }
}
