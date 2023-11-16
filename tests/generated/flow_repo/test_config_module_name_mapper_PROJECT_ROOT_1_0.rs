#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_main_js_format_1_83b791e5() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nimport {test} from 'testmodule';\n\nvar a: number = test;\nvar b: string = test; // Error: number ~> string") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\nimport { test } from \"testmodule\";\n\nvar a: number = test;\nvar b: string = test; // Error: number ~> string");
}
