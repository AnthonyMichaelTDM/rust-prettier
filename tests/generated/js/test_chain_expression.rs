#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_test_js_format_1_7cc940dc() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer
        .format("(a?.b).c;\n(a?.()).b;\n\n(a?.b)();\n(a?.())();\n\nnew (a?.b)();\nnew (a?.())();");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "(a?.b).c;\n(a?.()).b;\n\n(a?.b)();\n(a?.())();\n\nnew (a?.b)();\nnew (a?.())();"
    );
}
