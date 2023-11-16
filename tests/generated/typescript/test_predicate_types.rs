#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_predicate_types_ts_format_1_4853cf8f() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format(
        "class Foo {\n  isBar(): this is string {\n  }\n  isBaz = (): this is string => {\n  }\n}",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "class Foo {\n  isBar(): this is string {}\n  isBaz = (): this is string => {};\n}"
    );
}
