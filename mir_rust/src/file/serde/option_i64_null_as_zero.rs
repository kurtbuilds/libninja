pub mod option_i64_null_as_zero {
    use serde::de::{Deserializer, Error};
    use std::fmt;

    struct IntVisitor;

    impl<'de> serde::de::Visitor<'de> for IntVisitor {
        type Value = Option<i64>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "an integer")
        }

        fn visit_i64<E: Error>(self, value: i64) -> Result<Self::Value, E> {
            if value == 0 {
                Ok(None)
            } else {
                Ok(Some(value))
            }
        }

        fn visit_u64<E: Error>(self, value: u64) -> Result<Self::Value, E> {
            if value == 0 {
                Ok(None)
            } else {
                Ok(Some(value as i64))
            }
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_i64(IntVisitor)
    }

    pub fn serialize<S>(value: &Option<i64>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if let Some(i) = value {
            serializer.serialize_i64(*i)
        } else {
            serializer.serialize_i64(0)
        }
    }
}
