#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_no_regression_js_format_1_2e652434() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("// Ensure no regression.\n\"use strict\";")?;
    assert_eq!(formatted, "// Ensure no regression.\n\"use strict\";");
    Ok(())
}
#[test]
fn test_use_strict_js_format_1_22c8297b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// Parentheses around expression statement should be preserved in this case.\n(\"use strict\");") ? ;
    assert_eq ! (formatted , "// Parentheses around expression statement should be preserved in this case.\n(\"use strict\");");
    Ok(())
}
