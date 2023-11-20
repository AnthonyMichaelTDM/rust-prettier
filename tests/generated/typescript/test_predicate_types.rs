#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_predicate_types_ts_format_1_4853cf8f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
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
