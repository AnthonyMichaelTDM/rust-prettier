#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_import_require_ts_format_1_97b01819() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("import { IModel } from 'vs/editor/common/editorCommon';\nimport JSONContributionRegistry = require('vs/platform/jsonschemas/common/jsonContributionRegistry');") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "import { IModel } from \"vs/editor/common/editorCommon\";\nimport JSONContributionRegistry = require(\"vs/platform/jsonschemas/common/jsonContributionRegistry\");");
}
#[test]
fn test_type_imports_ts_format_1_99d8498b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("import type A = require(\"foo\");\nexport import type = require(\"A\");\n\nimport type\nA = require(\"A\");\n\nimport\ntype\na = require(\"a\");\n\nexport import\ntype\nB = require(\"B\");\n\nexport\nimport\ntype\nb = require(\"b\");") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "import type A = require(\"foo\");\nexport import type = require(\"A\");\n\nimport type A = require(\"A\");\n\nimport type a = require(\"a\");\n\nexport import type B = require(\"B\");\n\nexport import type b = require(\"b\");");
}
