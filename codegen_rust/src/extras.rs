use hir::HirSpec;
use mir::{DateSerialization, IntegerSerialization};

#[derive(Debug)]
pub struct Extras {
    pub null_as_zero: bool,
    pub option_i64_str: bool,
    pub date_serialization: bool,
    pub currency: bool,
    pub integer_date_serialization: bool,
    pub basic_auth: bool,
    pub oauth2: bool,
}

impl Extras {
    pub fn needs_serde(&self) -> bool {
        self.null_as_zero || self.integer_date_serialization || self.option_i64_str
    }
}

pub fn calculate_extras(spec: &HirSpec) -> Extras {
    use mir::Ty;
    let mut null_as_zero = false;
    let mut date_serialization = false;
    let mut currency = false;
    let mut integer_date_serialization = false;
    let mut option_i64_str = false;
    for (_, record) in &spec.schemas {
        for field in record.fields() {
            match &field.ty {
                Ty::Integer {
                    ser: IntegerSerialization::NullAsZero,
                } => {
                    null_as_zero = true;
                }
                Ty::Integer {
                    ser: IntegerSerialization::String,
                } => {
                    option_i64_str = true;
                }
                Ty::Date {
                    ser: DateSerialization::Integer,
                } => {
                    integer_date_serialization = true;
                    date_serialization = true;
                }
                Ty::DateTime => {
                    date_serialization = true;
                }
                Ty::Currency { .. } => {
                    currency = true;
                }
                _ => {}
            }
        }
    }
    let basic_auth = spec.has_basic_auth();
    let oauth2 = spec.oauth2_auth().is_some();
    Extras {
        null_as_zero,
        date_serialization,
        integer_date_serialization,
        currency,
        option_i64_str,
        basic_auth,
        oauth2,
    }
}
