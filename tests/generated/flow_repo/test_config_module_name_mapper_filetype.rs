#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_test_js_format_1_cebe22c7() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nimport {className} from \"./SomeCSSFile.css\";\nimport {doesntExist} from \"./SomeCSSFile.css\"; // Error: \\`doestExist\\` isn't an export") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\nimport { className } from \"./SomeCSSFile.css\";\nimport { doesntExist } from \"./SomeCSSFile.css\"; // Error: \\`doestExist\\` isn't an export");
}
