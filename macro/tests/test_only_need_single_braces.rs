use libninja_macro::{body, function};
use pretty_assertions::assert_eq;

#[test]
fn test_only_need_single_braces() {
    let client_name = "foobar";
    let mut s = function!(pub NewClientFromEnv() -> #client_name);
    s.body = body! {
        baseUrl, exists := os.LookupEnv("PET_STORE_BASE_URL");
        if !exists {
            fmt.Fprintln(os.Stderr, "Environment variable PET_STORE_BASE_URL is not set.");
            os.Exit(1);
        }
        return Client{baseUrl: baseUrl}
    };
    assert_eq!(
        s.body,
        r#"
baseUrl, exists := os.LookupEnv("PET_STORE_BASE_URL")
if !exists {
    fmt.Fprintln(os.Stderr, "Environment variable PET_STORE_BASE_URL is not set.")
    os.Exit(1)
}
return Client{baseUrl : baseUrl}
"#
        .trim()
    );
}
