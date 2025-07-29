use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct I18n {
    messages: HashMap<String, HashMap<String, String>>,
    current_lang: String,
}

#[derive(Deserialize)]
struct Messages {
    messages: HashMap<String, String>,
}

impl I18n {
    pub fn new() -> Self {
        let mut i18n = Self {
            messages: HashMap::new(),
            current_lang: "en".to_string(),
        };
        
        // Load default languages
        i18n.load_language("en", include_str!("../locales/en.json"));
        i18n.load_language("zh", include_str!("../locales/zh.json"));
        
        i18n
    }
    
    pub fn load_language(&mut self, lang: &str, json_content: &str) {
        if let Ok(messages) = serde_json::from_str::<Messages>(json_content) {
            self.messages.insert(lang.to_string(), messages.messages);
        }
    }
    
    pub fn set_language(&mut self, lang: &str) {
        if self.messages.contains_key(lang) {
            self.current_lang = lang.to_string();
        }
    }
    
    pub fn t(&self, key: &str) -> String {
        self.messages
            .get(&self.current_lang)
            .and_then(|lang_messages| lang_messages.get(key))
            .cloned()
            .unwrap_or_else(|| {
                // Fallback to English
                self.messages
                    .get("en")
                    .and_then(|lang_messages| lang_messages.get(key))
                    .cloned()
                    .unwrap_or_else(|| key.to_string())
            })
    }
    
    pub fn current_language(&self) -> &str {
        &self.current_lang
    }
}

impl Default for I18n {
    fn default() -> Self {
        Self::new()
    }
}