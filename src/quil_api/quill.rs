use dashmap::DashMap;
use js_sys::Function;
use serde::Deserialize;
use serde_wasm_bindgen::to_value;
use uuid::Uuid;
use wasm_bindgen::{JsCast, JsValue, prelude::Closure};
use web_sys::{Node, js_sys};

use super::{Delta, Quill};
use crate::{
    DeltaEditor, Embeds, Inline, QuillOptions, hash_map_to_js_object, inlines_to_map,
    jsvalue_to_inlines,
};

use std::fmt::Display;
#[derive(Debug, Deserialize)]
pub struct Selection {
    pub index: u32,
    pub length: u32,
}
#[derive(Debug, Default)]
pub enum Source {
    #[default]
    Api,
    User,
    Silent,
}
impl Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Api => "api",
                Self::Silent => "silent",
                Self::User => "user",
            }
        )
    }
}
#[derive(Default, Debug, Clone, Copy)]
pub struct ContentRange {
    index: usize,
    length: Option<usize>,
}
impl ContentRange {
    pub fn new() -> Self {
        Self {
            index: 0,
            length: None,
        }
    }
    pub fn set_index(mut self, index: usize) -> Self {
        self.index = index;
        self
    }
    pub fn set_length(mut self, length: usize) -> Self {
        self.length = Some(length);
        self
    }

    pub fn index(&self) -> usize {
        self.index
    }
    pub fn length(&self) -> usize {
        self.length.unwrap_or_default()
    }
}
#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Bounds {
    top: f64,
    right: f64,
    bottom: f64,
    left: f64,
}
impl Bounds {
    pub fn new(top: f64, right: f64, bottom: f64, left: f64) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }
    pub fn from_js(value: JsValue) -> Option<Self> {
        let jsb: Bounds = serde_wasm_bindgen::from_value(value).ok()?;
        Some(Self {
            top: jsb.top,
            left: jsb.left,
            right: jsb.left,
            bottom: jsb.top,
        })
    }
}

pub enum EventChange {
    Text(Delta, Delta, Source),
    Selection(Option<ContentRange>, Option<ContentRange>, Source),
    Editor(String, Vec<JsValue>),
    Generic(String, Vec<JsValue>),
}
impl Display for EventChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Text(..) => "text-change",
                Self::Selection(..) => "selection-change",
                Self::Editor(..) => "editor-change",
                Self::Generic(evnt_name, _) => evnt_name,
            }
        )
    }
}
pub struct QuillEditor {
    /// todo : consider implementing a wrapper to support different closure kinds (FnMut,FnOnce)
    handlers: DashMap<Uuid, (String, Function)>,
    quill: Quill,
}
impl QuillEditor {
    pub fn new(selector: &str) -> Self {
        Self::new_with_options(selector, QuillOptions::default())
    }
    ///```no_run
    ///  let options = QuillOption::builder()
    ///    .theme("snow")
    ///    .placeholder("Write something...")
    ///    .read_only(false)
    ///    .modules(
    ///        ModulesConfigBuilder::new()
    ///            .toolbar(ToolbarConfig::Array(vec![
    ///                ToolbarItem::Group(vec!["bold".into(), "italic".into()]),
    ///                ToolbarItem::WithOptions {
    ///                    name: "header".into(),
    ///                    options: vec![1.into(), 2.into(), 3.into()],
    ///                },
    ///            ]))
    ///            .build()
    ///    )
    ///    .build();
    ///```
    //let quill = QuillEditor::new_with_options("#editor", options);
    pub fn new_with_options(selector: &str, options: QuillOptions) -> Self {
        let js_options = options.to_js_value();
        let quill = Quill::new(selector, &js_options);
        Self {
            quill,
            handlers: DashMap::new(),
        }
    }
    pub fn get_length(&self) -> u32 {
        self.quill.get_length()
    }
    /// default index=0 , length = get_length()
    pub fn get_text(&self, index: Option<u32>, length: Option<u32>) -> String {
        self.quill
            .get_text(index.unwrap_or(0), length.unwrap_or(self.get_length()))
    }
    /// this method is perfect for serializing content with styles/format to json!!
    ///
    /// default index=0 , length = get_length()
    pub fn get_content(&self, index: Option<u32>, length: Option<u32>) -> DeltaEditor {
        let delta = self
            .quill
            .get_contents(index.unwrap_or(0), length.unwrap_or(self.get_length()));
        DeltaEditor::from_delta(delta)
    }
    /// # warning
    /// broken unstable method
    ///
    ///  default index=0 , length = get_length()
    pub fn get_semantic_html(&self, index: Option<u32>, length: Option<u32>) -> String {
        self.quill.get_semantic_html(
            &to_value(&index.unwrap_or(0)).expect("Failed to convert to js"),
            length.unwrap_or(self.get_length()),
        )
    }
    /// source defaults to 'api'
    pub fn insert_embed(&self, index: u32, type_m: Embeds, source: Option<Source>) -> DeltaEditor {
        DeltaEditor::from_delta(self.quill.insert_embed(
            index,
            type_m.get_type_name(),
            &to_value(type_m.get_value()).expect("failed to convert value "),
            &source.unwrap_or(Source::Api).to_string(),
        ))
    }
    pub fn insert_text(
        &self,
        index: u32,
        text: &str,
        formats: &[Inline],
        source: Option<Source>,
    ) -> DeltaEditor {
        let formats = &inlines_to_map(formats);
        let js_formats = hash_map_to_js_object(formats);
        DeltaEditor::from_delta(self.quill.insert_text(
            index,
            text,
            &js_formats,
            &source.unwrap_or(Source::Api).to_string(),
        ))
    }
    pub fn set_content(&self, delta: &DeltaEditor, source: Option<Source>) -> DeltaEditor {
        DeltaEditor::from_delta(self.quill.set_contents(
            delta.get_inner_delta(),
            &source.unwrap_or(Source::Api).to_string(),
        ))
    }
    pub fn set_text(&self, text: &str, source: Option<Source>) -> DeltaEditor {
        DeltaEditor::from_delta(
            self.quill
                .set_text(text, &source.unwrap_or(Source::Api).to_string()),
        )
    }
    pub fn delete_text(&self, index: u32, length: u32, source: Option<Source>) -> DeltaEditor {
        DeltaEditor::from_delta(self.quill.delete_text(
            index,
            length,
            &source.unwrap_or(Source::Api).to_string(),
        ))
    }
    pub fn update_content(&self, delta: DeltaEditor, source: Option<Source>) -> DeltaEditor {
        DeltaEditor::from_delta(self.quill.update_contents(
            delta.get_inner_delta(),
            &source.unwrap_or(Source::Api).to_string(),
        ))
    }
    pub fn format(&self, name_value: Inline, source: Option<Source>) -> DeltaEditor {
        let (name, value) = name_value.as_kv();
        DeltaEditor::from_delta(self.quill.format(
            &name,
            &to_value(&value).expect("Failed to convert to jsValue"),
            &source.unwrap_or(Source::Api).to_string(),
        ))
    }
    /// source defaults to 'api'
    pub fn format_line(
        &self,
        index: u32,
        length: u32,
        formats: &[Inline],
        source: Option<Source>,
    ) -> DeltaEditor {
        let formats = to_value(&inlines_to_map(formats)).expect("Failed to convert");
        DeltaEditor::from_delta(self.quill.format_line(
            index,
            length,
            &formats,
            &source.unwrap_or(Source::Api).to_string(),
        ))
    }
    pub fn format_text(
        &self,
        index: u32,
        length: u32,
        formats: &[Inline],
        source: Option<Source>,
    ) -> DeltaEditor {
        let formats = to_value(&inlines_to_map(formats)).expect("Failed to convert");
        DeltaEditor::from_delta(self.quill.format_text(
            index,
            length,
            &formats,
            &source.unwrap_or(Source::Api).to_string(),
        ))
    }
    pub fn get_format(&self, index: u32, length: Option<u32>) -> Vec<Inline> {
        let js_inlines = self.quill.get_format(index, length.unwrap_or(0));
        jsvalue_to_inlines(js_inlines)
    }
    pub fn remove_format(
        &mut self,
        index: u32,
        length: u32,
        source: Option<Source>,
    ) -> DeltaEditor {
        DeltaEditor::from_delta(self.quill.remove_format(
            index,
            length,
            &source.unwrap_or(Source::Api).to_string(),
        ))
    }
    pub fn get_bounds(&self, index: u32, length: Option<u32>) -> Option<Bounds> {
        Bounds::from_js(self.quill.get_bounds(index, length.unwrap_or(0)))
    }
    pub fn get_selection(&self, focus: Option<bool>) -> Option<Selection> {
        let js_val = self.quill.get_selection(focus.unwrap_or(false));
        serde_wasm_bindgen::from_value(js_val).ok()
    }
    pub fn set_selection(&self, index: u32, length: Option<u32>, source: Option<Source>) {
        self.quill.set_selection(
            index,
            length.unwrap_or(0),
            &source.unwrap_or(Source::Api).to_string(),
        );
    }
    pub fn blur(&self) {
        self.quill.blur();
    }
    pub fn disable(&self) {
        self.enable(false);
    }
    pub fn enable(&self, enabled: bool) {
        self.quill.enable(enabled);
    }
    pub fn focus(&self, prevent_scroll: Option<bool>) {
        self.quill.focus(prevent_scroll.unwrap_or(false));
    }
    pub fn has_focus(&self) -> bool {
        self.quill.has_focus()
    }
    /// default to 'user'
    pub fn update(&self, source: Option<Source>) {
        self.quill
            .update(&source.unwrap_or(Source::User).to_string());
    }
    pub fn scroll_selection_view(&self) {
        self.quill.scroll_selection_view();
    }
    pub fn scroll_rect_into_view(&self, bounds: Bounds) {
        self.quill
            .scroll_rect_into_view(bounds.top, bounds.right, bounds.bottom, bounds.left);
    }

    //--------------------------event methods --------------------------
    pub fn on(&self, evt_name: &str, mut handler: impl FnMut(Vec<JsValue>) + 'static) -> Uuid {
        let wrapper = Closure::wrap(Box::new(move |args: &js_sys::Array| {
            // Collect all non-undefined arguments
            let vec: Vec<JsValue> = args.to_vec();
            handler(vec);
        }) as Box<dyn FnMut(&js_sys::Array)>);
        let func = wrapper.as_ref().unchecked_ref::<js_sys::Function>();
        let id = Uuid::new_v4();

        self.handlers
            .insert(id, (evt_name.to_string(), func.clone()));
        self.quill.on_event(evt_name, func);
        wrapper.forget();
        id
    }
    pub fn once(&self, evnt_name: &str, mut handler: impl FnMut(Vec<JsValue>) + 'static) {
        let wrapper = Closure::wrap(Box::new(move |args: &js_sys::Array| {
            let vec: Vec<JsValue> = args.to_vec();
            handler(vec);
        }) as Box<dyn FnMut(&js_sys::Array)>);
        let func = wrapper.as_ref().unchecked_ref::<js_sys::Function>();

        self.quill.once_event(evnt_name, func);
        wrapper.forget();
    }
    // its job is to disconnect the handler
    pub fn off(&self, id: Uuid) {
        if let Some((id, (evnt_name, func))) = self.handlers.remove(&id) {
            self.quill.off_event(&evnt_name, &func);
        }
    }
    //-------------------------------  ---------------------------------
    /// if None then level = false disables all messages, otherwise you should provide ("error","warn","log","info")
    pub fn debug(level: Option<&str>) {
        Quill::debug(level.unwrap_or("log"));
    }
    pub fn import(path: &str) -> JsValue {
        Quill::import(path)
    }
    pub fn add_container_class(class_name: &str, ref_node: Option<Node>) -> JsValue {
        Quill::add_container(
            &to_value(class_name).expect("failed to cast down"),
            ref_node,
        )
    }

    pub fn get_module(name: &str) -> JsValue {
        Quill::get_module(name)
    }

    // returns either Blot | Quill
    /*    pub fn find(&self, dom_node: &Node, bubble: Option<bool>) {
        let bubble = bubble.unwrap_or(false);
    }
    pub fn get_index(&self, blot: Blot) {}
    pub fn get_leaf(&self, index: usize) {}
    pub fn get_line(&self, index: usize) {}
    pub fn get_lines(&self, range: ContentRange) {}*/
}
