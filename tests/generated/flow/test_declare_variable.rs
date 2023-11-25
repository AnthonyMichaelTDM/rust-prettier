#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_decare_js_babel_flow_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_decare_js_format_1_16ff7f1f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("declare var a: string;\ndeclare let b: string;\ndeclare const c: string;\ndeclare export var a: string;\ndeclare export let b: string;\ndeclare export const c: string;") ? ;
    assert_eq ! (formatted , "declare var a: string;\ndeclare let b: string;\ndeclare const c: string;\ndeclare export var a: string;\ndeclare export let b: string;\ndeclare export const c: string;");
    Ok(())
}
