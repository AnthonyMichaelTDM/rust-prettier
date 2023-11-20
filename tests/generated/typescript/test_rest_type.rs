#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_complex_ts_format_1_9599b5ca() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["typescript"])
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("type TupleWithRest = [number, ...(1 extends 2 ? string[] : number[])];");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "type TupleWithRest = [number, ...(1 extends 2 ? string[] : number[])];"
    );
}
#[test]
fn test_infer_type_ts_format_1_8f7f911b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type Tail<T extends any[]> = T extends [infer U, ...infer R] ? R : never;\n\n// should remove parens from this, to avoid a type issue with TypeScript 4.0:\ntype Tail2<T extends any[]> = T extends [infer U, ...(infer R)] ? R : never;\n\n// but not remove parens from this:\ntype Tail3<T extends any[]> = T extends [infer U, ...(infer R)[]] ? R : never;\n\ntype ReduceNextElement<\n  T extends readonly unknown[]\n> = T extends readonly [infer V, ...infer R] ? [V, R] : never") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type Tail<T extends any[]> = T extends [infer U, ...infer R] ? R : never;\n\n// should remove parens from this, to avoid a type issue with TypeScript 4.0:\ntype Tail2<T extends any[]> = T extends [infer U, ...infer R] ? R : never;\n\n// but not remove parens from this:\ntype Tail3<T extends any[]> = T extends [infer U, ...(infer R)[]] ? R : never;\n\ntype ReduceNextElement<T extends readonly unknown[]> = T extends readonly [\n  infer V,\n  ...infer R,\n]\n  ? [V, R]\n  : never;");
}
#[test]
fn test_simple_ts_format_1_337cbed7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("type X = [...number[]];");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "type X = [...number[]];");
}
