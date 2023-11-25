#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_declare_function_js_trailing_commaall_format_1_8a72d456() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\ndeclare function foo (this : number, a : string, b) : void\n\ndeclare function bar (this : number): void\n\ndeclare function baz (this : number, ...a : any): void") ? ;
    assert_eq ! (formatted , "// @flow\n\ndeclare function foo(this: number, a: string, b): void;\n\ndeclare function bar(this: number): void;\n\ndeclare function baz(this: number, ...a: any): void;");
    Ok(())
}
#[test]
fn test_function_declaration_js_trailing_commaall_format_1_efcc594e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nfunction foo (this : number, a : string, b) {}\n\nfunction bar (this : number) {}\n\nfunction baz (this : number, ...a) {}") ? ;
    assert_eq ! (formatted , "// @flow\n\nfunction foo(this: number, a: string, b) {}\n\nfunction bar(this: number) {}\n\nfunction baz(this: number, ...a) {}");
    Ok(())
}
#[test]
fn test_function_type_js_trailing_commaall_format_1_7ebc0a83() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\ntype T = (this : number, a : string, b : number) => void\n\ntype U = (this : number, ...c : any) => void\n\ntype V = (this : number) => void") ? ;
    assert_eq ! (formatted , "// @flow\n\ntype T = (this: number, a: string, b: number) => void;\n\ntype U = (this: number, ...c: any) => void;\n\ntype V = (this: number) => void;");
    Ok(())
}
#[test]
fn test_line_break_js_trailing_commaall_format_1_a4c65dc9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\ntype T = (this: boolean,\n          a: number,\n\n          b: number,\n         ) => boolean;\n\ntype T2 = (_this: boolean,\n          a: number,\n\n          b: number,\n         ) => boolean;\n\ntype A = (\n  this: SupperLongLongLongLongLongLongLongLongLongLongLongType,\n\n  b: number,\n) => boolean;\n\ntype B = (\n  _this: SupperLongLongLongLongLongLongLongLongLongLongLongType,\n\n  b: number,\n) => boolean") ? ;
    assert_eq ! (formatted , "// @flow\n\ntype T = (\n  this: boolean,\n  a: number,\n\n  b: number,\n) => boolean;\n\ntype T2 = (\n  _this: boolean,\n  a: number,\n\n  b: number,\n) => boolean;\n\ntype A = (\n  this: SupperLongLongLongLongLongLongLongLongLongLongLongType,\n\n  b: number,\n) => boolean;\n\ntype B = (\n  _this: SupperLongLongLongLongLongLongLongLongLongLongLongType,\n\n  b: number,\n) => boolean;");
    Ok(())
}
#[test]
fn test_method_js_trailing_commaall_format_1_1096e939() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nclass A {\n    m(this : number, a : number, b : string) {}\n    n(this : number, ...c) {}\n    o(this : number) {}\n}") ? ;
    assert_eq ! (formatted , "// @flow\n\nclass A {\n  m(this: number, a: number, b: string) {}\n  n(this: number, ...c) {}\n  o(this: number) {}\n}");
    Ok(())
}
#[test]
fn test_union_type_js_trailing_commaall_format_1_854b3c81() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\ntype A = (\n  this: | SupperLongLongLongLongLongLongLongLongLongLongLongType | FooBarBazLorem12345,\n  b: number,\n) => boolean;\n\ntype B = (\n  _this: | SupperLongLongLongLongLongLongLongLongLongLongLongType | FooBarBazLorem12345,\n  b: number,\n) => boolean") ? ;
    assert_eq ! (formatted , "// @flow\n\ntype A = (\n  this:\n    | SupperLongLongLongLongLongLongLongLongLongLongLongType\n    | FooBarBazLorem12345,\n  b: number,\n) => boolean;\n\ntype B = (\n  _this:\n    | SupperLongLongLongLongLongLongLongLongLongLongLongType\n    | FooBarBazLorem12345,\n  b: number,\n) => boolean;");
    Ok(())
}
