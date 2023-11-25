#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_destructuring_ts_format_1_91e4ed7c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("({ foo = [] } = bar);\n\nfunction f({ x }?) {}\nfunction g([ x ]?) {}")?;
    assert_eq!(
        formatted,
        "({ foo = [] } = bar);\n\nfunction f({ x }?) {}\nfunction g([x]?) {}"
    );
    Ok(())
}
