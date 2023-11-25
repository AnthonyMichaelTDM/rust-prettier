#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_arrow_function_expression_ts_arrow_parensalways_format_1_f3e2773e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .arrow_parens("always")
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a = (b?) => c;")?;
    assert_eq!(formatted, "a = (b?) => c;");
    Ok(())
}
#[test]
fn test_arrow_function_expression_ts_arrow_parensavoid_format_1_f3e2773e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .arrow_parens("avoid")
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a = (b?) => c;")?;
    assert_eq!(formatted, "a = (b?) => c;");
    Ok(())
}
#[test]
fn test_short_body_ts_arrow_parensalways_format_1_0c383d7f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .arrow_parens("always")
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const initializeSnapshotState = (\n  testFile: Path,\n  update: boolean,\n  testPath: string,\n  expand: boolean,\n) => new SnapshotState(testFile, update, testPath, expand);") ? ;
    assert_eq ! (formatted , "const initializeSnapshotState = (\n  testFile: Path,\n  update: boolean,\n  testPath: string,\n  expand: boolean,\n) => new SnapshotState(testFile, update, testPath, expand);");
    Ok(())
}
#[test]
fn test_short_body_ts_arrow_parensavoid_format_1_0c383d7f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .arrow_parens("avoid")
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const initializeSnapshotState = (\n  testFile: Path,\n  update: boolean,\n  testPath: string,\n  expand: boolean,\n) => new SnapshotState(testFile, update, testPath, expand);") ? ;
    assert_eq ! (formatted , "const initializeSnapshotState = (\n  testFile: Path,\n  update: boolean,\n  testPath: string,\n  expand: boolean,\n) => new SnapshotState(testFile, update, testPath, expand);");
    Ok(())
}
#[test]
fn test_type_params_ts_arrow_parensalways_format_1_efdc5259() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .arrow_parens("always")
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<T>(a) => { }")?;
    assert_eq!(formatted, "<T>(a) => {};");
    Ok(())
}
#[test]
fn test_type_params_ts_arrow_parensavoid_format_1_efdc5259() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .arrow_parens("avoid")
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<T>(a) => { }")?;
    assert_eq!(formatted, "<T>(a) => {};");
    Ok(())
}
