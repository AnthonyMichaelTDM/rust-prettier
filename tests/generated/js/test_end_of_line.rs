#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_example_js_end_of_linecr_format_1_575aaa40() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .end_of_line("cr")
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("function f() {<LF>\n  console.log(\"testing line endings\");<LF>\n}<LF>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "function f() {<CR>\n  console.log(\"testing line endings\");<CR>\n}<CR>"
    );
}
#[test]
fn test_example_js_end_of_linecrlf_format_1_575aaa40() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .end_of_line("crlf")
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("function f() {<LF>\n  console.log(\"testing line endings\");<LF>\n}<LF>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "function f() {<CRLF>\n  console.log(\"testing line endings\");<CRLF>\n}<CRLF>"
    );
}
#[test]
fn test_example_js_end_of_linelf_format_1_575aaa40() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .end_of_line("lf")
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("function f() {<LF>\n  console.log(\"testing line endings\");<LF>\n}<LF>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "function f() {<LF>\n  console.log(\"testing line endings\");<LF>\n}<LF>"
    );
}
