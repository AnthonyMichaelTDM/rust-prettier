#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_arrow_function_js_format_1_c1e27de2() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const x = () => [].includes(true) || \"ontouchend\" in document\n\nconst y = () => [] in x") ? ;
    assert_eq ! (formatted , "const x = () => [].includes(true) || \"ontouchend\" in document;\n\nconst y = () => [] in x;");
    Ok(())
}
#[test]
fn test_arrow_function_invalid_js_format_1_15b3722b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("() => [] in x")?;
    assert_eq!(formatted, "() => [] in x;");
    Ok(())
}
