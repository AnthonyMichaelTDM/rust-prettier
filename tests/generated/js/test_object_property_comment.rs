#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_after_key_js_format_1_7d53dd76() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("let a = {\n   a /* comment */: () => 1\n};\n\nlet b = {\n   \"a\" /* comment */: () => 1\n};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "let a = {\n  a /* comment */: () => 1,\n};\n\nlet b = {\n  a /* comment */: () => 1,\n};"
    );
}
