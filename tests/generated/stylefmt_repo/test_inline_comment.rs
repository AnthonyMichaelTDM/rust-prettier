#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_inline_comment_css_format_1_c96a675f() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("\n  // inline comments\n\n\n\n    .class {color: red;}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "// inline comments\n\n.class {\n  color: red;\n}"
    );
}
