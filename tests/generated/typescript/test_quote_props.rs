#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_types_ts_quote_propsas_needed_format_1_49a1ec8e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .quote_props("as-needed")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type T = {\n  0: string;\n  5: number;\n}\n\ntype U = {\n  0: string;\n  \"5\": number;\n}") ? ;
    assert_eq ! (formatted , "type T = {\n  0: string;\n  5: number;\n};\n\ntype U = {\n  0: string;\n  \"5\": number;\n};");
    Ok(())
}
#[test]
fn test_types_ts_quote_propsconsistent_format_1_49a1ec8e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .quote_props("consistent")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type T = {\n  0: string;\n  5: number;\n}\n\ntype U = {\n  0: string;\n  \"5\": number;\n}") ? ;
    assert_eq ! (formatted , "type T = {\n  0: string;\n  5: number;\n};\n\ntype U = {\n  0: string;\n  \"5\": number;\n};");
    Ok(())
}
#[test]
fn test_types_ts_quote_propspreserve_format_1_49a1ec8e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .quote_props("preserve")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type T = {\n  0: string;\n  5: number;\n}\n\ntype U = {\n  0: string;\n  \"5\": number;\n}") ? ;
    assert_eq ! (formatted , "type T = {\n  0: string;\n  5: number;\n};\n\ntype U = {\n  0: string;\n  \"5\": number;\n};");
    Ok(())
}
