#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_decare_js_babel_flow_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_decare_js_format_1_16ff7f1f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("declare var a: string;\ndeclare let b: string;\ndeclare const c: string;\ndeclare export var a: string;\ndeclare export let b: string;\ndeclare export const c: string;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "declare var a: string;\ndeclare let b: string;\ndeclare const c: string;\ndeclare export var a: string;\ndeclare export let b: string;\ndeclare export const c: string;");
}
