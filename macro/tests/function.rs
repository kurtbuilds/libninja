use pretty_assertions::assert_eq;

use ln_macro::function;
use mir::{Function, Visibility};

#[test]
fn test_function() {
    let s: Function<String> = function!(async main() {});
    assert_eq!(s.name.0, "main");
    assert_eq!(s.async_, true);
    assert_eq!(s.vis, Visibility::Public);
}

#[test]
fn test_function_args() {
    let s: Function<String> = function!(print_repeated(s: str, n: int) {});
    assert_eq!(s.name.0, "print_repeated");
    assert_eq!(s.async_, false);
    assert_eq!(s.vis, Visibility::Private);
    assert_eq!(s.args.len(), 2);
    assert_eq!(s.args[0].name().unwrap(), "s");
    assert_eq!(s.args[0].ty().unwrap(), "str");
    assert_eq!(s.args[1].name().unwrap(), "n");
    assert_eq!(s.args[1].ty().unwrap(), "int");
    assert_eq!(s.ret, "".to_string());
}

#[test]
fn test_function_return() {
    let s: Function<String> = function!(add(a: int, b: int) -> int {});
    assert_eq!(s.name.0, "add");
    assert_eq!(s.async_, false);
    assert_eq!(s.vis, Visibility::Private);
    assert_eq!(s.args.len(), 2);
    assert_eq!(s.ret, "int".to_string());
}

#[test]
fn test_interpolation_in_arg_position() {
    let z = "int";
    let s: Function<String> = function!(add(a: int, b: #z) -> int {});
    assert_eq!(s.name.0, "add");
    assert_eq!(s.async_, false);
    assert_eq!(s.vis, Visibility::Private);
    assert_eq!(s.args.len(), 2);
    assert_eq!(s.args[1].ty().unwrap(), "int");
    assert_eq!(s.ret, "int".to_string());
}

#[test]
fn test_interpolation_in_ret_position() {
    let z = "int";
    let s: Function<String> = function!(add(a: int, b: int) -> #z {});
    assert_eq!(s.ret, "int");
}

#[test]
fn test_interpolation_in_name_position() {
    let z = "main";
    let s: Function<String> = function!(#z(a: int, b: int) {});
    assert_eq!(s.name.0, z);
}

#[test]
fn test_function_stringified_body() {
    let s: Function<String> = function!(debug_add(a: int, b: int) -> int {
        print(a);
        print(b);
        a + b;
    });
    assert_eq!(s.name.0, "debug_add");
    assert_eq!(
        s.body,
        "\
print(a)
print(b)
a + b\
"
        .to_string()
    );
}

#[test]
fn test_use_body_variable() {
    let s: Function<String> = function!(debug_add(a: int, b: int) -> int {
        print(a);
        print(b);
        a + b;
    });
    assert_eq!(s.name.0, "debug_add");
    assert_eq!(
        s.body,
        "\
print(a)
print(b)
a + b\
"
        .to_string()
    );
}
