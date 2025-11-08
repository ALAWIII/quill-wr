use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use wasm_bindgen::JsValue;

pub fn jsvalue_to_inlines(value: JsValue) -> Vec<Inline> {
    // Try to parse JsValue as { key: value } map
    let Ok(map): Result<HashMap<String, Value>, _> = serde_wasm_bindgen::from_value(value) else {
        return vec![];
    };

    map.into_iter()
        .filter_map(|(k, v)| match (k.as_str(), v) {
            ("bold", Value::Bool(true)) => Some(Inline::Bold),
            ("italic", Value::Bool(true)) => Some(Inline::Italic),
            ("underline", Value::Bool(true)) => Some(Inline::Underline),
            ("strike", Value::Bool(true)) => Some(Inline::Strike),
            ("code", Value::Bool(true)) => Some(Inline::Code),
            ("link", Value::String(s)) => Some(Inline::Link(s)),
            ("background", Value::String(s)) => Some(Inline::BgColor(s)),
            ("color", Value::String(s)) => Some(Inline::Color(s)),
            ("font", Value::String(s)) => Some(Inline::Font(s)),
            ("size", Value::String(s)) => Some(Inline::Size(s)),
            ("script", Value::String(ref s)) if s == "sub" => Some(Inline::Sub),
            ("script", Value::String(ref s)) if s == "super" => Some(Inline::Sup),
            ("align", Value::String(s)) => Some(Inline::Align(s)),
            (k, v) => Some(Inline::Custom(k.to_string(), v)),
        })
        .collect()
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)] // ✅ ADDED: Hash + Eq for HashSet
pub enum Inline {
    Bold,
    Italic,
    Underline,
    Link(String),
    Strike,
    Code,
    /// higlight color (background-color)
    BgColor(String),
    /// text color
    Color(String),
    /// font family
    Font(String),
    /// text size
    Size(String),
    Sub,
    Sup,
    Align(String),
    Custom(String, Value),
}

impl Inline {
    pub fn as_kv(&self) -> (String, Value) {
        use Inline::*;
        match self {
            Bold => ("bold".into(), Value::Bool(true)),
            Italic => ("italic".into(), Value::Bool(true)),
            Underline => ("underline".into(), Value::Bool(true)),
            Strike => ("strike".into(), Value::Bool(true)),
            Code => ("code".into(), Value::Bool(true)),
            Link(v) => ("link".into(), Value::String(v.clone())),
            BgColor(v) => ("background".into(), Value::String(v.clone())),
            Color(v) => ("color".into(), Value::String(v.clone())),
            Font(v) => ("font".into(), Value::String(v.clone())),
            Size(v) => ("size".into(), Value::String(v.clone())),
            Sub => ("script".into(), Value::String("sub".into())),
            Sup => ("script".into(), Value::String("super".into())),
            Align(v) => ("align".into(), Value::String(v.clone())),
            Custom(k, v) => (k.clone(), v.clone()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Embeds {
    Video(String),
    Image(String),
    Formula(String),
}

impl Embeds {
    pub fn get_value(&self) -> &str {
        match self {
            Self::Video(v) => v,
            Self::Image(v) => v,
            Self::Formula(v) => v,
        }
    }
    pub fn get_type_name(&self) -> &str {
        match self {
            Self::Video(..) => "video",
            Self::Image(..) => "image",
            _ => "formula",
        }
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum EmbedAttr {
    Width(String),
    Height(String),
    Alt(String),
    Align(String),
    /// custom/unknown embed attribute
    Custom(String, Value),
}

impl EmbedAttr {
    pub fn as_kv(&self) -> (String, Value) {
        use EmbedAttr::*;
        match self {
            Width(v) => ("width".into(), Value::String(v.clone())),
            Height(v) => ("height".into(), Value::String(v.clone())),
            Alt(v) => ("alt".into(), Value::String(v.clone())),
            Align(v) => ("align".into(), Value::String(v.clone())),
            Custom(k, v) => (k.clone(), v.clone()),
        }
    }
}
