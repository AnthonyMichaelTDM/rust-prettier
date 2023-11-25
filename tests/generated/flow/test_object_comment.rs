#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_flow_object_comment_js_format_1_085c0144() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("export default (\n  {\n    foo,\n    bar\n  }: {\n    // comment\n    foo?: Object,\n    // comment 2\n    bar?: Object,\n  },\n) => {}") ? ;
    assert_eq ! (formatted , "export default ({\n  foo,\n  bar,\n}: {\n  // comment\n  foo?: Object,\n  // comment 2\n  bar?: Object,\n}) => {};");
    Ok(())
}
