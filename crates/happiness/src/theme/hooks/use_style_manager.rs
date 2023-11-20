use yew::hook;
use stylist::manager::StyleManager;
use crate::StyleManagerContext;

/// Use a theme
#[hook]
pub(crate) fn use_style_manager() -> StyleManager {
    let mgr = yew::use_context::<StyleManagerContext>();
    mgr.map(|m| (*m).clone()).unwrap_or_else(|| {
        StyleManager::builder()
            .prefix("happiness".into())
            .build()
            .unwrap_or_default()
    })
}
