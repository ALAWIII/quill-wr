use super::*;
use serde_json::Value;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
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

    editor.set_text("hello", None);
    let txt = editor.get_text(None, None);
    let contnt = editor.get_content(None, None);
    let length = editor.get_length();

    gloo::console::log!(txt);
    gloo::console::log!(contnt.to_json().expect("Failed to convert to json"));
    gloo::console::log!(length);
    gloo::console::log!(
        editor
            .set_content(&contnt, None)
            .to_json()
            .expect("Failed to convert to json")
    );
    editor.set_text("eat pizza", None);
    let fore = [
        Inline::Bold,
        Inline::Underline,
        Inline::Code,
        Inline::Strike,
        // Inline::BgColor("red".to_string()),
        Inline::Sup,
        Inline::Custom("background".to_string(), Value::String("red".into())), //works
        Inline::Size(50.0),                                                    //not working
    ];
    editor.insert_embed(
        50,
        Embeds::Video("video_2025-09-13_18-52-54.mp4".to_string()),
        None,
    );
    let formats = inlines_to_map(&fore);
    gloo::console::log!(to_value(&formats).expect("msg"));
    let inserted = editor.insert_text(10, "delcious", &fore, None);
    gloo::console::log!(inserted.to_json().unwrap());
    editor.delete_text(0, 20, None);
    editor.update_content(contnt, None);
    editor.format(Inline::Color("red".into()), Some(Source::Api));
    editor.blur();
    editor.disable();
    editor.enable(true);
    editor.focus(Some(false));
    // let deleted = editor.delete_text(0, 10, None);
    // gloo::console::log!(deleted.to_json().expect("Failed to convert to json"));

    // Add event handlers
    let id = editor.on("text-change", |args| {
        web_sys::console::log_1(&"Text changed!".into());
    });
    editor.off(id);

    /*
    let sem_html = editor.get_semantic_html(Some(0), Some(10));
    gloo::console::log!(sem_html);*/
}
