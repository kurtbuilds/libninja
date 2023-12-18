#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Transaction {
    #[serde(flatten)]
    pub transaction_base: TransactionBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_datetime: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counterparties: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datetime: Option<chrono::DateTime<chrono::Utc>>,
    pub payment_channel: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_finance_category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_finance_category_icon_url: Option<String>,
    pub transaction_code: String,
}
impl std::fmt::Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for Transaction {
    type Target = TransactionBase;
    fn deref(&self) -> &Self::Target {
        &self.transaction_base
    }
}
impl std::ops::DerefMut for Transaction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.transaction_base
    }
}