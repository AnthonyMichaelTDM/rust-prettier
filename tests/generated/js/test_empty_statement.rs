#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_body_js_format_1_dd686c89() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["babel", "flow", "typescript"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("with (a);\nif (1); else if (2); else;\nfor (;;);\nwhile (1);\nfor (var i in o);\nfor (var i of o);\ndo; while(1);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "with (a);\nif (1);\nelse if (2);\nelse;\nfor (;;);\nwhile (1);\nfor (var i in o);\nfor (var i of o);\ndo;\nwhile (1);");
}
#[test]
fn test_no_newline_js_format_1_9cc36922() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["babel", "flow", "typescript"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("if (a) {\n  b;\n\n\n  ;\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "if (a) {\n  b;\n}");
}
