#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    #[serde(flatten)]
    pub transaction_base: TransactionBase,
    pub authorized_date: Option<String>,
    pub authorized_datetime: Option<String>,
    pub counterparties: Vec<String>,
    pub datetime: Option<String>,
    pub payment_channel: String,
    pub personal_finance_category: String,
    pub personal_finance_category_icon_url: String,
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