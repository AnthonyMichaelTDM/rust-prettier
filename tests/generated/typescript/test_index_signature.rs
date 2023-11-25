#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_index_signature_ts_format_1_ef087fbd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class LocalStorage implements Storage {\n  [index: number]: string;\n  [key: string]: any;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class LocalStorage implements Storage {\n  [index: number]: string;\n  [key: string]: any;\n}");
}
#[test]
fn test_static_ts_format_1_b8f1899e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("class Foo {\n  static [value: string]: Type;\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "class Foo {\n  static [value: string]: Type;\n}");
}
