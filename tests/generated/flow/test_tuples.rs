#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_labeled_js_babel_flow_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_labeled_js_format_1_51d998b9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("type T = [a: string, b: number];")?;
    assert_eq!(formatted, "type T = [a: string, b: number];");
    Ok(())
}
#[test]
fn test_optional_js_babel_flow_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_optional_js_format_1_ef69d145() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type A = [a?: string, +b?: number, -c?: boolean];\n\n// Unaffected\ntype B = [?string, [?string], (?string) => boolean];") ? ;
    assert_eq ! (formatted , "type A = [a?: string, +b?: number, -c?: boolean];\n\n// Unaffected\ntype B = [?string, [?string], (?string) => boolean];");
    Ok(())
}
#[test]
fn test_variance_js_babel_flow_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_variance_js_format_1_d97fd206() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("type T = [a: string, +b: number, -c: boolean];")?;
    assert_eq!(formatted, "type T = [a: string, +b: number, -c: boolean];");
    Ok(())
}
