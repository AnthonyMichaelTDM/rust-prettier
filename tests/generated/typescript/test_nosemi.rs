use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_index_signature_ts_semifalse_format_1_ef087fbd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class LocalStorage implements Storage {\n  [index: number]: string;\n  [key: string]: any;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class LocalStorage implements Storage {\n  [index: number]: string\n  [key: string]: any\n}");
}
#[test]
fn test_interface_ts_semifalse_format_1_3bd02467() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("interface Inline { x: string }\n\ninterface MultiLine {\n    x: string;\n    y: string;\n}\n\ninterface InlineMultiple { x: string; y: string }") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "interface Inline {\n  x: string\n}\n\ninterface MultiLine {\n  x: string\n  y: string\n}\n\ninterface InlineMultiple {\n  x: string\n  y: string\n}");
}
#[test]
fn test_type_ts_semifalse_format_1_a93590ab() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type A = {disabled?: boolean, error?: string}\n\nconst foo: {aasdf?: string; asdf?: number; foob?: string; zzz?: string; yyy: string}  = {}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type A = { disabled?: boolean; error?: string }\n\nconst foo: {\n  aasdf?: string\n  asdf?: number\n  foob?: string\n  zzz?: string\n  yyy: string\n} = {}");
}
