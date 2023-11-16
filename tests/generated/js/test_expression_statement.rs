#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_no_regression_js_format_1_2e652434() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("// Ensure no regression.\n\"use strict\";");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "// Ensure no regression.\n\"use strict\";");
}
#[test]
fn test_use_strict_js_format_1_22c8297b() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("// Parentheses around expression statement should be preserved in this case.\n(\"use strict\");") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// Parentheses around expression statement should be preserved in this case.\n(\"use strict\");");
}
