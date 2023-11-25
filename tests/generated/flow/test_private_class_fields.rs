#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_quotes_js_quote_propsconsistent_format_1_075ff22b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .quote_props("consistent")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("class Foo\n{\na:a;\n  'q-w': ee;\n#privateProp: number;}")?;
    assert_eq!(
        formatted,
        "class Foo {\n  \"a\": a;\n  \"q-w\": ee;\n  #privateProp: number;\n}"
    );
    Ok(())
}
