#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_cursor_1_js_end_of_lineauto_format_1_c6ae08c4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(26)
        .end_of_line("auto")
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("(function(){return        <|>15})()<LF>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "(function () {<LF>\n  return <|>15;<LF>\n})();<LF>"
    );
}
#[test]
fn test_cursor_1_js_end_of_linecr_format_1_c6ae08c4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(26)
        .end_of_line("cr")
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("(function(){return        <|>15})()<LF>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "(function () {<CR>\n  return <|>15;<CR>\n})();<CR>"
    );
}
#[test]
fn test_cursor_1_js_end_of_linecrlf_format_1_c6ae08c4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(26)
        .end_of_line("crlf")
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("(function(){return        <|>15})()<LF>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "(function () {<CRLF>\n  return <|>15;<CRLF>\n})();<CRLF>"
    );
}
#[test]
fn test_cursor_1_js_end_of_linelf_format_1_c6ae08c4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(26)
        .end_of_line("lf")
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("(function(){return        <|>15})()<LF>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "(function () {<LF>\n  return <|>15;<LF>\n})();<LF>"
    );
}
#[test]
fn test_cursor_and_range_js_end_of_lineauto_format_1_82619bf8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(21)
        .end_of_line("auto")
        .parser("js")
        .print_width(80)
        .range_end(26)
        .range_start(10)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1 |<LF>\n  2 |<LF>\n> 3 | class    a {<LF>\n    |         ^^^^<LF>\n> 4 |   b(  <|> ) {}<LF>\n    | ^^^^^^^^^^^^^^<LF>\n  5 | }<LF>\n  6 |<LF>\n  7 | let    ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "<LF>\n<LF>\nclass a {<LF>\n  b(<|>) {}<LF>\n}<LF>\n<LF>\nlet    "
    );
}
#[test]
fn test_cursor_and_range_js_end_of_linecr_format_1_82619bf8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(21)
        .end_of_line("cr")
        .parser("js")
        .print_width(80)
        .range_end(26)
        .range_start(10)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1 |<LF>\n  2 |<LF>\n> 3 | class    a {<LF>\n    |         ^^^^<LF>\n> 4 |   b(  <|> ) {}<LF>\n    | ^^^^^^^^^^^^^^<LF>\n  5 | }<LF>\n  6 |<LF>\n  7 | let    ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "<CR>\n<CR>\nclass a {<CR>\n  b(<|>) {}<CR>\n}<CR>\n<CR>\nlet    "
    );
}
#[test]
fn test_cursor_and_range_js_end_of_linecrlf_format_1_82619bf8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(21)
        .end_of_line("crlf")
        .parser("js")
        .print_width(80)
        .range_end(26)
        .range_start(10)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1 |<LF>\n  2 |<LF>\n> 3 | class    a {<LF>\n    |         ^^^^<LF>\n> 4 |   b(  <|> ) {}<LF>\n    | ^^^^^^^^^^^^^^<LF>\n  5 | }<LF>\n  6 |<LF>\n  7 | let    ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "<CRLF>\n<CRLF>\nclass a {<CRLF>\n  b(<|>) {}<CRLF>\n}<CRLF>\n<CRLF>\nlet    "
    );
}
#[test]
fn test_cursor_and_range_js_end_of_linelf_format_1_82619bf8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(21)
        .end_of_line("lf")
        .parser("js")
        .print_width(80)
        .range_end(26)
        .range_start(10)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1 |<LF>\n  2 |<LF>\n> 3 | class    a {<LF>\n    |         ^^^^<LF>\n> 4 |   b(  <|> ) {}<LF>\n    | ^^^^^^^^^^^^^^<LF>\n  5 | }<LF>\n  6 |<LF>\n  7 | let    ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "<LF>\n<LF>\nclass a {<LF>\n  b(<|>) {}<LF>\n}<LF>\n<LF>\nlet    "
    );
}
#[test]
fn test_range_1_js_end_of_lineauto_format_1_4b4219bc() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .end_of_line("auto")
        .parser("js")
        .print_width(80)
        .range_end(26)
        .range_start(10)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1 |<LF>\n  2 |<LF>\n> 3 | class    a {<LF>\n    |         ^^^^<LF>\n> 4 |   b(   ) {}<LF>\n    | ^^^^^^^^^^^<LF>\n  5 | }<LF>\n  6 |<LF>\n  7 | let    x<LF>\n  8 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "<LF>\n<LF>\nclass a {<LF>\n  b() {}<LF>\n}<LF>\n<LF>\nlet    x<LF>"
    );
}
#[test]
fn test_range_1_js_end_of_linecr_format_1_4b4219bc() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .end_of_line("cr")
        .parser("js")
        .print_width(80)
        .range_end(26)
        .range_start(10)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1 |<LF>\n  2 |<LF>\n> 3 | class    a {<LF>\n    |         ^^^^<LF>\n> 4 |   b(   ) {}<LF>\n    | ^^^^^^^^^^^<LF>\n  5 | }<LF>\n  6 |<LF>\n  7 | let    x<LF>\n  8 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "<CR>\n<CR>\nclass a {<CR>\n  b() {}<CR>\n}<CR>\n<CR>\nlet    x<CR>"
    );
}
#[test]
fn test_range_1_js_end_of_linecrlf_format_1_4b4219bc() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .end_of_line("crlf")
        .parser("js")
        .print_width(80)
        .range_end(26)
        .range_start(10)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1 |<LF>\n  2 |<LF>\n> 3 | class    a {<LF>\n    |         ^^^^<LF>\n> 4 |   b(   ) {}<LF>\n    | ^^^^^^^^^^^<LF>\n  5 | }<LF>\n  6 |<LF>\n  7 | let    x<LF>\n  8 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "<CRLF>\n<CRLF>\nclass a {<CRLF>\n  b() {}<CRLF>\n}<CRLF>\n<CRLF>\nlet    x<CRLF>"
    );
}
#[test]
fn test_range_1_js_end_of_linelf_format_1_4b4219bc() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .end_of_line("lf")
        .parser("js")
        .print_width(80)
        .range_end(26)
        .range_start(10)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1 |<LF>\n  2 |<LF>\n> 3 | class    a {<LF>\n    |         ^^^^<LF>\n> 4 |   b(   ) {}<LF>\n    | ^^^^^^^^^^^<LF>\n  5 | }<LF>\n  6 |<LF>\n  7 | let    x<LF>\n  8 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "<LF>\n<LF>\nclass a {<LF>\n  b() {}<LF>\n}<LF>\n<LF>\nlet    x<LF>"
    );
}
#[test]
fn test_range_and_cursor_1_js_end_of_lineauto_format_1_de8ed09b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(22)
        .end_of_line("auto")
        .parser("js")
        .print_width(80)
        .range_end(27)
        .range_start(11)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1 |<LF>\n  2 |<LF>\n  3 |<LF>\n> 4 | class    a {<LF>\n    |         ^^^^<LF>\n> 5 |   b(  <|> ) {}<LF>\n    | ^^^^^^^^^^^^^^<LF>\n  6 | }<LF>\n  7 |<LF>\n  8 | let    x <LF>\n  9 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "<LF>\n<LF>\n<LF>\nclass a {<LF>\n  b(<|>) {}<LF>\n}<LF>\n<LF>\nlet    x <LF>"
    );
}
#[test]
fn test_range_and_cursor_1_js_end_of_linecr_format_1_de8ed09b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(22)
        .end_of_line("cr")
        .parser("js")
        .print_width(80)
        .range_end(27)
        .range_start(11)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1 |<LF>\n  2 |<LF>\n  3 |<LF>\n> 4 | class    a {<LF>\n    |         ^^^^<LF>\n> 5 |   b(  <|> ) {}<LF>\n    | ^^^^^^^^^^^^^^<LF>\n  6 | }<LF>\n  7 |<LF>\n  8 | let    x <LF>\n  9 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "<CR>\n<CR>\n<CR>\nclass a {<CR>\n  b(<|>) {}<CR>\n}<CR>\n<CR>\nlet    x <CR>"
    );
}
#[test]
fn test_range_and_cursor_1_js_end_of_linecrlf_format_1_de8ed09b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(22)
        .end_of_line("crlf")
        .parser("js")
        .print_width(80)
        .range_end(27)
        .range_start(11)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1 |<LF>\n  2 |<LF>\n  3 |<LF>\n> 4 | class    a {<LF>\n    |         ^^^^<LF>\n> 5 |   b(  <|> ) {}<LF>\n    | ^^^^^^^^^^^^^^<LF>\n  6 | }<LF>\n  7 |<LF>\n  8 | let    x <LF>\n  9 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<CRLF>\n<CRLF>\n<CRLF>\nclass a {<CRLF>\n  b(<|>) {}<CRLF>\n}<CRLF>\n<CRLF>\nlet    x <CRLF>");
}
#[test]
fn test_range_and_cursor_1_js_end_of_linelf_format_1_de8ed09b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(22)
        .end_of_line("lf")
        .parser("js")
        .print_width(80)
        .range_end(27)
        .range_start(11)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1 |<LF>\n  2 |<LF>\n  3 |<LF>\n> 4 | class    a {<LF>\n    |         ^^^^<LF>\n> 5 |   b(  <|> ) {}<LF>\n    | ^^^^^^^^^^^^^^<LF>\n  6 | }<LF>\n  7 |<LF>\n  8 | let    x <LF>\n  9 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "<LF>\n<LF>\n<LF>\nclass a {<LF>\n  b(<|>) {}<LF>\n}<LF>\n<LF>\nlet    x <LF>"
    );
}
