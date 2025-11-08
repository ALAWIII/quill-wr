use super::*;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
pub fn test_quill() {
    let toolbar = ToolbarConfig::Selector("#toolbar-container".into());
    let modules = ModulesConfigBuilder::new().toolbar(toolbar).build();

    let options = QuillOptions::builder()
        .theme("snow")
        .placeholder("Type here...")
        .modules(modules)
        .build();

    let editor = QuillEditor::new_with_options("#editor", options);
    web_sys::console::log_1(&"before set_text".into());
    editor.set_text("shawarma", None);
    let length = editor.get_length();
    editor.blur();
    editor.disable();
    editor.enable(true);
    editor.focus(Some(false));

    web_sys::console::log_1(&length.into());

    // Add event handlers
    editor.on("text-change", |args| {
        web_sys::console::log_1(&"Text changed!".into());
    });
}
