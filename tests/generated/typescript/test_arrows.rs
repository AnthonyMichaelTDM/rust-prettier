#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_arrow_function_expression_ts_arrow_parensalways_format_1_f3e2773e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .arrow_parens("always")
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a = (b?) => c;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "a = (b?) => c;");
}
#[test]
fn test_arrow_function_expression_ts_arrow_parensavoid_format_1_f3e2773e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .arrow_parens("avoid")
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a = (b?) => c;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "a = (b?) => c;");
}
#[test]
fn test_short_body_ts_arrow_parensalways_format_1_0c383d7f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .arrow_parens("always")
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const initializeSnapshotState = (\n  testFile: Path,\n  update: boolean,\n  testPath: string,\n  expand: boolean,\n) => new SnapshotState(testFile, update, testPath, expand);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const initializeSnapshotState = (\n  testFile: Path,\n  update: boolean,\n  testPath: string,\n  expand: boolean,\n) => new SnapshotState(testFile, update, testPath, expand);");
}
#[test]
fn test_short_body_ts_arrow_parensavoid_format_1_0c383d7f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .arrow_parens("avoid")
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const initializeSnapshotState = (\n  testFile: Path,\n  update: boolean,\n  testPath: string,\n  expand: boolean,\n) => new SnapshotState(testFile, update, testPath, expand);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const initializeSnapshotState = (\n  testFile: Path,\n  update: boolean,\n  testPath: string,\n  expand: boolean,\n) => new SnapshotState(testFile, update, testPath, expand);");
}
#[test]
fn test_type_params_ts_arrow_parensalways_format_1_efdc5259() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .arrow_parens("always")
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<T>(a) => { }");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<T>(a) => {};");
}
#[test]
fn test_type_params_ts_arrow_parensavoid_format_1_efdc5259() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .arrow_parens("avoid")
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<T>(a) => { }");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<T>(a) => {};");
}
