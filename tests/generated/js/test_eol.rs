#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_cursor_1_js_end_of_lineauto_format_1_c6ae08c4() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(26)
        .end_of_line("auto")
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("(function(){return        <|>15})()\n")?;
    assert_eq!(formatted, "(function () {\n  return <|>15;\n})();\n");
    Ok(())
}
#[test]
fn test_cursor_1_js_end_of_linecr_format_1_c6ae08c4() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(26)
        .end_of_line("cr")
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("(function(){return        <|>15})()\n")?;
    assert_eq!(formatted, "(function () {\r  return <|>15;\r})();\r");
    Ok(())
}
#[test]
fn test_cursor_1_js_end_of_linecrlf_format_1_c6ae08c4() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(26)
        .end_of_line("crlf")
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("(function(){return        <|>15})()\n")?;
    assert_eq!(formatted, "(function () {\r\n  return <|>15;\r\n})();\r\n");
    Ok(())
}
#[test]
fn test_cursor_1_js_end_of_linelf_format_1_c6ae08c4() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(26)
        .end_of_line("lf")
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("(function(){return        <|>15})()\n")?;
    assert_eq!(formatted, "(function () {\n  return <|>15;\n})();\n");
    Ok(())
}
#[test]
fn test_cursor_and_range_js_end_of_lineauto_format_1_82619bf8() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(21)
        .end_of_line("auto")
        .parser("js")
        .print_width(80)
        .range_end(26)
        .range_start(10)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1 |\n  2 |\n> 3 | class    a {\n    |         ^^^^\n> 4 |   b(  <|> ) {}\n    | ^^^^^^^^^^^^^^\n  5 | }\n  6 |\n  7 | let    ") ? ;
    assert_eq!(formatted, "\n\nclass a {\n  b(<|>) {}\n}\n\nlet    ");
    Ok(())
}
#[test]
fn test_cursor_and_range_js_end_of_linecr_format_1_82619bf8() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(21)
        .end_of_line("cr")
        .parser("js")
        .print_width(80)
        .range_end(26)
        .range_start(10)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1 |\n  2 |\n> 3 | class    a {\n    |         ^^^^\n> 4 |   b(  <|> ) {}\n    | ^^^^^^^^^^^^^^\n  5 | }\n  6 |\n  7 | let    ") ? ;
    assert_eq!(formatted, "\r\rclass a {\r  b(<|>) {}\r}\r\rlet    ");
    Ok(())
}
#[test]
fn test_cursor_and_range_js_end_of_linecrlf_format_1_82619bf8() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(21)
        .end_of_line("crlf")
        .parser("js")
        .print_width(80)
        .range_end(26)
        .range_start(10)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1 |\n  2 |\n> 3 | class    a {\n    |         ^^^^\n> 4 |   b(  <|> ) {}\n    | ^^^^^^^^^^^^^^\n  5 | }\n  6 |\n  7 | let    ") ? ;
    assert_eq!(
        formatted,
        "\r\n\r\nclass a {\r\n  b(<|>) {}\r\n}\r\n\r\nlet    "
    );
    Ok(())
}
#[test]
fn test_cursor_and_range_js_end_of_linelf_format_1_82619bf8() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(21)
        .end_of_line("lf")
        .parser("js")
        .print_width(80)
        .range_end(26)
        .range_start(10)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1 |\n  2 |\n> 3 | class    a {\n    |         ^^^^\n> 4 |   b(  <|> ) {}\n    | ^^^^^^^^^^^^^^\n  5 | }\n  6 |\n  7 | let    ") ? ;
    assert_eq!(formatted, "\n\nclass a {\n  b(<|>) {}\n}\n\nlet    ");
    Ok(())
}
#[test]
fn test_range_1_js_end_of_lineauto_format_1_4b4219bc() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .end_of_line("auto")
        .parser("js")
        .print_width(80)
        .range_end(26)
        .range_start(10)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1 |\n  2 |\n> 3 | class    a {\n    |         ^^^^\n> 4 |   b(   ) {}\n    | ^^^^^^^^^^^\n  5 | }\n  6 |\n  7 | let    x\n  8 ") ? ;
    assert_eq!(formatted, "\n\nclass a {\n  b() {}\n}\n\nlet    x\n");
    Ok(())
}
#[test]
fn test_range_1_js_end_of_linecr_format_1_4b4219bc() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .end_of_line("cr")
        .parser("js")
        .print_width(80)
        .range_end(26)
        .range_start(10)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1 |\n  2 |\n> 3 | class    a {\n    |         ^^^^\n> 4 |   b(   ) {}\n    | ^^^^^^^^^^^\n  5 | }\n  6 |\n  7 | let    x\n  8 ") ? ;
    assert_eq!(formatted, "\r\rclass a {\r  b() {}\r}\r\rlet    x\r");
    Ok(())
}
#[test]
fn test_range_1_js_end_of_linecrlf_format_1_4b4219bc() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .end_of_line("crlf")
        .parser("js")
        .print_width(80)
        .range_end(26)
        .range_start(10)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1 |\n  2 |\n> 3 | class    a {\n    |         ^^^^\n> 4 |   b(   ) {}\n    | ^^^^^^^^^^^\n  5 | }\n  6 |\n  7 | let    x\n  8 ") ? ;
    assert_eq!(
        formatted,
        "\r\n\r\nclass a {\r\n  b() {}\r\n}\r\n\r\nlet    x\r\n"
    );
    Ok(())
}
#[test]
fn test_range_1_js_end_of_linelf_format_1_4b4219bc() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .end_of_line("lf")
        .parser("js")
        .print_width(80)
        .range_end(26)
        .range_start(10)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1 |\n  2 |\n> 3 | class    a {\n    |         ^^^^\n> 4 |   b(   ) {}\n    | ^^^^^^^^^^^\n  5 | }\n  6 |\n  7 | let    x\n  8 ") ? ;
    assert_eq!(formatted, "\n\nclass a {\n  b() {}\n}\n\nlet    x\n");
    Ok(())
}
#[test]
fn test_range_and_cursor_1_js_end_of_lineauto_format_1_de8ed09b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(22)
        .end_of_line("auto")
        .parser("js")
        .print_width(80)
        .range_end(27)
        .range_start(11)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1 |\n  2 |\n  3 |\n> 4 | class    a {\n    |         ^^^^\n> 5 |   b(  <|> ) {}\n    | ^^^^^^^^^^^^^^\n  6 | }\n  7 |\n  8 | let    x \n  9 ") ? ;
    assert_eq!(formatted, "\n\n\nclass a {\n  b(<|>) {}\n}\n\nlet    x \n");
    Ok(())
}
#[test]
fn test_range_and_cursor_1_js_end_of_linecr_format_1_de8ed09b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(22)
        .end_of_line("cr")
        .parser("js")
        .print_width(80)
        .range_end(27)
        .range_start(11)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1 |\n  2 |\n  3 |\n> 4 | class    a {\n    |         ^^^^\n> 5 |   b(  <|> ) {}\n    | ^^^^^^^^^^^^^^\n  6 | }\n  7 |\n  8 | let    x \n  9 ") ? ;
    assert_eq!(formatted, "\r\r\rclass a {\r  b(<|>) {}\r}\r\rlet    x \r");
    Ok(())
}
#[test]
fn test_range_and_cursor_1_js_end_of_linecrlf_format_1_de8ed09b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(22)
        .end_of_line("crlf")
        .parser("js")
        .print_width(80)
        .range_end(27)
        .range_start(11)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1 |\n  2 |\n  3 |\n> 4 | class    a {\n    |         ^^^^\n> 5 |   b(  <|> ) {}\n    | ^^^^^^^^^^^^^^\n  6 | }\n  7 |\n  8 | let    x \n  9 ") ? ;
    assert_eq!(
        formatted,
        "\r\n\r\n\r\nclass a {\r\n  b(<|>) {}\r\n}\r\n\r\nlet    x \r\n"
    );
    Ok(())
}
#[test]
fn test_range_and_cursor_1_js_end_of_linelf_format_1_de8ed09b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(22)
        .end_of_line("lf")
        .parser("js")
        .print_width(80)
        .range_end(27)
        .range_start(11)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1 |\n  2 |\n  3 |\n> 4 | class    a {\n    |         ^^^^\n> 5 |   b(  <|> ) {}\n    | ^^^^^^^^^^^^^^\n  6 | }\n  7 |\n  8 | let    x \n  9 ") ? ;
    assert_eq!(formatted, "\n\n\nclass a {\n  b(<|>) {}\n}\n\nlet    x \n");
    Ok(())
}
