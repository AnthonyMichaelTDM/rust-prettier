#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_return_outside_function_js_format_1_f3113d06() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("return someVeryLongStringA && someVeryLongStringB && someVeryLongStringC && someVeryLongStringD") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "return (\n  someVeryLongStringA &&\n  someVeryLongStringB &&\n  someVeryLongStringC &&\n  someVeryLongStringD\n);");
}
