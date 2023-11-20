#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_indent_js_format_1_f3102197() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["babel", "flow", "typescript"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("if (someVeryLongStringA && someVeryLongStringB && someVeryLongStringC && someVeryLongStringD) {}\nwhile (someVeryLongStringA && someVeryLongStringB && someVeryLongStringC && someVeryLongStringD) {}\ndo {}\nwhile (someVeryLongStringA && someVeryLongStringB && someVeryLongStringC && someVeryLongStringD);\n\nif (someVeryLongFunc(someVeryLongArgA, someVeryLongArgB, someVeryLongArgC, someVeryLongArgD)) {}\nwhile (someVeryLongFunc(someVeryLongArgA, someVeryLongArgB, someVeryLongArgC, someVeryLongArgD)) {}\ndo {}\nwhile (someVeryLongFunc(someVeryLongArgA, someVeryLongArgB, someVeryLongArgC, someVeryLongArgD));\n\nwhile (0) 1;\n\ndo 1;\nwhile (0);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "if (\n  someVeryLongStringA &&\n  someVeryLongStringB &&\n  someVeryLongStringC &&\n  someVeryLongStringD\n) {\n}\nwhile (\n  someVeryLongStringA &&\n  someVeryLongStringB &&\n  someVeryLongStringC &&\n  someVeryLongStringD\n) {}\ndo {} while (\n  someVeryLongStringA &&\n  someVeryLongStringB &&\n  someVeryLongStringC &&\n  someVeryLongStringD\n);\n\nif (\n  someVeryLongFunc(\n    someVeryLongArgA,\n    someVeryLongArgB,\n    someVeryLongArgC,\n    someVeryLongArgD,\n  )\n) {\n}\nwhile (\n  someVeryLongFunc(\n    someVeryLongArgA,\n    someVeryLongArgB,\n    someVeryLongArgC,\n    someVeryLongArgD,\n  )\n) {}\ndo {} while (\n  someVeryLongFunc(\n    someVeryLongArgA,\n    someVeryLongArgB,\n    someVeryLongArgC,\n    someVeryLongArgD,\n  )\n);\n\nwhile (0) 1;\n\ndo 1;\nwhile (0);");
}
