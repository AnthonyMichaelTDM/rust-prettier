use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_break_continue_js_format_1_0d34b19f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function foo() {\n    while(true) { break; }\n}\n\nfunction bar() {\n    L: do { continue L; } while(false)\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function foo() {\n  while (true) {\n    break;\n  }\n}\n\nfunction bar() {\n  L: do {\n    continue L;\n  } while (false);\n}");
}
#[test]
fn test_return_js_format_1_da42602d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function bar(x:number) { }\nfunction foo() {\n    var x = null;\n    if (x == null) return;\n    bar(x);\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function bar(x: number) {}\nfunction foo() {\n  var x = null;\n  if (x == null) return;\n  bar(x);\n}");
}
#[test]
fn test_toplevel_throw_js_format_1_39b6e8ed() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("// @flow\n\nthrow new Error('foo'); // no error");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "// @flow\n\nthrow new Error(\"foo\"); // no error"
    );
}
