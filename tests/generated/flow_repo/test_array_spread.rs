#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_test_js_format_1_9fdf3595() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var A = [1,2,3];\nvar B = [...A];\nvar C = [1,2,3];\nB.sort((a, b) => a - b);\nC.sort((a, b) => a - b);\n\nvar x: Array<string> = ['1', '2'];\nvar y: Array<string> = ['3', ...x];") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var A = [1, 2, 3];\nvar B = [...A];\nvar C = [1, 2, 3];\nB.sort((a, b) => a - b);\nC.sort((a, b) => a - b);\n\nvar x: Array<string> = [\"1\", \"2\"];\nvar y: Array<string> = [\"3\", ...x];");
}
