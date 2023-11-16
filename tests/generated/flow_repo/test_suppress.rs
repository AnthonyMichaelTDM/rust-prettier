#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_a_js_format_1_a556c08a() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("// $FlowFixMe\nvar test1: string = 123; // This error should be suppressed\n\n// $FlowIssue\nvar test2: string = 123; // This error should be suppressed\n\nfunction getNum() {\n  return 123;\n}\n\n// $FlowFixMe This was the second loc in the error\nvar test3: string = getNum(); // This error should be suppressed\n\n// $FlowFixMe Error unused suppression\n\nvar test4: string = 123; // This error is NOT suppressed\n\n                         // $FlowFixMe Indentation shouldn't matter\nvar test5: string = 123; // This error should be suppressed\n\n/*\n * $FlowNewLine\n */\nvar test6: string = 123;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// $FlowFixMe\nvar test1: string = 123; // This error should be suppressed\n\n// $FlowIssue\nvar test2: string = 123; // This error should be suppressed\n\nfunction getNum() {\n  return 123;\n}\n\n// $FlowFixMe This was the second loc in the error\nvar test3: string = getNum(); // This error should be suppressed\n\n// $FlowFixMe Error unused suppression\n\nvar test4: string = 123; // This error is NOT suppressed\n\n// $FlowFixMe Indentation shouldn't matter\nvar test5: string = 123; // This error should be suppressed\n\n/*\n * $FlowNewLine\n */\nvar test6: string = 123;");
}
#[test]
fn test_b_js_format_1_5028902a() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("// $FlowFixMe\nvar test1: string = library_num;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "// $FlowFixMe\nvar test1: string = library_num;");
}
#[test]
fn test_c_js_format_1_01f60f2e() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("function takesAString(x: string): void {}\n\nfunction runTest(y: number): void {\n  takesAString(\n    /* $FlowFixMe - suppressing the error op location should also work */\n    y,\n  );\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function takesAString(x: string): void {}\n\nfunction runTest(y: number): void {\n  takesAString(\n    /* $FlowFixMe - suppressing the error op location should also work */\n    y,\n  );\n}");
}
#[test]
fn test_d_js_format_1_2678d60b() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("declare var x: {\n  x: { foo: string }\n};\ndeclare var y: {\n  // $FlowFixMe - this location is only mentioned in the extra section\n  x: { bar: number }\n};\nx = y;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "declare var x: {\n  x: { foo: string },\n};\ndeclare var y: {\n  // $FlowFixMe - this location is only mentioned in the extra section\n  x: { bar: number },\n};\nx = y;");
}
#[test]
fn test_lib_js_format_1_4c91862e() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("declare var library_num: number;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "declare var library_num: number;");
}
