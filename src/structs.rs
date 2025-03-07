#[derive(serde::Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct UIDescriptor {
    #[serde(rename = "type")]
    widget_type: String,
    class: Vec<String>,
    id: String,
    children: Vec<UIDescriptor>,
}
