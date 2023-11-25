#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_index_signature_ts_format_1_ef087fbd() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class LocalStorage implements Storage {\n  [index: number]: string;\n  [key: string]: any;\n}") ? ;
    assert_eq ! (formatted , "class LocalStorage implements Storage {\n  [index: number]: string;\n  [key: string]: any;\n}");
    Ok(())
}
#[test]
fn test_static_ts_format_1_b8f1899e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("class Foo {\n  static [value: string]: Type;\n}")?;
    assert_eq!(formatted, "class Foo {\n  static [value: string]: Type;\n}");
    Ok(())
}
