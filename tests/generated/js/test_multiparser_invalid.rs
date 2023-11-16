#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_text_js_acorn_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_text_js_espree_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_text_js_flow_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_text_js_meriyah_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_text_js_typescript_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_text_js_format_1_61ef9dd2() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("foo = foo\\`\\\\u{prettier}\\\\u{0065}\\`;\nfoo = html\\`\\\\u{prettier}\\\\u{0065}\\`;\nfoo = graphql\\`\\\\u{prettier}\\\\u{0065}\\`;\nfoo = markdown\\`\\\\u{prettier}\\\\u{0065}\\`;\nfoo = css\\`\\\\u{prettier}\\\\u{0065}\\`;\nfoo = /* HTML */\\`\\\\u{prettier}\\\\u{0065}\\`;\nfoo = /* GraphQL */\\`\\\\u{prettier}\\\\u{0065}\\`;\n\nfoo = foo\\`\\\\u{prettier}\\${foo}pr\\\\u{0065}ttier\\`;\nfoo = html\\`\\\\u{prettier}\\${foo}pr\\\\u{0065}ttier\\`;\nfoo = graphql\\`\\\\u{prettier}\\${foo}pr\\\\u{0065}ttier\\`;\nfoo = markdown\\`\\\\u{prettier}\\${foo}pr\\\\u{0065}ttier\\`;\nfoo = css\\`\\\\u{prettier}\\${foo}pr\\\\u{0065}ttier\\`;\nfoo = /* HTML */\\`\\\\u{prettier}\\${foo}pr\\\\u{0065}ttier\\`;\nfoo = /* GraphQL */\\`\\\\u{prettier}\\${foo}pr\\\\u{0065}ttier\\`;\n\nfoo = foo\\`pr\\\\u{0065}ttier\\${foo}\\\\u{prettier}\\`;\nfoo = html\\`pr\\\\u{0065}ttier\\${foo}\\\\u{prettier}\\`;\nfoo = graphql\\`pr\\\\u{0065}ttier\\${foo}\\\\u{prettier}\\`;\nfoo = markdown\\`pr\\\\u{0065}ttier\\${foo}\\\\u{prettier}\\`;\nfoo = css\\`pr\\\\u{0065}ttier\\${foo}\\\\u{prettier}\\`;\nfoo = /* HTML */\\`pr\\\\u{0065}ttier\\${foo}\\\\u{prettier}\\`;\nfoo = /* GraphQL */\\`pr\\\\u{0065}ttier\\${foo}\\\\u{prettier}\\`;\n\nfoo = foo\\`pr\\\\u{0065}ttier\\${foo}\\\\u{prettier}\\${bar}pr\\\\u{0065}ttier\\`;\nfoo = html\\`pr\\\\u{0065}ttier\\${foo}\\\\u{prettier}\\${bar}pr\\\\u{0065}ttier\\`;\nfoo = graphql\\`pr\\\\u{0065}ttier\\${foo}\\\\u{prettier}\\${bar}pr\\\\u{0065}ttier\\`;\nfoo = markdown\\`pr\\\\u{0065}ttier\\${foo}\\\\u{prettier}\\${bar}pr\\\\u{0065}ttier\\`;\nfoo = css\\`pr\\\\u{0065}ttier\\${foo}\\\\u{prettier}\\${bar}pr\\\\u{0065}ttier\\`;\nfoo = /* HTML */\\`pr\\\\u{0065}ttier\\${foo}\\\\u{prettier}\\${bar}pr\\\\u{0065}ttier\\`;\nfoo = /* GraphQL */\\`pr\\\\u{0065}ttier\\${foo}\\\\u{prettier}\\${bar}pr\\\\u{0065}ttier\\`;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "foo = foo\\`\\\\u{prettier}\\\\u{0065}\\`;\nfoo = html\\`\\\\u{prettier}\\\\u{0065}\\`;\nfoo = graphql\\`\\\\u{prettier}\\\\u{0065}\\`;\nfoo = markdown\\`\\\\u{prettier}\\\\u{0065}\\`;\nfoo = css\\`\\\\u{prettier}\\\\u{0065}\\`;\nfoo = /* HTML */ \\`\\\\u{prettier}\\\\u{0065}\\`;\nfoo = /* GraphQL */ \\`\\\\u{prettier}\\\\u{0065}\\`;\n\nfoo = foo\\`\\\\u{prettier}\\${foo}pr\\\\u{0065}ttier\\`;\nfoo = html\\`\\\\u{prettier}\\${foo}pr\\\\u{0065}ttier\\`;\nfoo = graphql\\`\\\\u{prettier}\\${foo}pr\\\\u{0065}ttier\\`;\nfoo = markdown\\`\\\\u{prettier}\\${foo}pr\\\\u{0065}ttier\\`;\nfoo = css\\`\\\\u{prettier}\\${foo}pr\\\\u{0065}ttier\\`;\nfoo = /* HTML */ \\`\\\\u{prettier}\\${foo}pr\\\\u{0065}ttier\\`;\nfoo = /* GraphQL */ \\`\\\\u{prettier}\\${foo}pr\\\\u{0065}ttier\\`;\n\nfoo = foo\\`pr\\\\u{0065}ttier\\${foo}\\\\u{prettier}\\`;\nfoo = html\\`pr\\\\u{0065}ttier\\${foo}\\\\u{prettier}\\`;\nfoo = graphql\\`pr\\\\u{0065}ttier\\${foo}\\\\u{prettier}\\`;\nfoo = markdown\\`pr\\\\u{0065}ttier\\${foo}\\\\u{prettier}\\`;\nfoo = css\\`pr\\\\u{0065}ttier\\${foo}\\\\u{prettier}\\`;\nfoo = /* HTML */ \\`pr\\\\u{0065}ttier\\${foo}\\\\u{prettier}\\`;\nfoo = /* GraphQL */ \\`pr\\\\u{0065}ttier\\${foo}\\\\u{prettier}\\`;\n\nfoo = foo\\`pr\\\\u{0065}ttier\\${foo}\\\\u{prettier}\\${bar}pr\\\\u{0065}ttier\\`;\nfoo = html\\`pr\\\\u{0065}ttier\\${foo}\\\\u{prettier}\\${bar}pr\\\\u{0065}ttier\\`;\nfoo = graphql\\`pr\\\\u{0065}ttier\\${foo}\\\\u{prettier}\\${bar}pr\\\\u{0065}ttier\\`;\nfoo = markdown\\`pr\\\\u{0065}ttier\\${foo}\\\\u{prettier}\\${bar}pr\\\\u{0065}ttier\\`;\nfoo = css\\`pr\\\\u{0065}ttier\\${foo}\\\\u{prettier}\\${bar}pr\\\\u{0065}ttier\\`;\nfoo = /* HTML */ \\`pr\\\\u{0065}ttier\\${foo}\\\\u{prettier}\\${bar}pr\\\\u{0065}ttier\\`;\nfoo = /* GraphQL */ \\`pr\\\\u{0065}ttier\\${foo}\\\\u{prettier}\\${bar}pr\\\\u{0065}ttier\\`;");
}
