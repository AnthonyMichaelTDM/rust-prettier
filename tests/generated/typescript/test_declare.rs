#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_declare_class_fields_ts_format_1_814a35c6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class B {p: number;}\nclass C extends B {declare p: 256 | 1000;}\nclass D {\n  declare field = \"field\";\n}\ndeclare class D {\n  field = \"field\";\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class B {\n  p: number;\n}\nclass C extends B {\n  declare p: 256 | 1000;\n}\nclass D {\n  declare field = \"field\";\n}\ndeclare class D {\n  field = \"field\";\n}");
}
#[test]
fn test_declare_enum_ts_format_1_ee82f6a3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("declare const enum Foo {}\ndeclare enum Bar {}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "declare const enum Foo {}\ndeclare enum Bar {}");
}
#[test]
fn test_declare_function_ts_format_1_d86b0c43() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("declare function x();\ndeclare function y(): void;\n\ndeclare namespace A {\n    function x();\n    function y(): void;\n}\n\ndeclare function f([]?)") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "declare function x();\ndeclare function y(): void;\n\ndeclare namespace A {\n  function x();\n  function y(): void;\n}\n\ndeclare function f([]?);");
}
#[test]
fn test_declare_function_with_body_ts_format_1_3aa08d24() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// Invalid, but recoverable\ndeclare function foo() {}\ndeclare function bar() {\n  // comment\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// Invalid, but recoverable\ndeclare function foo() {};\ndeclare function bar() {\n  // comment\n};");
}
#[test]
fn test_declare_interface_ts_format_1_123e088c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("declare interface Dictionary<T> {\n  [index: string]: T\n}\n\ndeclare interface B {\n  foo([]?): void;\n  bar({}, []?): any;\n  baz(a: string, b: number, []?): void;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "declare interface Dictionary<T> {\n  [index: string]: T;\n}\n\ndeclare interface B {\n  foo([]?): void;\n  bar({}, []?): any;\n  baz(a: string, b: number, []?): void;\n}");
}
#[test]
fn test_declare_module_ts_format_1_b5b0447f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("declare module m {\n  class C {\n    field = \"field\";\n  }\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "declare module m {\n  class C {\n    field = \"field\";\n  }\n}"
    );
}
#[test]
fn test_declare_namespace_ts_format_1_f2e38ad9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("declare namespace m {\n  class C {\n    field = \"field\";\n  }\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "declare namespace m {\n  class C {\n    field = \"field\";\n  }\n}"
    );
}
#[test]
fn test_declare_var_ts_format_1_1d63c4a4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// tslint:disable-next-line:no-use-before-declare\nconst hello = 5;\n\n// tslint:disable-next-line:no-use-before-declare\ndeclare const hello2 = 5;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// tslint:disable-next-line:no-use-before-declare\nconst hello = 5;\n\n// tslint:disable-next-line:no-use-before-declare\ndeclare const hello2 = 5;");
}
#[test]
fn test_declare_get_set_field_ts_format_1_fafda7b8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("class C {\n  declare get: string\n  declare set: string;\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "class C {\n  declare get: string;\n  declare set: string;\n}"
    );
}
#[test]
fn test_object_type_in_declare_function_ts_format_1_59daf8de() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("declare function foo(this: { a: boolean, b: string, c: number }):\n  Promise<Array<foo>>\n\ndeclare function bazFlip({ a: boolean, b: string, c: number }):\n  Promise<Array<foo>>\n\ndeclare function bar(...{ a: boolean, b: string, c: number }):\n  Promise<Array<foo>>\n\ndeclare function bar(...x: { a: boolean, b: string, c: number }):\n  Promise<Array<foo>>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "declare function foo(this: {\n  a: boolean;\n  b: string;\n  c: number;\n}): Promise<Array<foo>>;\n\ndeclare function bazFlip({\n  a: boolean,\n  b: string,\n  c: number,\n}): Promise<Array<foo>>;\n\ndeclare function bar(\n  ...{ a: boolean, b: string, c: number }\n): Promise<Array<foo>>;\n\ndeclare function bar(\n  ...x: { a: boolean; b: string; c: number }\n): Promise<Array<foo>>;");
}
