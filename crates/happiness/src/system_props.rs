//! System properties are exclusive, and provide translations to "real" css properties

use std::borrow::Cow;
use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::theme::breakpoint::Breakpoints;

/// Contains standard system properties and their translations, should only exist as a
/// singleton instance [`SYSTEM_PROPERTIES`](SYSTEM_PROPERTIES).
#[derive(Debug, Clone)]
pub struct SystemProperties {
    mappings: HashMap<String, String>,
}

impl SystemProperties {
    /// Create a new system properties instance
    fn new() -> Self {
        Self {
            mappings: [
                ("p", "padding"),
                ("pl", "paddingLeft"),
                ("pr", "paddingRight"),
                ("pt", "paddingTop"),
                ("pd", "paddingDown"),
                ("bgcolor", "backgroundColor"),
                ("bg", "background"),
            ]
            .into_iter()
            .map(|(k, v): (&str, &str)| (k.to_string(), v.to_string()))
            .collect(),
        }
    }
}

impl CssPropertyTranslator for SystemProperties {
    fn translate<'a>(&self, query: &'a str) -> Cow<'a, str> {
        self.mappings
            .get(query)
            .map(|result| Cow::Owned(result.clone()))
            .unwrap_or_else(|| Cow::Borrowed(query))
    }
}

pub static SYSTEM_PROPERTIES: Lazy<SystemProperties> = Lazy::new(|| SystemProperties::new());

/// attempts to translate a given css query into a modified one
#[derive(Debug)]
pub struct TranslationUnit {
    props: SystemProperties,
    bps: Breakpoints,
}

impl TranslationUnit {
    pub fn new(bps: &Breakpoints) -> Self {
        Self {
            props: SYSTEM_PROPERTIES.clone(),
            bps: bps.clone(),
        }
    }
}

impl CssPropertyTranslator for TranslationUnit {
    fn translate<'a>(&self, query: &'a str) -> Cow<'a, str> {
        if let Some(translated) = self.props.mappings.get(query) {
            Cow::Owned(translated.clone())
        } else if let Some(breakpoint) = self.bps.get(query) {
            Cow::Owned(format!("@media (min-width: {}px)", breakpoint.width()))
        } else {
            Cow::Borrowed(query)
        }
    }
}

/// Translate a given property into something else
pub trait CssPropertyTranslator {
    /// Translates
    fn translate<'a>(&self, query: &'a str) -> Cow<'a, str>;
}
