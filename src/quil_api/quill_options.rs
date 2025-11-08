use js_sys::{Array, Object, Reflect};
use wasm_bindgen::JsValue;
use web_sys::HtmlElement;

#[derive(Debug, Default)]
pub struct QuillOptions {
    theme: Option<String>,
    modules: Option<ModulesConfig>,
    placeholder: Option<String>,
    read_only: Option<bool>,
    formats: Option<Vec<String>>,
    bounds: Option<String>,
    debug: Option<String>,
    registry: Option<JsValue>,
}

impl QuillOptions {
    pub fn builder() -> QuillOptionsBuilder {
        QuillOptionsBuilder::new()
    }

    pub fn to_js_value(&self) -> JsValue {
        let obj = Object::new();

        if let Some(theme) = &self.theme {
            let _ = Reflect::set(&obj, &"theme".into(), &theme.into());
        }

        if let Some(modules) = &self.modules {
            let _ = Reflect::set(&obj, &"modules".into(), &modules.to_js_value());
        }

        if let Some(placeholder) = &self.placeholder {
            let _ = Reflect::set(&obj, &"placeholder".into(), &placeholder.into());
        }

        if let Some(read_only) = self.read_only {
            let _ = Reflect::set(&obj, &"readOnly".into(), &read_only.into());
        }

        if let Some(formats) = &self.formats {
            let arr = Array::new();
            for format in formats {
                arr.push(&format.into());
            }
            let _ = Reflect::set(&obj, &"formats".into(), &arr.into());
        }

        if let Some(bounds) = &self.bounds {
            let _ = Reflect::set(&obj, &"bounds".into(), &bounds.into());
        }

        if let Some(debug) = &self.debug {
            let _ = Reflect::set(&obj, &"debug".into(), &debug.into());
        }

        if let Some(registry) = &self.registry {
            let _ = Reflect::set(&obj, &"registry".into(), registry);
        }

        obj.into()
    }
}

#[derive(Debug, Default)]
pub struct ModulesConfig {
    toolbar: Option<ToolbarConfig>,
    clipboard: Option<JsValue>,
    keyboard: Option<JsValue>,
    history: Option<JsValue>,
}

impl ModulesConfig {
    pub fn to_js_value(&self) -> JsValue {
        let obj = Object::new();

        if let Some(toolbar) = &self.toolbar {
            let _ = Reflect::set(&obj, &"toolbar".into(), &toolbar.to_js_value());
        }

        if let Some(clipboard) = &self.clipboard {
            let _ = Reflect::set(&obj, &"clipboard".into(), clipboard);
        }

        if let Some(keyboard) = &self.keyboard {
            let _ = Reflect::set(&obj, &"keyboard".into(), keyboard);
        }

        if let Some(history) = &self.history {
            let _ = Reflect::set(&obj, &"history".into(), history);
        }

        obj.into()
    }
}

#[derive(Debug)]
pub enum ToolbarConfig {
    Simple(bool),
    Selector(String),
    Container(HtmlElement),
    Array(Vec<ToolbarItem>),
}

impl ToolbarConfig {
    pub fn to_js_value(&self) -> JsValue {
        match self {
            ToolbarConfig::Simple(val) => (*val).into(),
            ToolbarConfig::Selector(sel) => sel.into(),
            ToolbarConfig::Container(el) => el.clone().into(),
            ToolbarConfig::Array(items) => {
                let arr = Array::new();
                for item in items {
                    arr.push(&item.to_js_value());
                }
                arr.into()
            }
        }
    }
}

#[derive(Debug)]
pub enum ToolbarItem {
    Single(String),
    Group(Vec<String>),
    WithOptions { name: String, options: Vec<JsValue> },
}

impl ToolbarItem {
    pub fn to_js_value(&self) -> JsValue {
        match self {
            ToolbarItem::Single(s) => s.into(),
            ToolbarItem::Group(group) => {
                let arr = Array::new();
                for item in group {
                    arr.push(&item.into());
                }
                arr.into()
            }
            ToolbarItem::WithOptions { name, options } => {
                let obj = Object::new();
                let opts_arr = Array::new();
                for opt in options {
                    opts_arr.push(opt);
                }
                let _ = Reflect::set(&obj, &name.as_str().into(), &opts_arr.into());
                obj.into()
            }
        }
    }
}

pub struct QuillOptionsBuilder {
    quill_option: QuillOptions,
}

impl QuillOptionsBuilder {
    pub fn new() -> Self {
        Self {
            quill_option: QuillOptions::default(),
        }
    }

    pub fn theme(mut self, theme: impl Into<String>) -> Self {
        self.quill_option.theme = Some(theme.into());
        self
    }

    pub fn modules(mut self, modules: ModulesConfig) -> Self {
        self.quill_option.modules = Some(modules);
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.quill_option.placeholder = Some(placeholder.into());
        self
    }

    pub fn read_only(mut self, read_only: bool) -> Self {
        self.quill_option.read_only = Some(read_only);
        self
    }

    pub fn formats(mut self, formats: Vec<String>) -> Self {
        self.quill_option.formats = Some(formats);
        self
    }

    pub fn bounds(mut self, bounds: impl Into<String>) -> Self {
        self.quill_option.bounds = Some(bounds.into());
        self
    }

    pub fn debug(mut self, debug: impl Into<String>) -> Self {
        self.quill_option.debug = Some(debug.into());
        self
    }

    pub fn registry(mut self, registry: JsValue) -> Self {
        self.quill_option.registry = Some(registry);
        self
    }

    pub fn build(self) -> QuillOptions {
        self.quill_option
    }
}

// Builder for ModulesConfig
pub struct ModulesConfigBuilder {
    config: ModulesConfig,
}

impl ModulesConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: ModulesConfig::default(),
        }
    }

    pub fn toolbar(mut self, toolbar: ToolbarConfig) -> Self {
        self.config.toolbar = Some(toolbar);
        self
    }

    pub fn clipboard(mut self, clipboard: JsValue) -> Self {
        self.config.clipboard = Some(clipboard);
        self
    }

    pub fn keyboard(mut self, keyboard: JsValue) -> Self {
        self.config.keyboard = Some(keyboard);
        self
    }

    pub fn history(mut self, history: JsValue) -> Self {
        self.config.history = Some(history);
        self
    }

    pub fn build(self) -> ModulesConfig {
        self.config
    }
}

impl Default for QuillOptionsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ModulesConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}
