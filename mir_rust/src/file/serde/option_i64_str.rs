pub mod option_i64_str {
    use serde::de::{Deserializer, Error, Unexpected};
    use std::fmt;

    struct StrVisitor;

    impl<'de> serde::de::Visitor<'de> for StrVisitor {
        type Value = Option<i64>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "an integer")
        }

        fn visit_str<E: Error>(self, value: &str) -> Result<Self::Value, E> {
            if value.is_empty() {
                Ok(None)
            } else {
                value
                    .parse::<i64>()
                    .map(Some)
                    .map_err(|_| Error::invalid_value(Unexpected::Str(value), &self))
            }
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(StrVisitor)
    }

    pub fn serialize<S>(value: &Option<i64>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if let Some(i) = value {
            serializer.serialize_str(&i.to_string())
        } else {
            serializer.serialize_str("")
        }
    }
}
