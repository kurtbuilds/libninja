///A representation of a transaction
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Transaction {
    #[serde(flatten)]
    pub transaction_base: TransactionBase,
    ///The date that the transaction was authorized. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ( `YYYY-MM-DD` ).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorized_date: Option<chrono::NaiveDate>,
    /**Date and time when a transaction was authorized in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ( `YYYY-MM-DDTHH:mm:ssZ` ).

This field is returned for select financial institutions and comes as provided by the institution. It may contain default time values (such as 00:00:00).*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorized_datetime: Option<chrono::DateTime<chrono::Utc>>,
    ///The counterparties present in the transaction. Counterparties, such as the financial institutions, are extracted by Plaid from the raw description.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub counterparties: Vec<String>,
    /**Date and time when a transaction was posted in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ( `YYYY-MM-DDTHH:mm:ssZ` ).

This field is returned for select financial institutions and comes as provided by the institution. It may contain default time values (such as 00:00:00).*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub datetime: Option<chrono::DateTime<chrono::Utc>>,
    /**The channel used to make a payment.
`online:` transactions that took place online.

`in store:` transactions that were made at a physical location.

`other:` transactions that relate to banks, e.g. fees or deposits.

This field replaces the `transaction_type` field.*/
    pub payment_channel: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub personal_finance_category: Option<String>,
    ///A link to the icon associated with the primary personal finance category. The logo will always be 100x100 pixels.
    #[serde(default, skip_serializing_if = "Option::is_none")]
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