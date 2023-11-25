#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
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
    let formatted =
        pretty_printer.format("function f() {\n  console.log(\"testing line endings\");\n}\n");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "function f() {\r  console.log(\"testing line endings\");\r}\r"
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
    let formatted =
        pretty_printer.format("function f() {\n  console.log(\"testing line endings\");\n}\n");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "function f() {\r\n  console.log(\"testing line endings\");\r\n}\r\n"
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
    let formatted =
        pretty_printer.format("function f() {\n  console.log(\"testing line endings\");\n}\n");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "function f() {\n  console.log(\"testing line endings\");\n}\n"
    );
}
