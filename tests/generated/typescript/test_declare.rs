#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_declare_class_fields_ts_format_1_814a35c6() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class B {p: number;}\nclass C extends B {declare p: 256 | 1000;}\nclass D {\n  declare field = \"field\";\n}\ndeclare class D {\n  field = \"field\";\n}") ? ;
    assert_eq ! (formatted , "class B {\n  p: number;\n}\nclass C extends B {\n  declare p: 256 | 1000;\n}\nclass D {\n  declare field = \"field\";\n}\ndeclare class D {\n  field = \"field\";\n}");
    Ok(())
}
#[test]
fn test_declare_enum_ts_format_1_ee82f6a3() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("declare const enum Foo {}\ndeclare enum Bar {}")?;
    assert_eq!(formatted, "declare const enum Foo {}\ndeclare enum Bar {}");
    Ok(())
}
#[test]
fn test_declare_function_ts_format_1_d86b0c43() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("declare function x();\ndeclare function y(): void;\n\ndeclare namespace A {\n    function x();\n    function y(): void;\n}\n\ndeclare function f([]?)") ? ;
    assert_eq ! (formatted , "declare function x();\ndeclare function y(): void;\n\ndeclare namespace A {\n  function x();\n  function y(): void;\n}\n\ndeclare function f([]?);");
    Ok(())
}
#[test]
fn test_declare_function_with_body_ts_format_1_3aa08d24() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// Invalid, but recoverable\ndeclare function foo() {}\ndeclare function bar() {\n  // comment\n}") ? ;
    assert_eq ! (formatted , "// Invalid, but recoverable\ndeclare function foo() {};\ndeclare function bar() {\n  // comment\n};");
    Ok(())
}
#[test]
fn test_declare_interface_ts_format_1_123e088c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("declare interface Dictionary<T> {\n  [index: string]: T\n}\n\ndeclare interface B {\n  foo([]?): void;\n  bar({}, []?): any;\n  baz(a: string, b: number, []?): void;\n}") ? ;
    assert_eq ! (formatted , "declare interface Dictionary<T> {\n  [index: string]: T;\n}\n\ndeclare interface B {\n  foo([]?): void;\n  bar({}, []?): any;\n  baz(a: string, b: number, []?): void;\n}");
    Ok(())
}
#[test]
fn test_declare_module_ts_format_1_b5b0447f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("declare module m {\n  class C {\n    field = \"field\";\n  }\n}")?;
    assert_eq!(
        formatted,
        "declare module m {\n  class C {\n    field = \"field\";\n  }\n}"
    );
    Ok(())
}
#[test]
fn test_declare_namespace_ts_format_1_f2e38ad9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("declare namespace m {\n  class C {\n    field = \"field\";\n  }\n}")?;
    assert_eq!(
        formatted,
        "declare namespace m {\n  class C {\n    field = \"field\";\n  }\n}"
    );
    Ok(())
}
#[test]
fn test_declare_var_ts_format_1_1d63c4a4() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// tslint:disable-next-line:no-use-before-declare\nconst hello = 5;\n\n// tslint:disable-next-line:no-use-before-declare\ndeclare const hello2 = 5;") ? ;
    assert_eq ! (formatted , "// tslint:disable-next-line:no-use-before-declare\nconst hello = 5;\n\n// tslint:disable-next-line:no-use-before-declare\ndeclare const hello2 = 5;");
    Ok(())
}
#[test]
fn test_declare_get_set_field_ts_format_1_fafda7b8() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("class C {\n  declare get: string\n  declare set: string;\n}")?;
    assert_eq!(
        formatted,
        "class C {\n  declare get: string;\n  declare set: string;\n}"
    );
    Ok(())
}
#[test]
fn test_object_type_in_declare_function_ts_format_1_59daf8de() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("declare function foo(this: { a: boolean, b: string, c: number }):\n  Promise<Array<foo>>\n\ndeclare function bazFlip({ a: boolean, b: string, c: number }):\n  Promise<Array<foo>>\n\ndeclare function bar(...{ a: boolean, b: string, c: number }):\n  Promise<Array<foo>>\n\ndeclare function bar(...x: { a: boolean, b: string, c: number }):\n  Promise<Array<foo>>") ? ;
    assert_eq ! (formatted , "declare function foo(this: {\n  a: boolean;\n  b: string;\n  c: number;\n}): Promise<Array<foo>>;\n\ndeclare function bazFlip({\n  a: boolean,\n  b: string,\n  c: number,\n}): Promise<Array<foo>>;\n\ndeclare function bar(\n  ...{ a: boolean, b: string, c: number }\n): Promise<Array<foo>>;\n\ndeclare function bar(\n  ...x: { a: boolean; b: string; c: number }\n): Promise<Array<foo>>;");
    Ok(())
}
