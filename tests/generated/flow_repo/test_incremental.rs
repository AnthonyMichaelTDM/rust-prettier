#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_a_js_format_1_10f90113() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("/* @providesModule IncrModuleA */");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "/* @providesModule IncrModuleA */");
}
#[test]
fn test_b_js_format_1_d111e161() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("/* @providesModule IncrModuleB\n   @flow\n*/\n\nvar A = require('IncrModuleA');");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "/* @providesModule IncrModuleB\n   @flow\n*/\n\nvar A = require(\"IncrModuleA\");"
    );
}
#[test]
fn test_dup_a_js_format_1_a31e1841() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("/* @providesModule IncrModuleA */\n\nvar x:string = 0;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "/* @providesModule IncrModuleA */\n\nvar x: string = 0;"
    );
}
