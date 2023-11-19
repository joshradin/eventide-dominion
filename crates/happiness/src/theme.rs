//! Used for theming



mod color;

use std::ops::Deref;
use gloo::utils::document;
use yew::{Children, function_component, hook, Html, html, Properties};
pub use color::Color;

pub mod sx;
pub mod context;


/// The theme kind
#[derive(Debug, Clone, PartialEq, Default)]
pub enum ThemeMode {
    /// Dark mode
    Dark,
    /// Light mode
    #[default]
    Light,
    /// Follow the system
    System
}



#[derive(Debug, Clone, PartialEq, Default)]
pub struct Theme {
    pub mode: ThemeMode
}

