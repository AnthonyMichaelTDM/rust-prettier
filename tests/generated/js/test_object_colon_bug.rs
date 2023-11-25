#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_bug_js_print_width_35_format_1_d3856951() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(35)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("const foo = {\n  bar: props.bar ? props.bar : noop,\n  baz: props.baz\n}")?;
    assert_eq!(
        formatted,
        "const foo = {\n  bar: props.bar\n    ? props.bar\n    : noop,\n  baz: props.baz,\n};"
    );
    Ok(())
}
