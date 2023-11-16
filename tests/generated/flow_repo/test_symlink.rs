#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_bar_js_format_1_602046af() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("export type Foo = { x: number; }");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "export type Foo = { x: number };");
}
#[test]
fn test_foo_js_format_1_602046af() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("export type Foo = { x: number; }");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "export type Foo = { x: number };");
}
#[test]
fn test_qux_js_format_1_2ee67941() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted =
        pretty_printer.format("import type { Foo } from './bar.js';\n({ x: \"\" }: Foo);");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "import type { Foo } from \"./bar.js\";\n({ x: \"\" }: Foo);"
    );
}
