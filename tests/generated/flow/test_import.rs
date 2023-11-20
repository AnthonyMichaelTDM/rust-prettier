#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_import_as_as_js_format_1_da07b00e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\nimport { foo as as } from \"foo\";\nimport { as as as } from \"foo\";\nimport { as as foo } from \"foo\";") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\nimport { foo as as } from \"foo\";\nimport { as as as } from \"foo\";\nimport { as as foo } from \"foo\";");
}
#[test]
fn test_type_import_as_as_js_format_1_e7c4881a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("import {type foo as as} from \"foo\";");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "import { type foo as as } from \"foo\";");
}
