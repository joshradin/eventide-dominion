use yew::UseStateHandle;
use std::ops::{Deref, DerefMut};
use crate::theme::theme_mode::ThemeMode;

#[derive(Debug, Clone, PartialEq)]
pub struct ThemeModeContext {
    ctx: UseStateHandle<ThemeMode>
}

impl ThemeModeContext {
    pub fn new(ctx: UseStateHandle<ThemeMode>) -> Self {
        Self { ctx }
    }
}


impl Deref for ThemeModeContext {
    type Target = UseStateHandle<ThemeMode>;

    fn deref(&self) -> &Self::Target {
        &self.ctx
    }
}

impl DerefMut for ThemeModeContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ctx
    }
}
