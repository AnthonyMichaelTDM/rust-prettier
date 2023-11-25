#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_complex_ts_format_1_78f4bce1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type T = [(\"a\" | \"b\")?];\ntype TupleWithOptional = [number, (1 extends 2 ? string[] : number[])?];") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type T = [(\"a\" | \"b\")?];\ntype TupleWithOptional = [number, (1 extends 2 ? string[] : number[])?];");
}
#[test]
fn test_simple_ts_format_1_d84b091b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("type X = [number, string?];");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "type X = [number, string?];");
}
