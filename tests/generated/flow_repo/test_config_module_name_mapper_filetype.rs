#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_test_js_format_1_cebe22c7() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nimport {className} from \"./SomeCSSFile.css\";\nimport {doesntExist} from \"./SomeCSSFile.css\"; // Error: `doestExist` isn't an export") ? ;
    assert_eq ! (formatted , "// @flow\n\nimport { className } from \"./SomeCSSFile.css\";\nimport { doesntExist } from \"./SomeCSSFile.css\"; // Error: `doestExist` isn't an export");
    Ok(())
}
