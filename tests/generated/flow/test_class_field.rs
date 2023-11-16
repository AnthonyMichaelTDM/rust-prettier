#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_field_declare_modifier_js_format_1_de3f45dc() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer
        .format("class Foo {\n  declare foo: number;\n  declare static bar: string;\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "class Foo {\n  declare foo: number;\n  declare static bar: string;\n}"
    );
}
