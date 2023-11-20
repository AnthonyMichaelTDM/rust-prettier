#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_shadow_js_format_1_e430cb4f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("var o = {};\n(o.p: string);\n(o: $Shape<{p:string}>);");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "var o = {};\n(o.p: string);\n(o: $Shape<{ p: string }>);"
    );
}
#[test]
fn test_test_js_format_1_291edaf6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type Foo = {\n  field: number,\n}\n\nvar x: {field?: number} = {};\nvar y: $Shape<Foo> = x;\n(y.field: number)") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type Foo = {\n  field: number,\n};\n\nvar x: { field?: number } = {};\nvar y: $Shape<Foo> = x;\n(y.field: number);");
}
