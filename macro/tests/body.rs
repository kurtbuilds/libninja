use ln_macro::body;
use pretty_assertions::assert_eq;

// #[test]
// fn test_easy_declaration() {
//     let declaration: String = body!(let a = 1);
//     assert_eq!(declaration, "let a = 1".to_string());
// }

#[test]
fn test_capture_outside() {
    let z = 1;
    let declaration: String = body!(let a = #z);
    assert_eq!(declaration, "let a = 1".to_string());
}

#[test]
fn test_no_whitespace() {
    let declaration: String = body!(request.method("get"));
    assert_eq!(declaration, r#"request.method("get")"#.to_string());
}

#[test]
fn test_go_assignment() {
    let declaration: String = body!(a := 5);
    assert_eq!(declaration, "a := 5".to_string());
}

#[test]
fn test_capture_three_vars() {
    let x = 1;
    let y = 2;
    let z = 3;
    let declaration: String = body!(
        let a = #z;
        let b = #y;
        let c = #x;
    );
    assert_eq!(
        declaration,
        "\
let a = 3
let b = 2
let c = 1\
"
        .to_string()
    );
}

#[test]
fn test_fn_spacing() {
    let declaration: String = body!(console.log(response));
    assert_eq!(declaration, "console.log(response)".to_string());
}

#[test]
fn test_go_assignment_spacing() {
    let ident = "a";
    let value = 5;
    let dec = body!(#ident := #value);
    assert_eq!(dec, "a := 5".to_string());
}

#[test]
fn test_go_doesnt_wrap_brace() {
    let inside = "\"api\" : \"v1\"";
    let b = body!(postBody, _ := json.Marshal(map[string]string{#inside}));
    assert_eq!(b, "postBody, _ := json.Marshal(map[string]string{\"api\" : \"v1\"})");
}