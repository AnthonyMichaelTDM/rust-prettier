#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_nested_mixin_css_format_1_f3ca92cf() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("@mixin test() {\n  $values: blue red;\n  @each $val in $values {\n    color: $val;\n    @for $i from 2 through 10 {\n     background: $i;\n    }\n  }\n  @for $i from 2 through 10 {\n    color: $i;\n  }\n}") ? ;
    assert_eq ! (formatted , "@mixin test() {\n  $values: blue red;\n  @each $val in $values {\n    color: $val;\n    @for $i from 2 through 10 {\n      background: $i;\n    }\n  }\n  @for $i from 2 through 10 {\n    color: $i;\n  }\n}");
    Ok(())
}
