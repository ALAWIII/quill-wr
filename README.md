# 🪶 quill-wr

A **thin Rust→WASM wrapper** around the popular [**Quill**](https://quilljs.com) rich text editor.
This crate allows you to initialize and interact with a Quill editor instance directly from Rust via **wasm-bindgen**.

---

## 🚀 Overview

This project aims to provide Rust bindings for Quill.js — a modern, rich text editor built for the web.
You can control the editor (get/set contents, listen to events, apply formats, etc.) all from Rust, compiled to WebAssembly.

**⚠️ Project status:** Experimental and incomplete

- ✅ Most key methods (e.g. `get_text`, `set_text`, `get_content`, `set_content`, `on`, `off`) **work correctly**
- ⚠️ Some methods like `get_semantic_html()` are **broken or unimplemented**
- 🚧 Lacks error handling, proper testing, and structured abstractions

---

## 📚 Learn Quill

You can find the official Quill documentation here:
👉 [https://quilljs.com/docs/quickstart](https://quilljs.com/docs/quickstart)

---

## 🧪 Example Usage

The following example demonstrates how to initialize a Quill editor, set/get text and content, format text, and attach event listeners.

```rust
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
        Inline::Sup,
        Inline::Custom("background".into(), Value::String("red".into())), // works
        Inline::Size(50.0), // not working yet
    ];

    editor.insert_embed(50, Embeds::Video("video_2025-09-13_18-52-54.mp4".into()), None);

    let formats = inlines_to_map(&fore);
    gloo::console::log!(to_value(&formats).expect("msg"));

    let inserted = editor.insert_text(10, "delicious", &fore, None);
    gloo::console::log!(inserted.to_json().unwrap());

    editor.delete_text(0, 20, None);
    editor.update_content(contnt, None);
    editor.format(Inline::Color("red".into()), Some(Source::Api));
    editor.blur();
    editor.disable();
    editor.enable(true);
    editor.focus(Some(false));

    // Listen for changes
    let id = editor.on("text-change", |args| {
        web_sys::console::log_1(&"Text changed!".into());
    });
    editor.off(id);
}
```

---

## 🧰 Example HTML Setup

To set up a full toolbar, refer to `tests/test.html` in the repository.
Here’s a minimal HTML snippet you can use:

```html
<div id="toolbar-container">
  <span class="ql-formats">
    <select class="ql-font"></select>
    <select class="ql-size"></select>
  </span>
  <span class="ql-formats">
    <button class="ql-bold"></button>
    <button class="ql-italic"></button>
    <button class="ql-underline"></button>
    <button class="ql-strike"></button>
  </span>
  <span class="ql-formats">
    <select class="ql-color"></select>
    <select class="ql-background"></select>
  </span>
  <span class="ql-formats">
    <button class="ql-script" value="sub"></button>
    <button class="ql-script" value="super"></button>
  </span>
  <span class="ql-formats">
    <button class="ql-header" value="1"></button>
    <button class="ql-header" value="2"></button>
    <button class="ql-blockquote"></button>
    <button class="ql-code-block"></button>
  </span>
  <span class="ql-formats">
    <button class="ql-list" value="ordered"></button>
    <button class="ql-list" value="bullet"></button>
    <button class="ql-indent" value="-1"></button>
    <button class="ql-indent" value="+1"></button>
  </span>
  <span class="ql-formats">
    <button class="ql-direction" value="rtl"></button>
    <select class="ql-align"></select>
  </span>
  <span class="ql-formats">
    <button class="ql-link"></button>
    <button class="ql-image"></button>
    <button class="ql-video"></button>
  </span>
  <span class="ql-formats">
    <button class="ql-clean"></button>
  </span>
</div>
```

---

## 🧩 TODO / Roadmap

- [ ] Implement missing methods (e.g. `get_semantic_html` ,`align` ,`size`)
- [ ] Add structured error handling
- [ ] Improve internal architecture
- [ ] Add proper testing

---

## 🧠 Notes

- Tested by inspection with **wasm-bindgen** and served using **basic-http-server**
- The editor must be served over HTTP to work properly (file:// won’t work)
