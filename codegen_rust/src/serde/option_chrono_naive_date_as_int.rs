pub mod option_chrono_naive_date_as_int {
    use chrono::Datelike;
    use serde::de::{Deserializer, Error};
    use std::fmt;

    struct NaiveDateVisitor;

    impl<'de> serde::de::Visitor<'de> for NaiveDateVisitor {
        type Value = Option<chrono::NaiveDate>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "an integer that looks like a date")
        }

        fn visit_u64<E: Error>(self, value: u64) -> Result<Self::Value, E> {
            if value == 0 {
                Ok(None)
            } else {
                let day = value % 100;
                let month = (value / 100) % 100;
                let year = value / 10000;
                Ok(chrono::NaiveDate::from_ymd_opt(
                    year as i32,
                    month as u32,
                    day as u32,
                ))
            }
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<chrono::NaiveDate>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_u64(NaiveDateVisitor)
    }

    pub fn serialize<S>(value: &Option<chrono::NaiveDate>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if let Some(value) = value {
            let day = value.day() as i32;
            let month = value.month() as i32;
            let year = value.year();
            let value = year * 10000 + month * 100 + day;
            serializer.serialize_i64(value as i64)
        } else {
            serializer.serialize_i64(0)
        }
    }
}
