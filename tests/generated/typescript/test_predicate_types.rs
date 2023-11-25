#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_predicate_types_ts_format_1_4853cf8f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "class Foo {\n  isBar(): this is string {\n  }\n  isBaz = (): this is string => {\n  }\n}",
    )?;
    assert_eq!(
        formatted,
        "class Foo {\n  isBar(): this is string {}\n  isBaz = (): this is string => {};\n}"
    );
    Ok(())
}
