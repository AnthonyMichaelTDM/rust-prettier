#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_typeof_with_type_arguments_js_format_1_6c3b6495() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("type Foo = typeof MyGenericClass<string, number>;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "type Foo = typeof MyGenericClass<string, number>;"
    );
}
