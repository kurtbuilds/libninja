pub fn default<T: Default>() -> T {
    Default::default()
}

#[macro_export]
macro_rules! assert_code_eq {
    ($code: expr, $expected:expr) => {
        pretty_assertions::assert_eq!(
            mir_rust::format_code(mir_rust::ToRustCode::to_rust_code($code)),
            mir_rust::format_string($expected)
        );
    };
    () => {};
}
#[macro_export]
macro_rules! bmap {
    ($($key:expr => $value:expr),* $(,)?) => {
        {
            let mut map = std::collections::BTreeMap::new();
            $(map.insert($key.into(), $value);)*
            map
        }
    };
    () => {};
}
