//! Used for laying out happiness

use std::collections::HashMap;
use yew::{Children, function_component, Html, html, html_nested, Properties};
use crate::components::surfaces::Sheet;

pub mod flex {
    //! Contains flex enums

    use strum::AsRefStr;

    #[derive(Debug, Default, PartialEq, Eq, AsRefStr)]
    pub enum Direction {
        #[default]
        Row,
        RowReverse,
        Column,
        ColumnReverse
    }

    #[derive(Debug, Default, PartialEq, Eq,AsRefStr)]
    pub enum AlignItems {
        #[default]
        FlexStart,
        Center,
        FlexEnd,
        Stretch,
        Baseline
    }

    #[derive(Debug, Default, PartialEq, Eq,AsRefStr)]
    pub enum JustifyContent {
        #[default]
        FlexStart,
        Center,
        FlexEnd,
        SpaceBetween,
        SpaceAround,
        SpaceEvenly
    }
}

#[derive(Debug, Properties, PartialEq)]
pub struct StackProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub direction: flex::Direction,
    #[prop_or_default]
    pub align_items: flex::AlignItems,
    #[prop_or_default]
    pub justify_content: flex::JustifyContent,
    #[prop_or_default]
    pub spacing: f64
}

#[function_component]
pub fn Stack(props: &StackProps) -> Html {
    let mut style = HashMap::<&str, &str>::new();
    style.insert("flexDirection", props.direction.as_ref());

    return html! {
        <Sheet>
            <div style={style.iter().map(|(key, value)| format!("{key}: {value};")).collect::<String>()}>
                {for props.children.iter()}
            </div>
        </Sheet>
    }
}