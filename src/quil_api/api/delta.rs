use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = Delta)]
    pub type Delta;

    #[wasm_bindgen(constructor)]
    pub fn new(ops: JsValue) -> Delta;

    // Low-level operations
    #[wasm_bindgen(method)]
    pub fn insert(this: &Delta, text: &JsValue, attributes: &JsValue) -> Delta;

    #[wasm_bindgen(method)]
    pub fn delete(this: &Delta, length: u32) -> Delta;

    #[wasm_bindgen(method)]
    pub fn retain(this: &Delta, length: u32, attributes: &JsValue) -> Delta;

    #[wasm_bindgen(method)]
    pub fn concat(this: &Delta, other: &Delta) -> Delta;

    #[wasm_bindgen(method)]
    pub fn diff(this: &Delta, other: &Delta) -> Delta;

    #[wasm_bindgen(method, js_name = "compose")]
    pub fn compose(this: &Delta, other: &Delta) -> Delta;

    #[wasm_bindgen(method, js_name = "transformPosition")]
    pub fn transform_position(this: &Delta, index: u32, priority: bool) -> u32;
    #[wasm_bindgen(method, js_name = "transform")]
    pub fn transform(this: &Delta, other: &Delta, priority: bool) -> Delta;
}
