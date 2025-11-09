use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_wasm_bindgen::to_value;
use std::collections::HashMap;
use wasm_bindgen::{JsCast, JsValue};

use crate::{EmbedAttr, Embeds, Inline, embed_attrs_to_map, inlines_to_map, quil_api::api::Delta};
#[derive(Serialize, Deserialize, Debug)]
pub enum DeltaOps {
    Insert(String, Vec<Inline>),
    Delete(usize),
    Retain(usize, Vec<Inline>),
}

fn ops_to_jsvalue(ops: &[DeltaOps]) -> JsValue {
    let js_ops: Vec<serde_json::Value> = ops
        .iter()
        .map(|op| match op {
            DeltaOps::Insert(text, inlines) => serde_json::json!({
                "insert": text,
                "attributes": inlines.iter().map(|i| i.as_kv()).collect::<serde_json::Map<_, _>>()
            }),
            DeltaOps::Retain(length, attrs) => serde_json::json!({
                "retain": length,
                "attributes": attrs.iter().map(|a| a.as_kv()).collect::<serde_json::Map<_, _>>()
            }),
            DeltaOps::Delete(length) => serde_json::json!({ "delete": length }),
        })
        .collect();

    serde_wasm_bindgen::to_value(&js_ops).unwrap()
}

pub struct DeltaEditor {
    delta: Delta,
}

impl DeltaEditor {
    pub fn new(ops: Vec<DeltaOps>) -> Self {
        let js_ops = ops_to_jsvalue(&ops);
        Self {
            delta: Delta::new(js_ops),
        }
    }
    pub fn from_delta(delta: Delta) -> Self {
        Self { delta }
    }
    pub fn get_inner_delta(&self) -> &Delta {
        &self.delta
    }
    pub fn to_json(&self) -> Result<String, JsValue> {
        js_sys::JSON::stringify(&self.delta.clone()).map(|s| s.as_string().unwrap())
    }

    // Deserialize from JSON string
    pub fn from_json(json: &str) -> Result<Self, JsValue> {
        let js_val = js_sys::JSON::parse(json)?;
        let delta: Delta = js_val.unchecked_into();
        Ok(Self { delta })
    }
    // insert text with attributes
    pub fn insert_text(&mut self, text: &str, attributes: &[Inline]) -> &mut Self {
        let attrs_js = to_value(&inlines_to_map(attributes)).unwrap();
        self.delta = self.delta.insert(&to_value(text).unwrap(), &attrs_js);
        self
    }

    // insert embed with attributes
    pub fn insert_embed(&mut self, embed: Embeds, attributes: &[EmbedAttr]) -> &mut Self {
        let embed_obj = match embed {
            Embeds::Image(url) => serde_json::json!({ "image": url }),
            Embeds::Video(url) => serde_json::json!({ "video": url }),
            Embeds::Formula(f) => serde_json::json!({ "formula": f }),
        };
        let attrs_js = to_value(&embed_attrs_to_map(attributes)).unwrap();
        self.delta = self.delta.insert(&to_value(&embed_obj).unwrap(), &attrs_js);
        self
    }

    // delete N characters
    pub fn delete(&mut self, length: u32) -> &mut Self {
        self.delta = self.delta.delete(length);
        self
    }

    // retain N characters with optional attributes
    pub fn retain(&mut self, length: u32, attributes: &[Inline]) -> &mut Self {
        let attrs_js = to_value(&inlines_to_map(attributes)).unwrap();
        self.delta = self.delta.retain(length, &attrs_js);
        self
    }

    // returns a new delta object contains the concatenated old values!!
    pub fn concat(&self, other: &DeltaEditor) -> DeltaEditor {
        DeltaEditor::from_delta(self.delta.concat(&other.delta))
    }
    pub fn diff(&mut self, other: &mut DeltaEditor) -> DeltaEditor {
        DeltaEditor::from_delta(self.delta.diff(&other.delta))
    }
}
