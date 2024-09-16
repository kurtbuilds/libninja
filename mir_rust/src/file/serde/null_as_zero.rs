pub mod null_as_zero {
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_str(DecimalVisitor)
    }

    pub fn serialize<S>(value: &Decimal, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = crate::str::to_str_internal(value, true, None);
        serializer.serialize_str(value.0.as_ref())
    }
}
