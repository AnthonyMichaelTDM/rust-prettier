#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_test_js_format_1_cebe22c7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nimport {className} from \"./SomeCSSFile.css\";\nimport {doesntExist} from \"./SomeCSSFile.css\"; // Error: \\`doestExist\\` isn't an export") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\nimport { className } from \"./SomeCSSFile.css\";\nimport { doesntExist } from \"./SomeCSSFile.css\"; // Error: \\`doestExist\\` isn't an export");
}
