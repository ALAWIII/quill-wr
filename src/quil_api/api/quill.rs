use std::collections::HashMap;

use gloo::utils::format::JsValueSerdeExt;
use serde_json::json;
use wasm_bindgen::prelude::*;
use web_sys::{Node, js_sys::Function};

use crate::quil_api::api::Delta;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = Quill)]
    pub type Quill;

    #[wasm_bindgen(constructor)]
    pub fn new(selector: &str, options: &JsValue) -> Quill;

    // Content methods

    #[wasm_bindgen(method, js_name = "deleteText")]
    /// default 'api'
    pub fn delete_text(this: &Quill, index: u32, length: u32, source: &str) -> Delta;

    #[wasm_bindgen(method, js_name = "getContents")]
    /// index=0 , length = full as default values
    pub fn get_contents(this: &Quill, index: u32, length: u32) -> Delta;

    #[wasm_bindgen(method, js_name = "getLength")]
    /// if quill is empty, but it contains blank line -> returns 1
    pub fn get_length(this: &Quill) -> u32;

    #[wasm_bindgen(method, js_name = "getText")]
    /// index=0 , length = full as default values
    pub fn get_text(this: &Quill, index: u32, length: u32) -> String;

    #[wasm_bindgen(method, js_name = "getSemanticHTML")]
    /// index=0 , length = full as default values
    pub fn get_semantic_html(this: &Quill, index: &JsValue, length: u32) -> String;

    #[wasm_bindgen(method, js_name = "insertEmbed")]
    /// default soruce="api"
    pub fn insert_embed(
        this: &Quill,
        index: u32,
        type_: &str,
        value: &JsValue,
        source: &str,
    ) -> Delta;

    #[wasm_bindgen(method, js_name = "insertText")]
    /// default source ="api"
    pub fn insert_text(
        this: &Quill,
        index: u32,
        text: &str,
        formats: &JsValue,
        source: &str,
    ) -> Delta;

    #[wasm_bindgen(method, js_name = "setContents")]
    /// default source ="api"
    pub fn set_contents(this: &Quill, delta: &Delta, source: &str) -> Delta;

    #[wasm_bindgen(method, js_name = "setText")]
    /// default source ="api"
    pub fn set_text(this: &Quill, text: &str, source: &str) -> Delta;

    #[wasm_bindgen(method, js_name = "updateContents")]
    /// default source ="api"
    pub fn update_contents(this: &Quill, delta: &JsValue, source: &str) -> Delta;

    #[wasm_bindgen(method, js_name = "format")]
    /// default source ="api"
    pub fn format(this: &Quill, name: &str, value: &JsValue, source: &str) -> Delta;
    #[wasm_bindgen(method, js_name = "formatLine")]
    /// source defaults to 'api'
    pub fn format_line(
        this: &Quill,
        index: u32,
        length: u32,
        formats: &JsValue,
        source: &str,
    ) -> Delta;
    #[wasm_bindgen(method, js_name = "formatText")]
    /// default source ="api"
    pub fn format_text(
        this: &Quill,
        index: u32,
        length: u32,
        formats: &JsValue,
        source: &str,
    ) -> Delta;
    #[wasm_bindgen(method, js_name = "getFormat")]
    /// length default = 0
    pub fn get_format(this: &Quill, index: u32, length: u32) -> JsValue;
    #[wasm_bindgen(method, js_name = "removeFormat")]
    /// default source ="api"
    pub fn remove_format(this: &Quill, index: u32, length: u32, source: &str) -> Delta;
    #[wasm_bindgen(method, js_name = "getBounds")]
    /// returns Bounds object , length=0
    pub fn get_bounds(this: &Quill, index: u32, length: u32) -> JsValue;

    #[wasm_bindgen(method, js_name = "getSelection")]
    ///return either (index,length) or null , focus =false
    pub fn get_selection(this: &Quill, focus: bool) -> JsValue;
    #[wasm_bindgen(method, js_name = "setSelection")]
    /// default length=0 ,source='api'
    pub fn set_selection(this: &Quill, index: u32, length: u32, source: &str);
    #[wasm_bindgen(method, js_name = "scrollSelectionIntoView")]
    pub fn scroll_selection_view(this: &Quill);
    #[wasm_bindgen(method, js_name = "scrollRectIntoView")]
    pub fn scroll_rect_into_view(this: &Quill, top: f64, right: f64, bottom: f64, left: f64);
    #[wasm_bindgen(method, js_name = "blur")]
    pub fn blur(this: &Quill);
    #[wasm_bindgen(method, js_name = "disable")]
    pub fn disable(this: &Quill);
    #[wasm_bindgen(method, js_name = "enable")]
    /// default to true
    pub fn enable(this: &Quill, enabled: bool);
    #[wasm_bindgen(method, js_name = "focus")]
    /// default to false
    pub fn focus(this: &Quill, prevent_scroll: bool);
    #[wasm_bindgen(method, js_name = "hasFocus")]
    pub fn has_focus(this: &Quill) -> bool;
    #[wasm_bindgen(method, js_name = "update")]
    /// default to 'user'
    pub fn update(this: &Quill, source: &str);

    //-------------------- Event‑methods---------------------

    #[wasm_bindgen(method, js_name = "on")]
    pub fn on_event(this: &Quill, name: &str, handler: &Function) -> Quill;

    #[wasm_bindgen(method, js_name = "once")]
    pub fn once_event(this: &Quill, name: &str, handler: &Function) -> Quill;

    #[wasm_bindgen(method, js_name = "off")]
    pub fn off_event(this: &Quill, name: &str, handler: &Function) -> Quill;
    //-------------------------------------------------
    #[wasm_bindgen(static_method_of = Quill, js_name="debug")]
    /// passing None Disables all messages , otherwise the default must be log
    pub fn debug(level: &str);
    #[wasm_bindgen(static_method_of = Quill , js_name="import")]
    pub fn import(path: &str) -> JsValue;
    #[wasm_bindgen(static_method_of = Quill, js_name="addContainer")]
    pub fn add_container(class_or_node: &JsValue, ref_node: Option<Node>) -> JsValue;
    #[wasm_bindgen(static_method_of = Quill, js_name="getModule")]
    pub fn get_module(name: &str) -> JsValue;
    #[wasm_bindgen(static_method_of = Quill, js_name = "register")]
    /// default boolean = false
    pub fn register(defs: &JsValue, suppress_warning: bool);
}
