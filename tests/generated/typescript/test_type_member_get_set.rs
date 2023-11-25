use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_type_member_get_set_ts_format_1_8763ccd3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("interface Foo {\n  get foo(): string;\n  set bar(v);\n}\n\ntype Foo = {\n  get foo(): string;\n  set bar(v);\n}\n\ninterface Foo {\n  set bar(foo: string);\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "interface Foo {\n  get foo(): string;\n  set bar(v);\n}\n\ntype Foo = {\n  get foo(): string;\n  set bar(v);\n};\n\ninterface Foo {\n  set bar(foo: string);\n}");
}
