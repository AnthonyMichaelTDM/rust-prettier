#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_basic_ts_format_1_5f8d059e() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("// basic\nconst foo = bar<T>;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "// basic\nconst foo = bar<T>;");
}
#[test]
fn test_binary_expr_ts_format_1_7fd1e105() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("new A < B >\nC");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "new A<B>();\nC;");
}
#[test]
fn test_inferface_asi_ts_format_1_6b596118() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer
        .format("interface Example {\n  (a: number): typeof a\n      \n  <T>(): void\n};");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "interface Example {\n  (a: number): typeof a;\n\n  <T>(): void;\n}"
    );
}
#[test]
fn test_logical_expr_ts_format_1_482e2ad6() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("export class Foo<T> {\n  message: string;\n}\n\nfunction sample(error: unknown) {\n  if (!(error instanceof Foo<'some-type'> || error instanceof Error) || !error.message) {\n    return 'something';\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export class Foo<T> {\n  message: string;\n}\n\nfunction sample(error: unknown) {\n  if (\n    !(error instanceof Foo<\"some-type\"> || error instanceof Error) ||\n    !error.message\n  ) {\n    return \"something\";\n  }\n}");
}
#[test]
fn test_new_ts_format_1_aca772f7() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("// new\nnew A<T>;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "// new\nnew A<T>();");
}
#[test]
fn test_property_access_ts_format_1_f218477e() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("(Array<string>).name;\n(fn1<string>).bind(obj);\n(fn2<string, number>).bind(obj);\na[(Array<string>)];\na[(Array<string>).name];\n(Array<string>).a;\n(Array<string>)?.a;\n(Array<string>)[a];\n(Array<string>)?.[a];\n(Array<string>)[\"a\"];\n(Array<string>)?.[\"a\"];\n(Array<string>)[\\`a\\`];\n(Array<string>)?.[\\`a\\`];\n(Array<string>)[(Array<string>)];") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "(Array<string>).name;\n(fn1<string>).bind(obj);\n(fn2<string, number>).bind(obj);\na[Array<string>];\na[(Array<string>).name];\n(Array<string>).a;\n(Array<string>)?.a;\n(Array<string>)[a];\n(Array<string>)?.[a];\n(Array<string>)[\"a\"];\n(Array<string>)?.[\"a\"];\n(Array<string>)[\\`a\\`];\n(Array<string>)?.[\\`a\\`];\n(Array<string>)[Array<string>];");
}
#[test]
fn test_typeof_ts_format_1_9df6bc3c() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("let x: typeof y.z<w>;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "let x: typeof y.z<w>;");
}
