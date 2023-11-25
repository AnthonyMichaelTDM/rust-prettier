#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_complex_ts_format_1_9599b5ca() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("type TupleWithRest = [number, ...(1 extends 2 ? string[] : number[])];")?;
    assert_eq!(
        formatted,
        "type TupleWithRest = [number, ...(1 extends 2 ? string[] : number[])];"
    );
    Ok(())
}
#[test]
fn test_infer_type_ts_format_1_8f7f911b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type Tail<T extends any[]> = T extends [infer U, ...infer R] ? R : never;\n\n// should remove parens from this, to avoid a type issue with TypeScript 4.0:\ntype Tail2<T extends any[]> = T extends [infer U, ...(infer R)] ? R : never;\n\n// but not remove parens from this:\ntype Tail3<T extends any[]> = T extends [infer U, ...(infer R)[]] ? R : never;\n\ntype ReduceNextElement<\n  T extends readonly unknown[]\n> = T extends readonly [infer V, ...infer R] ? [V, R] : never") ? ;
    assert_eq ! (formatted , "type Tail<T extends any[]> = T extends [infer U, ...infer R] ? R : never;\n\n// should remove parens from this, to avoid a type issue with TypeScript 4.0:\ntype Tail2<T extends any[]> = T extends [infer U, ...infer R] ? R : never;\n\n// but not remove parens from this:\ntype Tail3<T extends any[]> = T extends [infer U, ...(infer R)[]] ? R : never;\n\ntype ReduceNextElement<T extends readonly unknown[]> = T extends readonly [\n  infer V,\n  ...infer R,\n]\n  ? [V, R]\n  : never;");
    Ok(())
}
#[test]
fn test_simple_ts_format_1_337cbed7() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("type X = [...number[]];")?;
    assert_eq!(formatted, "type X = [...number[]];");
    Ok(())
}
