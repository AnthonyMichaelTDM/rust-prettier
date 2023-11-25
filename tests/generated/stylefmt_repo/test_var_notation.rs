#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_var_notation_css_format_1_7abc28b3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
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
