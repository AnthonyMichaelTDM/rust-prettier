#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_issue_9874_ts_format_1_4dc95293() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "export type RequestNextDealAction = BaseAction<DealsActionTypes.REQUEST_NEXT_DEAL>;",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "export type RequestNextDealAction =\n  BaseAction<DealsActionTypes.REQUEST_NEXT_DEAL>;"
    );
}
#[test]
fn test_issue_100857_ts_format_1_87b994fd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type FieldLayoutWith<\n  T extends string,\n  S extends unknown = { width: string }\n> = {\n  type: T;\n  code: string;\n  size: S;\n};\n\ntype FieldLayoutWith<\n  T extends string,\n  S extends unknown,\n> = {\n  type: T;\n  code: string;\n  size: S;\n};\n\ntype FieldLayoutWith<\n  S extends unknown = { width: string }\n> = {\n  type: T;\n  code: string;\n  size: S;\n};\n\ntype FieldLayoutWith<\n  T extends stringggggggggggg,\n  T extends stringggggggggggg\n> = {\n  type: T;\n  code: string;\n  size: S;\n};\n\ntype FieldLayoutWith<\n  T extends stringggggggggggg,\n  S = stringggggggggggggggggg\n> = {\n  type: T;\n  code: string;\n  size: S;\n};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type FieldLayoutWith<\n  T extends string,\n  S extends unknown = { width: string },\n> = {\n  type: T;\n  code: string;\n  size: S;\n};\n\ntype FieldLayoutWith<T extends string, S extends unknown> = {\n  type: T;\n  code: string;\n  size: S;\n};\n\ntype FieldLayoutWith<S extends unknown = { width: string }> = {\n  type: T;\n  code: string;\n  size: S;\n};\n\ntype FieldLayoutWith<\n  T extends stringggggggggggg,\n  T extends stringggggggggggg,\n> = {\n  type: T;\n  code: string;\n  size: S;\n};\n\ntype FieldLayoutWith<\n  T extends stringggggggggggg,\n  S = stringggggggggggggggggg,\n> = {\n  type: T;\n  code: string;\n  size: S;\n};");
}
#[test]
fn test_pattern_parameter_ts_format_1_710ec194() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type C = {\n  foo([]?): void;\n  bar({}, []?): any;\n  baz(a: string, b: number, []?): void;\n};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type C = {\n  foo([]?): void;\n  bar({}, []?): any;\n  baz(a: string, b: number, []?): void;\n};");
}
