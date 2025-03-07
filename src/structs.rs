use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Debug, Default, Clone, Deserialize, Serialize)]
pub enum WidgetType {
    #[serde(rename = "button")]
    Button { content: String },

    #[serde(rename = "label")]
    Label { content: String },

    #[serde(rename = "dropDown")]
    DropDown { content: Vec<String> },

    #[default]
    #[serde(rename = "gap")]
    Gap,

    #[serde(rename = "row")]
    Row,

    #[serde(rename = "column")]
    Column,

    #[serde(rename = "keySampling")]
    KeyGetter,

    #[serde(rename = "input")]
    Input,

    #[serde(rename = "list")]
    List,
}

#[derive(Clone, Default, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct UIDescriptor {
    #[serde(rename = "type")]
    pub widget_type: WidgetType,
    pub class: Vec<String>,
    pub id: String,
    pub children: Vec<UIDescriptor>,
}

#[derive(Clone, Default, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ExtensionUI {
    pub name: String,
    #[serde(rename = "ui")]
    pub widgets: Vec<UIDescriptor>,
}
