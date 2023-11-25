#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_combinator_css_format_1_5d88cca7() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("-Option/root .public/section ~ .public/section:before {\n}\n\n.x .y {}\n.x > .y {}\n.x ~ .y {}\n.x + .y {}\n.x.y {}\n.x     .y {}\n.x\n    .y {}") ? ;
    assert_eq ! (formatted , "-Option/root .public/section ~ .public/section:before {\n}\n\n.x .y {\n}\n.x > .y {\n}\n.x ~ .y {\n}\n.x + .y {\n}\n.x.y {\n}\n.x .y {\n}\n.x .y {\n}");
    Ok(())
}
#[test]
fn test_leading_css_format_1_e1ce6bce() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a {\n  > * {\n  }\n  &, > * {\n  }\n  + * {\n  }\n}")?;
    assert_eq!(
        formatted,
        "a {\n  > * {\n  }\n  &,\n  > * {\n  }\n  + * {\n  }\n}"
    );
    Ok(())
}
