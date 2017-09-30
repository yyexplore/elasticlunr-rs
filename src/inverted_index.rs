
use std::collections::HashMap;

use serde::ser::{Serialize, Serializer, SerializeMap};

#[derive(Debug, Clone)]
pub struct IndexItem {
    docs: HashMap<String, i64>,
    df: usize,
    children: HashMap<String, IndexItem>,
}

impl IndexItem {
    pub fn new() -> Self {
        IndexItem {
            docs: HashMap::new(),
            df: 0,
            children: HashMap::new(),
        }
    }

    pub fn add_token(&mut self, token: &str, doc_ref: &str, freq: i64) 
    {
        if let Some((idx, char)) = token.char_indices().next() {
            let item = self.children.entry(char.to_string()).or_insert(IndexItem::new());
            item.add_token(&token[idx..], doc_ref, freq);
        }

        if self.docs.contains_key(doc_ref) { self.df += 1; }
        self.docs.insert(doc_ref.into(), freq);
    }
}

// Manually implement serialize so `children` are inline
impl Serialize for IndexItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut state = serializer.serialize_map(Some(2 + self.children.len()))?;
        state.serialize_entry("df", &self.df)?;
        state.serialize_entry("docs", &self.docs)?;
        
        for (key, value) in &self.children {
            state.serialize_entry(key, value)?;
        }

        state.end()
    }
}

#[derive(Serialize, Debug)]
pub struct InvertedIndex {
    root: IndexItem,
}

impl InvertedIndex {
    pub fn new() -> Self {
        InvertedIndex {
            root: IndexItem::new(),
        }
    }

    pub fn add_token(&mut self, token: &str, doc_ref: &str, freq: i64) 
    {
        self.root.add_token(token, doc_ref, freq);
    }
}

