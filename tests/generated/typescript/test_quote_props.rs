use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_types_ts_quote_propsas_needed_format_1_49a1ec8e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .quote_props("as-needed")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type T = {\n  0: string;\n  5: number;\n}\n\ntype U = {\n  0: string;\n  \"5\": number;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type T = {\n  0: string;\n  5: number;\n};\n\ntype U = {\n  0: string;\n  \"5\": number;\n};");
}
#[test]
fn test_types_ts_quote_propsconsistent_format_1_49a1ec8e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .quote_props("consistent")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type T = {\n  0: string;\n  5: number;\n}\n\ntype U = {\n  0: string;\n  \"5\": number;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type T = {\n  0: string;\n  5: number;\n};\n\ntype U = {\n  0: string;\n  \"5\": number;\n};");
}
#[test]
fn test_types_ts_quote_propspreserve_format_1_49a1ec8e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .quote_props("preserve")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type T = {\n  0: string;\n  5: number;\n}\n\ntype U = {\n  0: string;\n  \"5\": number;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type T = {\n  0: string;\n  5: number;\n};\n\ntype U = {\n  0: string;\n  \"5\": number;\n};");
}
