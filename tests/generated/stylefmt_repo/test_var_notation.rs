#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_var_notation_css_format_1_7abc28b3() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format(
        ".foo {margin:var(     --n-gutterSize) 0 var(   --gutterSize ) var( --n-gutterSize );\n}",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        ".foo {\n  margin: var(--n-gutterSize) 0 var(--gutterSize) var(--n-gutterSize);\n}"
    );
}
