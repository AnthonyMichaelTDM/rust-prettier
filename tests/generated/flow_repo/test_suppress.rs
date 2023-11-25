#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_a_js_format_1_a556c08a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// $FlowFixMe\nvar test1: string = 123; // This error should be suppressed\n\n// $FlowIssue\nvar test2: string = 123; // This error should be suppressed\n\nfunction getNum() {\n  return 123;\n}\n\n// $FlowFixMe This was the second loc in the error\nvar test3: string = getNum(); // This error should be suppressed\n\n// $FlowFixMe Error unused suppression\n\nvar test4: string = 123; // This error is NOT suppressed\n\n                         // $FlowFixMe Indentation shouldn't matter\nvar test5: string = 123; // This error should be suppressed\n\n/*\n * $FlowNewLine\n */\nvar test6: string = 123;") ? ;
    assert_eq ! (formatted , "// $FlowFixMe\nvar test1: string = 123; // This error should be suppressed\n\n// $FlowIssue\nvar test2: string = 123; // This error should be suppressed\n\nfunction getNum() {\n  return 123;\n}\n\n// $FlowFixMe This was the second loc in the error\nvar test3: string = getNum(); // This error should be suppressed\n\n// $FlowFixMe Error unused suppression\n\nvar test4: string = 123; // This error is NOT suppressed\n\n// $FlowFixMe Indentation shouldn't matter\nvar test5: string = 123; // This error should be suppressed\n\n/*\n * $FlowNewLine\n */\nvar test6: string = 123;");
    Ok(())
}
#[test]
fn test_b_js_format_1_5028902a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("// $FlowFixMe\nvar test1: string = library_num;")?;
    assert_eq!(formatted, "// $FlowFixMe\nvar test1: string = library_num;");
    Ok(())
}
#[test]
fn test_c_js_format_1_01f60f2e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function takesAString(x: string): void {}\n\nfunction runTest(y: number): void {\n  takesAString(\n    /* $FlowFixMe - suppressing the error op location should also work */\n    y,\n  );\n}") ? ;
    assert_eq ! (formatted , "function takesAString(x: string): void {}\n\nfunction runTest(y: number): void {\n  takesAString(\n    /* $FlowFixMe - suppressing the error op location should also work */\n    y,\n  );\n}");
    Ok(())
}
#[test]
fn test_d_js_format_1_2678d60b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("declare var x: {\n  x: { foo: string }\n};\ndeclare var y: {\n  // $FlowFixMe - this location is only mentioned in the extra section\n  x: { bar: number }\n};\nx = y;") ? ;
    assert_eq ! (formatted , "declare var x: {\n  x: { foo: string },\n};\ndeclare var y: {\n  // $FlowFixMe - this location is only mentioned in the extra section\n  x: { bar: number },\n};\nx = y;");
    Ok(())
}
#[test]
fn test_lib_js_format_1_4c91862e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("declare var library_num: number;")?;
    assert_eq!(formatted, "declare var library_num: number;");
    Ok(())
}
