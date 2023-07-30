use proc_macro2::TokenStream;
use quote::quote;

pub fn option_i64_null_as_zero_module() -> TokenStream {
    quote! {
        pub mod option_i64_null_as_zero {
            use std::fmt;
            use serde::de::{Error, Unexpected, Deserializer};

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
    }
}


pub fn option_chrono_naive_date_as_int_module() -> TokenStream {
    quote! {
        pub mod option_chrono_naive_date_as_int {
            use std::fmt;
            use serde::de::{Error, Unexpected, Deserializer};
            use chrono::Datelike;

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
                        Ok(chrono::NaiveDate::from_ymd_opt(year as i32, month as u32, day as u32))
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
    }
}

pub fn option_decimal_as_str_module() -> TokenStream {
    quote!{
        pub mod option_decimal_as_str {
            use std::fmt;
            use std::str::FromStr;
            use ::serde::de::{Error, Deserializer};
            use rust_decimal::Decimal;

            struct DecimalVisitor;

            impl<'de> ::serde::de::Visitor<'de> for DecimalVisitor {
                type Value = Option<Decimal>;
                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    write!(formatter, "an string, possibly empty, representing a decimal")
                }
                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
                    if v.is_empty() {
                        Ok(None)
                    } else {
                        let d = Decimal::from_str(v).unwrap();
                        Decimal::from_str(v)
                            .map(Some)
                            .map_err(|_| E::custom("invalid decimal"))
                    }
                }
            }

            pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
                where
                    D: Deserializer<'de>,
            {
                deserializer.deserialize_str(DecimalVisitor)
            }

            pub fn serialize<S>(value: &Option<Decimal>, serializer: S) -> Result<S::Ok, S::Error>
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
    }
}