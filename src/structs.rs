use serde::Deserialize;

#[derive(PartialEq, Eq, Debug, Default, Clone, Deserialize)]
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
    List { fields: Vec<String> },
}

#[derive(Clone, Default, Debug, PartialEq, Eq, Deserialize)]
pub struct WidgetDescriptor {
    #[serde(rename = "type")]
    widget_type: WidgetType,
    class: Vec<String>,
    id: String,
    children: Vec<WidgetDescriptor>,
}

#[derive(Clone, Default, Debug, PartialEq, Eq, Deserialize)]
pub struct ExtensionUI {
    name: String,
    widgets: Vec<WidgetDescriptor>,
}
