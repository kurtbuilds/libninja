#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SumType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field1: Option<String>,
    pub field2: String,
}
impl std::fmt::Display for SumType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
