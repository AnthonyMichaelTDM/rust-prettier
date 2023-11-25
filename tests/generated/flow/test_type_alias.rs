use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_issue_100857_js_format_1_ff161920() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type FieldLayoutWith<\n  T : string,\n  S : unknown = { xxxxxxxx: number; y: string; }\n> = {\n  type: T;\n  code: string;\n  size: S;\n};\n\ntype FieldLayoutWith<\n  T : string,\n  S : unknown,\n> = {\n  type: T;\n  code: string;\n  size: S;\n};\n\ntype FieldLayoutWith<\n  T : string,\n> = {\n  type: T;\n  code: string;\n  size: S;\n};\n\ntype FieldLayoutWith<\n  T : stringgggggggggggggggggg,\n  S : stringgggggggggggggggggg\n> = {\n  type: T;\n  code: string;\n  size: S;\n};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type FieldLayoutWith<\n  T: string,\n  S: unknown = { xxxxxxxx: number, y: string },\n> = {\n  type: T,\n  code: string,\n  size: S,\n};\n\ntype FieldLayoutWith<T: string, S: unknown> = {\n  type: T,\n  code: string,\n  size: S,\n};\n\ntype FieldLayoutWith<T: string> = {\n  type: T,\n  code: string,\n  size: S,\n};\n\ntype FieldLayoutWith<\n  T: stringgggggggggggggggggg,\n  S: stringgggggggggggggggggg,\n> = {\n  type: T,\n  code: string,\n  size: S,\n};");
}
