use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_basic_html_format_1_bc74a71e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<ng-container *ngTemplateOutlet=\"someTmpl; context: {app}\"></ng-container>\n<div dir [input]=\"{   a,   b:   2   }\"></div>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<ng-container *ngTemplateOutlet=\"someTmpl; context: { app }\"></ng-container>\n<div dir [input]=\"{ a, b: 2 }\"></div>");
}
