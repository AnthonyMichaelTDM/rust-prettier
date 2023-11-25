use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_export_js_format_1_623ebe68() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("/* @flow */\nexports.x = 1;\nexports.y = \"\";");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "/* @flow */\nexports.x = 1;\nexports.y = \"\";");
}
#[test]
fn test_import_js_format_1_8743504f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "/* @flow */\nvar e = require('./export');\nvar x: string = e.x;\nvar y: number = e.y;",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "/* @flow */\nvar e = require(\"./export\");\nvar x: string = e.x;\nvar y: number = e.y;"
    );
}
