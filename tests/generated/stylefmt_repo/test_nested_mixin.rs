#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_nested_mixin_css_format_1_f3ca92cf() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("@mixin test() {\n  $values: blue red;\n  @each $val in $values {\n    color: $val;\n    @for $i from 2 through 10 {\n     background: $i;\n    }\n  }\n  @for $i from 2 through 10 {\n    color: $i;\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "@mixin test() {\n  $values: blue red;\n  @each $val in $values {\n    color: $val;\n    @for $i from 2 through 10 {\n      background: $i;\n    }\n  }\n  @for $i from 2 through 10 {\n    color: $i;\n  }\n}");
}
