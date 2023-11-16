#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_snippet_cursor_1_js_format_1_8999657b() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("\u{feff}(function(){return        <|>15})()");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\u{feff}(function () {\n  return <|>15;\n})();");
}
#[test]
fn test_snippet_cursor_and_range_js_format_1_713e2d9e() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("  1 | \u{feff}\n  2 |\n> 3 | class    a {\n    |         ^^^^\n> 4 |   b(  <|> ) {}\n    | ^^^^^^^^^^^^^^\n  5 | }\n  6 |\n  7 | let    ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "\u{feff}\n\nclass a {\n  b(<|>) {}\n}\n\nlet    "
    );
}
#[test]
fn test_snippet_range_1_js_format_1_f19ae2ee() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("  1 | \u{feff}\n  2 |\n> 3 | class    a {\n    |         ^^^^\n> 4 |   b(   ) {}\n    | ^^^^^^^^^^^\n  5 | }\n  6 |\n  7 | let    x\n  8 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\u{feff}\n\nclass a {\n  b() {}\n}\n\nlet    x");
}
#[test]
fn test_snippet_range_and_cursor_1_js_format_1_1e50091b() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("  1 | \u{feff}\n  2 |\n  3 |\n> 4 | class    a {\n    |         ^^^^\n> 5 |   b(  <|> ) {}\n    | ^^^^^^^^^^^^^^\n  6 | }\n  7 |\n  8 | let    x \n  9 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "\u{feff}\n\n\nclass a {\n  b(<|>) {}\n}\n\nlet    x "
    );
}
